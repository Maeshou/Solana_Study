use syn::{visit::Visit, File, ItemFn, Expr, Stmt};
use petgraph::graph::{Graph, NodeIndex};
use serde::Serialize;
use std::collections::HashMap;
use quote::ToTokens;

#[derive(Serialize)]
struct CFGNode {
    id: usize,
    label: String,
}

#[derive(Serialize)]
struct CFGEdge {
    from: usize,
    to: usize,
    label: String,
}

#[derive(Serialize)]
struct FunctionCFG {
    name: String,
    nodes: Vec<CFGNode>,
    edges: Vec<CFGEdge>,
}

struct CFGBuilder {
    graph: Graph<String, ()>,
    node_indices: HashMap<usize, NodeIndex>,
    next_node_id: usize,
    functions: Vec<FunctionCFG>,
}

impl CFGBuilder {
    fn new() -> Self {
        CFGBuilder {
            graph: Graph::new(),
            node_indices: HashMap::new(),
            next_node_id: 0,
            functions: Vec::new(),
        }
    }

    fn new_node(&mut self, label: String) -> usize {
        let node_id = self.next_node_id;
        self.next_node_id += 1;
        let idx = self.graph.add_node(label.clone());
        self.node_indices.insert(node_id, idx);
        node_id
    }

    fn add_edge(&mut self, from: usize, to: usize, label: &str, cfg: &mut FunctionCFG) {
        let fi = self.node_indices[&from];
        let ti = self.node_indices[&to];
        self.graph.add_edge(fi, ti, ());
        cfg.edges.push(CFGEdge {
            from,
            to,
            label: label.to_string(),
        });
    }

    fn process_stmt(
        &mut self,
        stmt: &Stmt,
        cfg: &mut FunctionCFG,
        prev: usize,
    ) -> usize {
        // ① while ループの検出
        if let Stmt::Expr(Expr::While(expr_while), _) = stmt {
            // Loop Start
            let start = self.new_node("Loop Start".into());
            cfg.nodes.push(CFGNode { id: start, label: "Loop Start".into() });
            self.add_edge(prev, start, "next", cfg);

            // 本文の再帰処理
            let mut last = start;
            for s in &expr_while.body.stmts {
                last = self.process_stmt(s, cfg, last);
            }

            // Loop End
            let end = self.new_node("Loop End".into());
            cfg.nodes.push(CFGNode { id: end, label: "Loop End".into() });
            // ループ出口エッジに "while" ラベル
            self.add_edge(start, end, "while", cfg);
            return end;
        }

        // ② for ループの検出
        if let Stmt::Expr(Expr::ForLoop(expr_for), _) = stmt {
            let start = self.new_node("Loop Start".into());
            cfg.nodes.push(CFGNode { id: start, label: "Loop Start".into() });
            self.add_edge(prev, start, "next", cfg);

            let mut last = start;
            for s in &expr_for.body.stmts {
                last = self.process_stmt(s, cfg, last);
            }

            let end = self.new_node("Loop End".into());
            cfg.nodes.push(CFGNode { id: end, label: "Loop End".into() });
            self.add_edge(start, end, "for", cfg);
            return end;
        }

        // ③ if 文の検出
        if let Stmt::Expr(Expr::If(expr_if), _) = stmt {
            let if_id = self.new_node("if statement".into());
            cfg.nodes.push(CFGNode { id: if_id, label: "if statement".into() });
            self.add_edge(prev, if_id, "next", cfg);

            let pred = self.new_node("predicate".into());
            cfg.nodes.push(CFGNode { id: pred, label: "predicate".into() });
            self.add_edge(if_id, pred, "predicate", cfg);

            let cond_str = expr_if.cond.to_token_stream().to_string();
            let cond = self.new_node(cond_str.clone());
            cfg.nodes.push(CFGNode { id: cond, label: cond_str });
            self.add_edge(pred, cond, "next", cfg);

            let then_id = self.new_node("True body".into());
            cfg.nodes.push(CFGNode { id: then_id, label: "True body".into() });
            self.add_edge(if_id, then_id, "true", cfg);
            let mut then_last = then_id;
            for s in &expr_if.then_branch.stmts {
                then_last = self.process_stmt(s, cfg, then_last);
            }

            let else_id = self.new_node("False body".into());
            cfg.nodes.push(CFGNode { id: else_id, label: "False body".into() });
            self.add_edge(if_id, else_id, "false", cfg);
            let mut else_last = else_id;
            if let Some((_, else_expr)) = &expr_if.else_branch {
                match &**else_expr {
                    Expr::Block(b) => {
                        for s in &b.block.stmts {
                            else_last = self.process_stmt(s, cfg, else_last);
                        }
                    }
                    Expr::If(_) => {
                        let nested = Stmt::Expr((**else_expr).clone(), None);
                        else_last = self.process_stmt(&nested, cfg, else_last);
                    }
                    _ => {
                        let txt = else_expr.to_token_stream().to_string();
                        let nid = self.new_node(txt.clone());
                        cfg.nodes.push(CFGNode { id: nid, label: txt });
                        self.add_edge(else_last, nid, "next", cfg);
                        else_last = nid;
                    }
                }
            } else {
                let no_op = self.new_node("No-op".into());
                cfg.nodes.push(CFGNode { id: no_op, label: "No-op".into() });
                self.add_edge(else_last, no_op, "next", cfg);
                else_last = no_op;
            }

            let merge = self.new_node("merge".into());
            cfg.nodes.push(CFGNode { id: merge, label: "merge".into() });
            self.add_edge(then_last, merge, "next", cfg);
            self.add_edge(else_last, merge, "next", cfg);
            return merge;
        }

        // ④ 関数呼び出しのチェック (Ok/Some は除外)
        if let Stmt::Expr(expr, _) = stmt {
            if let Expr::Call(call) = expr {
                match &*call.func {
                    Expr::Path(p) if p.path.is_ident("Ok") || p.path.is_ident("Some") => {}
                    _ => {
                        let txt = call.to_token_stream().to_string();
                        let nid = self.new_node(txt.clone());
                        cfg.nodes.push(CFGNode { id: nid, label: txt });
                        self.add_edge(prev, nid, "call", cfg);
                        return nid;
                    }
                }
            }
            let txt = stmt.to_token_stream().to_string();
            let nid = self.new_node(txt.clone());
            cfg.nodes.push(CFGNode { id: nid, label: txt });
            self.add_edge(prev, nid, "next", cfg);
            return nid;
        }

        // ⑤ その他の文
        let txt = stmt.to_token_stream().to_string();
        let nid = self.new_node(txt.clone());
        cfg.nodes.push(CFGNode { id: nid, label: txt });
        self.add_edge(prev, nid, "next", cfg);
        nid
    }
}

impl<'ast> Visit<'ast> for CFGBuilder {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let name = i.sig.ident.to_string();
        let mut cfg = FunctionCFG { name, nodes: vec![], edges: vec![] };

        let entry = self.new_node("Entry".into());
        cfg.nodes.push(CFGNode { id: entry, label: "Entry".into() });

        let mut last = entry;
        for stmt in &i.block.stmts {
            last = self.process_stmt(stmt, &mut cfg, last);
        }

        self.functions.push(cfg);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: generate_cfg <input.rs> <output.json>");
        std::process::exit(1);
    }
    let src = std::fs::read_to_string(&args[1]).unwrap();
    let syntax: File = syn::parse_file(&src).unwrap();

    let mut builder = CFGBuilder::new();
    builder.visit_file(&syntax);

    let json = serde_json::to_string_pretty(&builder.functions).unwrap();
    std::fs::write(&args[2], json).unwrap();
    println!("CFG written to {}", &args[2]);
}

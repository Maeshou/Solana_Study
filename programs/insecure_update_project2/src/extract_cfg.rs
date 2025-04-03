use syn::{visit::Visit, File, ItemFn, Expr, Stmt};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
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
    current_func: Option<String>,
    next_node_id: usize,
    functions: Vec<FunctionCFG>,
}

impl CFGBuilder {
    fn new() -> Self {
        CFGBuilder {
            graph: Graph::new(),
            node_indices: HashMap::new(),
            current_func: None,
            next_node_id: 0,
            functions: Vec::new(),
        }
    }

    fn new_node(&mut self, label: String) -> usize {
        let node_id = self.next_node_id;
        self.next_node_id += 1;
        let node_index = self.graph.add_node(label.clone());
        self.node_indices.insert(node_id, node_index);
        node_id
    }

    // エッジ作成時、ラベルも渡して CFGEdge を生成
    fn add_edge(&mut self, from_id: usize, to_id: usize, label: &str, func_cfg: &mut FunctionCFG) {
        let from_index = self.node_indices[&from_id];
        let to_index = self.node_indices[&to_id];
        self.graph.add_edge(from_index, to_index, ());
        func_cfg.edges.push(CFGEdge {
            from: from_id,
            to: to_id,
            label: label.to_string(),
        });
    }

    // 各文(stmt)を処理してCFGノードとエッジを生成するヘルパー関数
    fn process_stmt(&mut self, stmt: &Stmt, func_cfg: &mut FunctionCFG, prev_node_id: usize) -> usize {
        // まず、if文と関数呼び出しを優先的にパターンマッチ
        if let Stmt::Expr(Expr::If(expr_if), _) = stmt {
            // if の条件部分のノードを作成
            let cond_str = expr_if.cond.to_token_stream().to_string();
            let if_node_id = self.new_node(format!("if ({})", cond_str));
            func_cfg.nodes.push(CFGNode { id: if_node_id, label: format!("if ({})", cond_str) });
            self.add_edge(prev_node_id, if_node_id, "next", func_cfg);

            // true ブランチのエントリとして一旦 "then" ノードを作成
            let then_entry = self.new_node("then".to_string());
            func_cfg.nodes.push(CFGNode { id: then_entry, label: "then".to_string() });
            self.add_edge(if_node_id, then_entry, "true", func_cfg);
            let mut then_prev = then_entry;
            for then_stmt in &expr_if.then_branch.stmts {
                then_prev = self.process_stmt(then_stmt, func_cfg, then_prev);
            }

            // false ブランチのエントリとして一旦 "else" ノードを作成
            let else_entry = self.new_node("else".to_string());
            func_cfg.nodes.push(CFGNode { id: else_entry, label: "else".to_string() });
            self.add_edge(if_node_id, else_entry, "false", func_cfg);
            let mut else_prev = else_entry;
            if let Some((_, else_expr)) = &expr_if.else_branch {
                match &**else_expr {
                    // else ブロックがブロック式の場合
                    Expr::Block(else_block) => {
                        for else_stmt in &else_block.block.stmts {
                            else_prev = self.process_stmt(else_stmt, func_cfg, else_prev);
                        }
                    },
                    // else if の場合も再帰的に処理
                    Expr::If(_) => {
                        let else_stmt = Stmt::Expr((**else_expr).clone(), None);
                        else_prev = self.process_stmt(&else_stmt, func_cfg, else_prev);
                    },
                    // その他は式として扱う
                    _ => {
                        let else_expr_str = else_expr.to_token_stream().to_string();
                        let node_id = self.new_node(else_expr_str.clone());
                        func_cfg.nodes.push(CFGNode { id: node_id, label: else_expr_str });
                        self.add_edge(else_prev, node_id, "next", func_cfg);
                        else_prev = node_id;
                    }
                }
            } else {
                // else ブロックがない場合は No-op ノードを追加
                let no_op_id = self.new_node("No-op".to_string());
                func_cfg.nodes.push(CFGNode { id: no_op_id, label: "No-op".to_string() });
                self.add_edge(else_prev, no_op_id, "next", func_cfg);
                else_prev = no_op_id;
            }

            // merge ノードを作成し、各分岐から merge へ "next" で接続
            let merge_node_id = self.new_node("merge".to_string());
            func_cfg.nodes.push(CFGNode { id: merge_node_id, label: "merge".to_string() });
            self.add_edge(then_prev, merge_node_id, "next", func_cfg);
            self.add_edge(else_prev, merge_node_id, "next", func_cfg);
            merge_node_id

        } else if let Stmt::Expr(expr, _) = stmt {
            // 関数呼び出しのチェック
            if let Expr::Call(expr_call) = expr {
                let call_str = expr_call.to_token_stream().to_string();
                let node_id = self.new_node(call_str.clone());
                func_cfg.nodes.push(CFGNode { id: node_id, label: call_str.clone() });
                // 「call」エッジを追加
                self.add_edge(prev_node_id, node_id, "call", func_cfg);
                // ※ここで、もし関数呼び出し先の CFG エントリが判明していれば、別途「call」エッジを追加可能
                return node_id;
            }
            // 通常の式の場合
            let stmt_str = stmt.to_token_stream().to_string();
            let node_id = self.new_node(stmt_str.clone());
            func_cfg.nodes.push(CFGNode { id: node_id, label: stmt_str });
            self.add_edge(prev_node_id, node_id, "next", func_cfg);
            node_id
        } else {
            // その他の文（Stmt::Semi など）はそのまま処理
            let stmt_str = stmt.to_token_stream().to_string();
            let node_id = self.new_node(stmt_str.clone());
            func_cfg.nodes.push(CFGNode { id: node_id, label: stmt_str });
            self.add_edge(prev_node_id, node_id, "next", func_cfg);
            node_id
        }
    }
}

impl<'ast> Visit<'ast> for CFGBuilder {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_string();
        self.current_func = Some(func_name.clone());

        let mut func_cfg = FunctionCFG {
            name: func_name.clone(),
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        // エントリノードの作成
        let entry_id = self.new_node("Entry".to_string());
        func_cfg.nodes.push(CFGNode { id: entry_id, label: "Entry".to_string() });
        let mut prev_node_id = entry_id;

        // 関数本体の各文を process_stmt で順次処理
        for stmt in &i.block.stmts {
            prev_node_id = self.process_stmt(stmt, &mut func_cfg, prev_node_id);
        }

        self.functions.push(func_cfg);
        self.current_func = None;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: generate_cfg <path_to_rust_file> <path_to_output_json>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_path = &args[2];

    let file_content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File = syn::parse_file(&file_content).expect("Failed to parse file");

    let mut cfg_builder = CFGBuilder::new();
    cfg_builder.visit_file(&syntax_tree);

    // CFG データを JSON 形式で出力
    let json = serde_json::to_string_pretty(&cfg_builder.functions).expect("Failed to serialize CFG");
    std::fs::write(output_path, json).expect("Failed to write CFG to file");

    println!("CFG has been generated and saved to {}", output_path);
}

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

    fn add_edge(&mut self, from_id: usize, to_id: usize) {
        let from_index = self.node_indices[&from_id];
        let to_index = self.node_indices[&to_id];
        self.graph.add_edge(from_index, to_index, ());
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

        // エントリノードを作成
        let entry_id = self.new_node("Entry".to_string());
        func_cfg.nodes.push(CFGNode { id: entry_id, label: "Entry".to_string() });

        let mut prev_node_id = entry_id;

        // 関数のボディを順番に処理
        for stmt in &i.block.stmts {
            let stmt_str = stmt.to_token_stream().to_string();
            let node_id = self.new_node(stmt_str.clone());
            func_cfg.nodes.push(CFGNode { id: node_id, label: stmt_str });

            // 前のノードから現在のノードへエッジを追加
            self.add_edge(prev_node_id, node_id);
            func_cfg.edges.push(CFGEdge { from: prev_node_id, to: node_id });

            prev_node_id = node_id;
        }

        // ここでは簡略化のため、関数の終了ノードを追加しませんが、必要に応じて追加できます。

        self.functions.push(func_cfg);

        // ノードとエッジの情報は `self.graph` にも格納されていますが、
        // JSON出力のために `functions` ベクタにも格納しています。

        // 関数の解析が終了したら、状態をリセット
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

    // CFGデータをJSON形式で出力
    let json = serde_json::to_string_pretty(&cfg_builder.functions).expect("Failed to serialize CFG");
    std::fs::write(output_path, json).expect("Failed to write CFG to file");

    println!("CFG has been generated and saved to {}", output_path);
}


use std::env;
use std::fs::File;
use std::io::Write;
use serde_json::json;
use syn::{visit::Visit, File as SynFile, ItemFn};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    label: String,
    attributes: String,
}

#[derive(Debug, Clone)]
struct Edge {
    source: usize,
    target: usize,
    label: String,
}

struct ASTGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    next_id: usize,
}

impl ASTGraph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            next_id: 1,
        }
    }

    fn add_node(&mut self, label: &str, attributes: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.push(Node {
            id,
            label: label.to_string(),
            attributes: attributes.to_string(),
        });
        id
    }

    fn add_edge(&mut self, source: usize, target: usize, label: &str) {
        self.edges.push(Edge {
            source,
            target,
            label: label.to_string(),
        });
    }

    fn to_json(&self) -> serde_json::Value {
        json!({
            "nodes": self.nodes.iter().map(|node| json!({
                "id": node.id,
                "label": node.label,
                "attributes": node.attributes,
            })).collect::<Vec<_>>(),
            "edges": self.edges.iter().map(|edge| json!({
                "source": edge.source,
                "target": edge.target,
                "label": edge.label,
            })).collect::<Vec<_>>(),
        })
    }
}

/// ヘルパー: トークン文字列から属性を割り当てる
fn get_token_attribute(token: &str) -> &str {
    match token {
        "mod" => "module",
        "fn" => "function",
        "let" => "define",
        "=" => "operator",
        "&" => "punctuation",
        "mut" => "mut",
        "Context" => "structure",
        "InitializeVault" => "structure",
        "Pubkey" => "structure",
        "inputs" => "inputs",
        "authority" => "structure",
        "expression" => "expression",
        "ctx" => "field base",
        "accounts" => "field member",
        "vault" => "variable",
        _ => "unknown",
    }
}

/// TokenStream 内の各トークンを親ノードの子として追加する関数
fn add_tokens(graph: &mut ASTGraph, token_stream: TokenStream, parent: usize, edge_label: &str) {
    let mut prev_id: Option<usize> = None;
    for token in token_stream {
        // TokenTree を文字列に変換
        let token_str = match &token {
            TokenTree::Group(g) => g.to_token_stream().to_string(),
            TokenTree::Ident(ident) => ident.to_string(),
            TokenTree::Punct(p) => p.as_char().to_string(),
            TokenTree::Literal(lit) => lit.to_string(),
        };
        let attr = get_token_attribute(&token_str);
        let current_id = graph.add_node(&token_str, attr);
        if let Some(prev) = prev_id {
            graph.add_edge(prev, current_id, "next");
        } else {
            // 最初のトークンは親ノードと edge_label で接続
            graph.add_edge(parent, current_id, edge_label);
        }
        prev_id = Some(current_id);
    }
}

/// AST を走査してトークンごとのノードを生成する Visitor
struct ASTVisitor {
    graph: ASTGraph,
    current_parent: Option<usize>,
}

impl ASTVisitor {
    fn new() -> Self {
        Self {
            graph: ASTGraph::new(),
            current_parent: None,
        }
    }
}

impl<'ast> Visit<'ast> for ASTVisitor {
    fn visit_file(&mut self, node: &'ast SynFile) {
        // ルートとなるモジュールノードを生成
        let mod_id = self.graph.add_node("mod", "module");
        self.current_parent = Some(mod_id);
        syn::visit::visit_file(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // 関数ノードを生成
        let func_id = self.graph.add_node("function", "function");
        if let Some(parent) = self.current_parent {
            self.graph.add_edge(parent, func_id, "contains");
        }

        // 関数シグネチャの入力部分 (inputs) ノードを生成
        let inputs_id = self.graph.add_node("inputs", "inputs");
        self.graph.add_edge(func_id, inputs_id, "has");

        // 各引数について、TokenStream を走査しノードを生成
        for input in &node.sig.inputs {
            let ts = input.to_token_stream();
            add_tokens(&mut self.graph, ts, inputs_id, "parameter");
        }

        // 関数本体の式を処理するための expression ノードを生成
        let expr_id = self.graph.add_node("expression", "expression");
        self.graph.add_edge(func_id, expr_id, "has");

        // 関数本体内の各ステートメントを処理
        for stmt in &node.block.stmts {
            let ts = stmt.to_token_stream();
            // ステートメント内に "=" が含まれているか（簡易チェック）
            if ts.to_string().contains("=") {
                // トークンを分割して "=" を境に左右を分離する
                let tokens: Vec<TokenTree> = ts.into_iter().collect();
                let mut lhs_tokens = vec![];
                let mut rhs_tokens = vec![];
                let mut found_eq = false;
                // シンプルにトークン列を前半と後半に分ける
                for token in tokens {
                    let token_text = match &token {
                        TokenTree::Group(g) => g.to_token_stream().to_string(),
                        TokenTree::Ident(ident) => ident.to_string(),
                        TokenTree::Punct(p) => p.as_char().to_string(),
                        TokenTree::Literal(lit) => lit.to_string(),
                    };
                    if token_text == "=" {
                        found_eq = true;
                        // "=" ノードを生成して expression ノードと接続
                        let eq_id = self.graph.add_node("=", "operator");
                        self.graph.add_edge(expr_id, eq_id, "contains");
                        // lhs 側のトークンを追加（clone() を利用して所有権移動を回避）
                        if !lhs_tokens.is_empty() {
                            let lhs_stream: TokenStream = lhs_tokens.clone().into_iter().collect();
                            add_tokens(&mut self.graph, lhs_stream, eq_id, "lhs");
                        }
                        continue;
                    }
                    if !found_eq {
                        lhs_tokens.push(token);
                    } else {
                        rhs_tokens.push(token);
                    }
                }
                if found_eq && !rhs_tokens.is_empty() {
                    // 直前に追加した "=" ノードの親として、rhs 側のトークンを追加
                    let rhs_stream: TokenStream = rhs_tokens.into_iter().collect();
                    // self.graph.next_id を直接使わず、一旦ローカル変数に退避
                    let parent_id = self.graph.next_id.saturating_sub(1);
                    add_tokens(&mut self.graph, rhs_stream, parent_id, "rhs");
                }
            } else {
                // "=" を含まないステートメントはそのまま expression ノードの下に追加
                add_tokens(&mut self.graph, stmt.to_token_stream(), expr_id, "contains");
            }
        }

        // 再帰的に中身を処理
        syn::visit::visit_item_fn(self, node);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: extract_ast <input_file> <output_file>");
        return;
    }
    let input_file = &args[1];
    let output_file = &args[2];

    // ソースコードの読み込みと解析
    let code = std::fs::read_to_string(input_file)
        .expect("Failed to read the input file");
    let syntax_tree: SynFile = syn::parse_file(&code)
        .expect("Failed to parse file");

    let mut visitor = ASTVisitor::new();
    visitor.visit_file(&syntax_tree);

    // AST グラフを JSON として保存
    let graph_json = visitor.graph.to_json();
    let mut file = File::create(output_file)
        .expect("Failed to create output file");
    file.write_all(graph_json.to_string().as_bytes())
        .expect("Failed to write to output file");

    println!("AST graph saved to {}", output_file);
}

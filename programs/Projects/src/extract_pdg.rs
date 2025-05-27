use syn::{visit::Visit, File, ItemFn, Expr, Stmt, Pat};
use serde::Serialize;
use std::collections::HashMap;
use quote::ToTokens;

#[derive(Serialize)]
struct PDGNode {
    id: usize,
    label: String,
}

#[derive(Serialize)]
struct PDGEdge {
    from: usize,
    to: usize,
    label: String,
}

#[derive(Serialize)]
struct FunctionPDG {
    name: String,
    inputs: Vec<String>,  // パラメータ名と型を含む
    nodes: Vec<PDGNode>,
    edges: Vec<PDGEdge>,
}

struct PDGBuilder {
    defs: HashMap<String, usize>, // 変数名とノードIDの対応
    next_node_id: usize,
    functions: Vec<FunctionPDG>,
    current_func_pdg: Option<FunctionPDG>, // 現在の関数のPDG
}

impl PDGBuilder {
    fn new() -> Self {
        PDGBuilder {
            defs: HashMap::new(),
            next_node_id: 0,
            functions: Vec::new(),
            current_func_pdg: None,
        }
    }

    fn new_node(&mut self, label: String) -> usize {
        let node_id = self.next_node_id;
        self.next_node_id += 1;
        node_id
    }

    fn add_edge(&mut self, from_id: usize, to_id: usize, label: String) {
        if let Some(func_pdg) = self.current_func_pdg.as_mut() {
            func_pdg.edges.push(PDGEdge { from: from_id, to: to_id, label });
        }
    }
}

impl<'ast> Visit<'ast> for PDGBuilder {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_token_stream().to_string();

        // 関数の引数を抽出（名前と型を取得）
        let inputs = i.sig.inputs.iter().map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    let param_name = pat_type.pat.to_token_stream().to_string();
                    let param_type = pat_type.ty.to_token_stream().to_string();
                    format!("{}: {}", param_name, param_type)
                },
                syn::FnArg::Receiver(receiver) => {
                    // `self` 引数の場合
                    let mutability = if receiver.mutability.is_some() { "mut " } else { "" };
                    let reference = if receiver.reference.is_some() { "&" } else { "" };
                    format!("{}{}self", reference, mutability)
                }
            }
        }).collect::<Vec<String>>();

        // 現在の関数のPDGを初期化
        self.current_func_pdg = Some(FunctionPDG {
            name: func_name,
            inputs,  // ここで入力引数を設定
            nodes: Vec::new(),
            edges: Vec::new(),
        });

        // 関数の開始ノードを追加
        let entry_id = self.new_node("Entry".to_string());
        if let Some(func_pdg) = self.current_func_pdg.as_mut() {
            func_pdg.nodes.push(PDGNode { id: entry_id, label: "Entry".to_string() });
        }

        // 関数のボディを処理
        for stmt in &i.block.stmts {
            self.visit_stmt(stmt);
        }

        // 現在の関数のPDGを保存
        if let Some(func_pdg) = self.current_func_pdg.take() {
            self.functions.push(func_pdg);
        }

        // 状態のリセット
        self.defs.clear();
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        let stmt_str = stmt.to_token_stream().to_string();
        let stmt_id = self.new_node(stmt_str.clone());

        // ノードを現在の関数のPDGに追加
        if let Some(func_pdg) = self.current_func_pdg.as_mut() {
            func_pdg.nodes.push(PDGNode { id: stmt_id, label: stmt_str });
        }

        // ステートメント内の変数を解析
        match stmt {
            Stmt::Local(local) => {
                // 変数の定義を処理
                if let Pat::Ident(pat_ident) = &local.pat {
                    let var_name = pat_ident.ident.to_string();
                    self.defs.insert(var_name.clone(), stmt_id);

                    // 変数定義のエッジを追加
                    if let Some(func_pdg) = self.current_func_pdg.as_mut() {
                        func_pdg.edges.push(PDGEdge { from: stmt_id, to: stmt_id, label: format!("def: {}", var_name) });
                    }
                }

                // initializer（=以降の式）が存在する場合、その式内の変数使用を処理
                if let Some(init) = &local.init {
                    // init は LocalInit 構造体であり、フィールド expr に初期化式が入っています
                    let used_vars = extract_variables(&init.expr);
                    for var_name in used_vars {
                        if let Some(&def_id) = self.defs.get(&var_name) {
                            // データ依存エッジを追加
                            self.add_edge(def_id, stmt_id, format!("data_dep: {}", var_name));
                        }
                    }
                }
            },
            Stmt::Expr(expr, _) => {
                // 式内の変数使用を処理
                let used_vars = extract_variables(expr);
                for var_name in used_vars {
                    if let Some(&def_id) = self.defs.get(&var_name) {
                        // データ依存エッジを追加
                        self.add_edge(def_id, stmt_id, format!("data_dep: {}", var_name));
                    }
                }
            },
            _ => {}
        }

        // 子ノードを再帰的に訪問
        syn::visit::visit_stmt(self, stmt);
    }
}

// 式内の変数名を抽出するヘルパー関数
fn extract_variables(expr: &Expr) -> Vec<String> {
    let mut vars = Vec::new();

    struct VarVisitor<'a> {
        vars: &'a mut Vec<String>,
    }

    impl<'ast, 'a> Visit<'ast> for VarVisitor<'a> {
        fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
            if let Some(ident) = expr_path.path.get_ident() {
                self.vars.push(ident.to_string());
            }
            syn::visit::visit_expr_path(self, expr_path);
        }
    }

    let mut visitor = VarVisitor { vars: &mut vars };
    visitor.visit_expr(expr);

    vars
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: generate_pdg <path_to_rust_file> <path_to_output_json>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_path = &args[2];

    let file_content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File = syn::parse_file(&file_content).expect("Failed to parse file");

    let mut pdg_builder = PDGBuilder::new();
    pdg_builder.visit_file(&syntax_tree);

    // PDGデータをJSON形式で出力
    let json = serde_json::to_string_pretty(&pdg_builder.functions).expect("Failed to serialize PDG");
    std::fs::write(output_path, json).expect("Failed to write PDG to file");

    println!("PDG has been generated and saved to {}", output_path);
}

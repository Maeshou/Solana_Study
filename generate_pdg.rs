use syn::{visit::Visit, Expr, Stmt, ItemFn, Ident};
use syn::__private::ToTokens;
use std::collections::{HashMap, HashSet};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

struct PDGBuilder<'ast> {
    graph: Graph<String, String>,
    current_func: Option<NodeIndex>,
    defs: HashMap<String, NodeIndex>,
}

impl<'ast> Visit<'ast> for PDGBuilder<'ast> {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_string();
        let func_node = self.graph.add_node(format!("Function: {}", func_name));
        self.current_func = Some(func_node);

        // 関数のボディを訪問
        for stmt in &i.block.stmts {
            self.visit_stmt(stmt);
        }

        self.current_func = None;
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        // ステートメントノードを追加
        let stmt_str = stmt.to_token_stream().to_string();
        let stmt_node = self.graph.add_node(format!("Stmt: {}", stmt_str));

        // 関数ノードからステートメントノードへエッジを追加
        if let Some(func_node) = self.current_func {
            self.graph.add_edge(func_node, stmt_node, "contains".to_string());
        }

        // 変数のDefとUseを解析
        self.visit_stmt_for_defs_uses(stmt, stmt_node);

        // ステートメント内の子ノードを訪問
        syn::visit::visit_stmt(self, stmt);
    }

    // 変数のDefとUseを解析するヘルパー関数
    fn visit_stmt_for_defs_uses(&mut self, stmt: &'ast Stmt, stmt_node: NodeIndex) {
        if let Stmt::Local(local) = stmt {
            if let Some((ident, _)) = &local.pat {
                if let syn::Pat::Ident(pat_ident) = ident {
                    let var_name = pat_ident.ident.to_string();
                    // Defノードを追加
                    self.defs.insert(var_name.clone(), stmt_node);
                }
            }
        } else if let Stmt::Expr(expr) = stmt {
            self.visit_expr_for_uses(expr, stmt_node);
        }
    }

    // 式内の変数Useを解析するヘルパー関数
    fn visit_expr_for_uses(&mut self, expr: &'ast Expr, stmt_node: NodeIndex) {
        if let Expr::Path(expr_path) = expr {
            if let Some(ident) = expr_path.path.get_ident() {
                let var_name = ident.to_string();
                if let Some(def_node) = self.defs.get(&var_name) {
                    // Defノードから現在のステートメントノードへエッジを追加
                    self.graph.add_edge(*def_node, stmt_node, format!("data: {}", var_name));
                }
            }
        }

        // 式内の子ノードを再帰的に訪問
        syn::visit::visit_expr(self, expr);
    }
}
fn main() {
    let file_content = std::fs::read_to_string("path/to/your/lib.rs").expect("Failed to read file");
    let syntax_tree = syn::parse_file(&file_content).expect("Failed to parse file");

    let mut pdg_builder = PDGBuilder {
        graph: Graph::new(),
        current_func: None,
        defs: HashMap::new(),
    };

    pdg_builder.visit_file(&syntax_tree);

    // グラフの可視化
    println!("{:?}", Dot::with_config(&pdg_builder.graph, &[Config::EdgeNoLabel]));
}

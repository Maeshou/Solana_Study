use syn::{visit::Visit, File, ItemFn, ItemStruct, Fields, Pat, Item, Attribute, Stmt, ItemMod};
use serde::Serialize;
use quote::ToTokens;
use std::fs;

#[derive(Serialize)]
struct FieldInfo {
    name: String,
    attribute: Option<String>,
    field_type: String,
}

#[derive(Serialize)]
struct ASTNode {
    name: String,
    node_type: String,
    fields: Option<Vec<FieldInfo>>,    // 構造体用
    inputs: Option<Vec<String>>,       // 関数用
    attributes: Option<Vec<String>>,   // 関数やモジュール、その他用
    body: Option<Vec<String>>,         // 関数本体(ステートメントの文字列)
    // 必要に応じて他のフィールドも追加可能
}

struct ASTBuilder {
    nodes: Vec<ASTNode>,
}

impl ASTBuilder {
    fn new() -> Self {
        ASTBuilder { nodes: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ASTBuilder {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let struct_name = node.ident.to_string();
        let fields = extract_struct_fields(&node.fields);

        let ast_node = ASTNode {
            name: struct_name,
            node_type: "struct".to_string(),
            fields: Some(fields),
            inputs: None,
            attributes: None,
            body: None,
        };

        self.nodes.push(ast_node);

        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_string();

        // 関数の引数を抽出 (ctx: Context<...>, authority: Pubkey など)
        let inputs = i.sig.inputs.iter().map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    let param_name = pat_type.pat.to_token_stream().to_string();
                    let param_type = pat_type.ty.to_token_stream().to_string();
                    format!("{}: {}", param_name, param_type)
                },
                syn::FnArg::Receiver(receiver) => {
                    let mutability = if receiver.mutability.is_some() { "mut " } else { "" };
                    let reference = if receiver.reference.is_some() { "&" } else { "" };
                    format!("{}{}self", reference, mutability)
                }
            }
        }).collect::<Vec<String>>();

        // 関数の属性を抽出
        let attributes = i.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect::<Vec<_>>();

        // 関数本体（Block）からステートメントを抽出
        let body_stmts = i.block.stmts.iter().map(|stmt| stmt.to_token_stream().to_string()).collect::<Vec<_>>();

        let ast_node = ASTNode {
            name: func_name,
            node_type: "function".to_string(),
            fields: None,
            inputs: Some(inputs),
            attributes: Some(attributes),
            body: Some(body_stmts),
        };

        self.nodes.push(ast_node);

        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        // モジュールなども必要ならここで処理可能
        syn::visit::visit_item_mod(self, i);
    }

    fn visit_file(&mut self, file: &'ast File) {
        // ファイル直下のItemを再帰的に訪問
        syn::visit::visit_file(self, file);
    }
}

fn extract_struct_fields(fields: &Fields) -> Vec<FieldInfo> {
    fields
        .iter()
        .map(|field| {
            // フィールド名を取得
            let name = field.ident.as_ref().map(|ident| ident.to_string()).unwrap_or("".to_string());

            // フィールドの型を文字列化
            let field_type = field.ty.to_token_stream().to_string();

            // 属性を文字列化して結合
            let attributes = field.attrs.iter()
                .map(|attr| attr.to_token_stream().to_string())
                .collect::<Vec<String>>()
                .join(" ");

            // 属性が存在しない場合は None
            let attribute = if attributes.is_empty() { None } else { Some(attributes) };

            FieldInfo {
                name,
                attribute,
                field_type,
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: extract_ast <path_to_rust_file> <path_to_output_json>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_path = &args[2];

    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File = syn::parse_file(&file_content).expect("Failed to parse file");

    let mut ast_builder = ASTBuilder::new();
    ast_builder.visit_file(&syntax_tree);

    // ASTデータをJSON形式で出力
    let json = serde_json::to_string_pretty(&ast_builder.nodes).expect("Failed to serialize AST");
    fs::write(output_path, json).expect("Failed to write AST to file");

    println!("AST has been generated and saved to {}", output_path);
}


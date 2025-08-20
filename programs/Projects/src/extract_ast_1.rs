use syn::{visit::Visit, File, ItemFn, ItemStruct, Fields, Stmt, Expr, ItemMod};
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
    fields: Option<Vec<FieldInfo>>,    // struct 用
    inputs: Option<Vec<String>>,       // fn 用
    attributes: Option<Vec<String>>,   // fn・mod 用
    body: Option<Vec<String>>,         // fn 本文
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
        self.nodes.push(ASTNode {
            name: struct_name,
            node_type: "struct".into(),
            fields: Some(fields),
            inputs: None,
            attributes: None,
            body: None,
        });
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let func_name = i.sig.ident.to_string();

        // 引数の抽出
        let inputs = i.sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Typed(pt) => {
                let n = pt.pat.to_token_stream().to_string();
                let t = pt.ty.to_token_stream().to_string();
                format!("{}: {}", n, t)
            }
            syn::FnArg::Receiver(rc) => {
                let m = if rc.mutability.is_some() { "mut " } else { "" };
                let r = if rc.reference.is_some()  { "&" } else { "" };
                format!("{}{}self", r, m)
            }
        }).collect::<Vec<_>>();

        // 属性の抽出
        let attributes = i.attrs.iter()
            .map(|a| a.to_token_stream().to_string())
            .collect::<Vec<_>>();

        // 本文ステートメントの収集
        let mut body_stmts = Vec::new();
        for stmt in &i.block.stmts {
            let text = stmt.to_token_stream().to_string();

            // コントロール構文かチェック
            let is_ctrl = match stmt {
                Stmt::Expr(expr, _) => matches!(expr, Expr::If(_) | Expr::ForLoop(_) | Expr::While(_)),
                _ => false,
            };

            if is_ctrl {
                // 1) 最初の '{' でヘッダーと本文に分割
                let parts: Vec<&str> = text.splitn(2, '{').collect();
                if parts.len() == 2 {
                    // ヘッダー (例: "if cond ")
                    body_stmts.push(parts[0].trim().to_string());
                    // 「{」を独立
                    body_stmts.push("{".to_string());

                    // 本文部：末尾の '}' を除き
                    let mut inner = parts[1];
                    if let Some(pos) = inner.rfind('}') {
                        inner = &inner[..pos];
                    }
                    // 2) セミコロンを含めて分割
                    for segment in inner.split_inclusive(';') {
                        let seg = segment.trim();
                        if seg.is_empty() { continue; }

                        // ネストしたコントロール構文があれば再分割
                        if let Some(brace_pos) = seg.find('{') {
                            body_stmts.push(seg[..brace_pos].trim().to_string());
                            body_stmts.push("{".to_string());
                            let rest = seg[brace_pos+1..].trim();
                            if !rest.is_empty() {
                                body_stmts.push(rest.to_string());
                            }
                        } else {
                            body_stmts.push(seg.to_string());
                        }
                    }
                    // 3) 閉じ括弧を独立要素として追加
                    body_stmts.push("}".to_string());
                    continue;
                }
            }

            // 分割不要な文はそのまま
            body_stmts.push(text);
        }

        self.nodes.push(ASTNode {
            name: func_name,
            node_type: "function".into(),
            fields: None,
            inputs: Some(inputs),
            attributes: Some(attributes),
            body: Some(body_stmts),
        });
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        syn::visit::visit_item_mod(self, i);
    }

    fn visit_file(&mut self, file: &'ast File) {
        syn::visit::visit_file(self, file);
    }
}

fn extract_struct_fields(fields: &Fields) -> Vec<FieldInfo> {
    fields.iter().map(|f| {
        let name = f.ident.as_ref().map(|id| id.to_string()).unwrap_or_default();
        let ty   = f.ty.to_token_stream().to_string();
        let attrs = f.attrs.iter()
            .map(|a| a.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        FieldInfo {
            name,
            attribute: if attrs.is_empty() { None } else { Some(attrs) },
            field_type: ty,
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: extract_ast <input.rs> <output.json>");
        std::process::exit(1);
    }
    let input = &args[1];
    let output = &args[2];

    let src = fs::read_to_string(input).expect("Failed to read source file");
    let syntax: File = syn::parse_file(&src).expect("Failed to parse Rust file");

    let mut builder = ASTBuilder::new();
    builder.visit_file(&syntax);

    let json = serde_json::to_string_pretty(&builder.nodes)
        .expect("Failed to serialize AST");
    fs::write(output, json).expect("Failed to write JSON file");

    println!("AST generated and saved to {}", output);
}

#!/usr/bin/env python3
import json
import re
import sys

def tokenize_statement(stmt):
    """ ステートメントをトークン化 """
    token_pattern = re.compile(
        # まず &mut、一続きの二文字演算子、整数リテラル、
        # 識別子、単一文字の記号、その他の１文字
        r'&mut'
      + r'|==|!=|<=|>=|&&|\|\|'
      + r'|\d+'
      + r'|[A-Za-z_][A-Za-z0-9_]*'
      + r'|[<>{}()\[\].,;:=+\-*/&|!~^%]'
      + r'|\S'
    )
    tokens = token_pattern.findall(stmt)

    # "&" と "mut" が離れたまま取れてしまった場合のマージ（念のため）
    merged_tokens = []
    i = 0
    while i < len(tokens):
        if tokens[i] == '&' and i + 1 < len(tokens) and tokens[i+1] == 'mut':
            merged_tokens.append('&mut')
            i += 2
        else:
            merged_tokens.append(tokens[i])
            i += 1

    return merged_tokens

TOKEN_ATTR_MAP = {
    'let': 'define',
    '=': 'operator',
    '==': 'operator',
    '!=': 'operator',
    '<=': 'operator',
    '>=': 'operator',
    '&&': 'operator',
    '||': 'operator',
    '&mut': 'mut',
    'fn': 'function',
    'mod': 'module',
    '.': 'member',
    '(': 'delimiter',
    ')': 'delimiter',
    '<': 'delimiter',
    '>': 'delimiter',
    '{': 'delimiter',
    '}': 'delimiter',
    '[': 'delimiter',
    ']': 'delimiter',
    ',': 'delimiter',
    ';': 'delimiter'
}

def get_token_attribute(token):
    """トークンの属性を取得"""
    return TOKEN_ATTR_MAP.get(token, "identifier")

def main():
    if len(sys.argv) < 3:
        print("Usage: python3 tokenize_ast.py <input_json> <output_json>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    with open(input_file, 'r', encoding='utf-8') as f:
        ast_data = json.load(f)
    print("Loaded AST Data:", json.dumps(ast_data, indent=2, ensure_ascii=False))

    token_graph = {
        "nodes": [],
        "edges": []
    }
    next_id = 1

    def add_node(label, attribute):
        nonlocal next_id
        node = {
            "id": next_id,
            "label": label,
            "attributes": attribute
        }
        token_graph["nodes"].append(node)
        next_id += 1
        return node["id"]

    def add_edge(source, target, label):
        token_graph["edges"].append({
            "source": source,
            "target": target,
            "label": label
        })

    # モジュール (mod) ノードを作成
    mod_id = add_node("mod", "module")

    # 関数ノードの処理
    for node in ast_data:
        if node.get("node_type") == "function":
            func_id = add_node(node["name"], "function")
            add_edge(mod_id, func_id, "contains")

            # inputs ノード
            inputs_id = add_node("inputs", "inputs")
            add_edge(func_id, inputs_id, "has")
            for param in node.get("inputs", []):
                match = re.search(r"<\s*([A-Za-z0-9_]+)\s*>", param)
                if match:
                    struct_name = match.group(1)
                    struct_id = add_node(struct_name, "structure")
                else:
                    struct_name = param
                    struct_id = add_node(struct_name, "value")
                add_edge(inputs_id, struct_id, "parameter")

            # expression ノード
            expr_id = add_node("expression", "expression")
            add_edge(func_id, expr_id, "has")

            # 関数本体の各ステートメントをトークン化してグラフ化
            for stmt in node.get("body", []):
                tokens = tokenize_statement(stmt)
                if "=" in tokens and tokens.count("=") == 1:
                    eq_index = tokens.index("=")
                    eq_id = add_node("=", "operator")
                    # lhs
                    prev = None
                    for t in tokens[:eq_index]:
                        tid = add_node(t, get_token_attribute(t))
                        if prev is None:
                            add_edge(eq_id, tid, "lhs")
                        else:
                            add_edge(prev, tid, "next")
                        prev = tid
                    # rhs
                    prev = None
                    for t in tokens[eq_index+1:]:
                        tid = add_node(t, get_token_attribute(t))
                        if prev is None:
                            add_edge(eq_id, tid, "rhs")
                        else:
                            add_edge(prev, tid, "next")
                        prev = tid
                    add_edge(expr_id, eq_id, "contains")
                else:
                    prev = None
                    for t in tokens:
                        tid = add_node(t, get_token_attribute(t))
                        if prev is not None:
                            add_edge(prev, tid, "next")
                        prev = tid

    # 構造体ノードの処理
    for node in ast_data:
        if node.get("node_type") == "struct":
            struct_id = add_node(node["name"], "structure")
            add_edge(mod_id, struct_id, "contains")

            for field in node.get("fields", []):
                # フィールドタイプからメイン型を抽出
                ftype = field.get("field_type", "")
                main_type = re.split(r'<', ftype, maxsplit=1)[0].strip() or "field"
                field_id = add_node(field["name"], main_type)
                add_edge(struct_id, field_id, "has")

                # ジェネリック内の型引数を inner_type ノードに
                if '<' in ftype and '>' in ftype:
                    inner = ftype.split('<',1)[1].rsplit('>',1)[0]
                    for part in [p.strip() for p in inner.split(',')]:
                        iid = add_node(part, "field_inner")
                        add_edge(field_id, iid, "inner_type")

    # 結果をファイルに書き込み
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(token_graph, f, indent=2, ensure_ascii=False)
    print(f"Token graph saved to {output_file}")

if __name__ == "__main__":
    main()

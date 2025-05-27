import json
import re
import sys

def tokenize_statement(stmt):
    """ ステートメントをトークン化 """
    token_pattern = re.compile(r'[A-Za-z_][A-Za-z0-9_]*|[<>{}()\[\].,;=&\+\-\*/]+|\S')
    tokens = token_pattern.findall(stmt)
    # "&" と "mut" を "&mut" として統合
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
        print("Usage: python tokenize_ast.py <input_json> <output_json>")
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
        """ノードを追加"""
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
        """エッジを追加"""
        token_graph["edges"].append({
            "source": source,
            "target": target,
            "label": label
        })

    # `mod` ノードを作成
    mod_id = add_node("mod", "module")

    # 関数 (`function`) の処理
    for node in ast_data:
        if node.get("node_type") == "function":
            function_id = add_node(node["name"], "function")
            add_edge(mod_id, function_id, "contains")
            # `inputs` ノード
            inputs_id = add_node("inputs", "inputs")
            add_edge(function_id, inputs_id, "has")
            # 引数を `inputs` に追加
            for param in node.get("inputs", []):
                match = re.search(r"<\s*([A-Za-z0-9_]+)\s*>", param)
                if match:
                    struct_name = match.group(1)
                    struct_id = add_node(struct_name, "structure")
                else:
                    # コロン (:) を用いたパターンマッチング: 例 "a: u64" のようなパラメータの場合
                    param_match = re.search(r"([A-Za-z0-9_]+)\s*:\s*([A-Za-z0-9_]+)", param)
                    if param_match:
                        variable_name = param_match.group(1)
                        variable_type = param_match.group(2)
                        # 変数名をノード名として、変数型（ここでは大文字に変換）をノードの種類として登録
                        struct_name = variable_name
                        struct_id = add_node(variable_name, variable_type.upper())
                    else:
                        # コロンが含まれていない場合は既存の処理
                        struct_name = param
                        struct_id = add_node(struct_name, "value")

                add_edge(inputs_id, struct_id, "parameter")
            # `expression` ノード
            expr_id = add_node("expression", "expression")
            add_edge(function_id, expr_id, "has")
            # `body` の処理
            for stmt in node.get("body", []):
                tokens = tokenize_statement(stmt)
                if "=" in tokens:
                    eq_index = tokens.index("=")
                    eq_id = add_node("=", "operator")
                    # 左辺 (lhs)
                    lhs_tokens = tokens[:eq_index]
                    prev_lhs = None
                    for t in lhs_tokens:
                        token_id = add_node(t, get_token_attribute(t))
                        if prev_lhs is None:
                            add_edge(eq_id, token_id, "lhs")
                        else:
                            add_edge(prev_lhs, token_id, "next")
                        prev_lhs = token_id
                    # 右辺 (rhs)
                    rhs_tokens = tokens[eq_index+1:]
                    prev_rhs = None
                    for t in rhs_tokens:
                        token_id = add_node(t, get_token_attribute(t))
                        if prev_rhs is None:
                            add_edge(eq_id, token_id, "rhs")
                        else:
                            add_edge(prev_rhs, token_id, "next")
                        prev_rhs = token_id
                    add_edge(expr_id, eq_id, "contains")
                else:
                    prev_id = None
                    for t in tokens:
                        token_id = add_node(t, get_token_attribute(t))
                        if prev_id is not None:
                            add_edge(prev_id, token_id, "next")
                        prev_id = token_id

    # 構造体 (`struct`) の処理
    for node in ast_data:
        if node.get("node_type") == "struct":
            struct_id = add_node(node["name"], "structure")
            add_edge(mod_id, struct_id, "contains")
            # `fields` の処理
            for field in node.get("fields", []):
                # field_type が存在する場合、'<'より前の部分を属性として抽出
                if field.get("field_type"):
                    field_type = field["field_type"]
                    main_type = re.split(r'<', field_type, maxsplit=1)[0].strip()
                else:
                    main_type = "field"
                # フィールド名のノードを作成（属性に main_type を使用）
                field_id = add_node(field["name"], main_type)
                add_edge(struct_id, field_id, "has")
                
                # <> 内にフィールド名等が記載されている場合は、別ノードとして追加する
                if field.get("field_type") and '<' in field["field_type"]:
                    # field_type の内側を取得（例："Account < 'info , Vault >" の場合、"'info , Vault" 部分）
                    inner_part = field["field_type"].split('<', 1)[1].rsplit('>', 1)[0].strip()
                    # カンマで分割（例："'info , Vault" → ["'info", "Vault"]）
                    inner_fields = [token.strip() for token in inner_part.split(',')]
                    for inner_field in inner_fields:
                        inner_field_id = add_node(inner_field, "field_inner")
                        add_edge(field_id, inner_field_id, "inner_type")

    # JSON ファイルにトークングラフを保存
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(token_graph, f, indent=2, ensure_ascii=False)
    print(f"Token graph saved to {output_file}")

if __name__ == "__main__":
    main()

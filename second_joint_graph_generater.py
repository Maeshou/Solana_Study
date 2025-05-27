import re
import json
import sys

def parse_attribute(attribute):
    """
    属性文字列から "payer = xxx" や "has_one = xxx" のペアを抽出する。
    抽出されるキーは 'payer' と 'has_one' のみ対象とする。
    """
    if not attribute:
        return []
    results = []
    # key = value の形式を抽出する正規表現
    pattern = re.compile(r'(\w+)\s*=\s*(\w+)')
    matches = pattern.findall(attribute)
    for key, value in matches:
        if key in ("payer", "has_one"):
            results.append((key, value))
    return results

def add_edges_from_ast(ast_data, token_graph):
    new_edges = []
    # AST 内の各構造体 (node_type が "struct") を処理
    for item in ast_data:
        if item.get("node_type") == "struct":
            # 各フィールドについて
            fields = item.get("fields", [])
            for field in fields:
                attribute = field.get("attribute")
                # key=value ペアからエッジを作成
                key_values = parse_attribute(attribute)
                for key, target_label in key_values:
                    for node in token_graph.get("nodes", []):
                        if node.get("label") == target_label:
                            new_edge = {
                                "source": node.get("id"),
                                "target": node.get("id"),
                                "label": key
                            }
                            new_edges.append(new_edge)
                # "init" フラグのチェック
                # フィールドがアカウントの場合、attribute 内に "init" が含まれていれば
                # フィールド名 (name) と一致するノードに対して "init" エッジを追加する
                if "Account" in field.get("field_type", "") and re.search(r'\binit\b', attribute):
                    target_label = field.get("name")
                    for node in token_graph.get("nodes", []):
                        if node.get("label") == target_label:
                            new_edge = {
                                "source": node.get("id"),
                                "target": node.get("id"),
                                "label": "init"
                            }
                            new_edges.append(new_edge)
    return new_edges

def main(ast_file, token_graph_file, output_file):
    # AST ファイルを読み込み
    with open(ast_file, 'r', encoding='utf-8') as f:
        ast_data = json.load(f)
    # トークングラフファイルを読み込み
    with open(token_graph_file, 'r', encoding='utf-8') as f:
        token_graph = json.load(f)

    # AST から新たなエッジを抽出
    new_edges = add_edges_from_ast(ast_data, token_graph)
    # 既存のエッジに追加
    token_graph.setdefault("edges", []).extend(new_edges)

    # 結果を出力ファイルに保存
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(token_graph, f, indent=2, ensure_ascii=False)

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python script.py <ast_file.json> <token_graph.json> <output_file.json>")
        sys.exit(1)
    main(sys.argv[1], sys.argv[2], sys.argv[3])

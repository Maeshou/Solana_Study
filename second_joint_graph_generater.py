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
    pattern = re.compile(r'(\w+)\s*=\s*(\w+)')
    for key, value in pattern.findall(attribute):
        if key in ("payer", "has_one"):
            results.append((key, value))
    return results


def extract_constraint_comparison_edges(text, token_graph):
    """
    constraint = xxx.key() == yyy.key() または != の形式を検出し、対応するノード同士にエッジを張る。
    """
    edges = []
    if not isinstance(text, str):
        return edges

    # constraint の右辺を抽出
    match_assign = re.search(r'constraint\s*=\s*(.+)', text)
    if not match_assign:
        return edges
    rhs_full = match_assign.group(1)
    rhs = rhs_full.split(',')[0].strip()
    # print(f"constraint ari rhs → {rhs}")
    # 比較式検出（== / !=）
    comparison_pattern = re.compile(r'(\w+)\s*\.\s*key\s*\(\s*\)\s*(==|!=)\s*(\w+)\s*\.\s*key\s*\(\s*\)')

    for match in comparison_pattern.finditer(rhs):
        left_name, operator, right_name = match.groups()
        # print(f"left_name = {left_name}")
        # print(f"operater = {operator}")
        # print(f"right_name = {right_name}")
        left_nodes = [n for n in token_graph.get("nodes", []) if n.get("label") == left_name and n.get("attributes") == "Account"]
        right_nodes = [n for n in token_graph.get("nodes", []) if n.get("label") == right_name and n.get("attributes") == "Account"] 
        for ln in left_nodes:
            for rn in right_nodes:
                edges.append({
                    "source": ln["id"],
                    "target": rn["id"],
                    "label": operator
                })
    return edges


def add_edges_from_ast(ast_data, token_graph):
    new_edges = []

    for item in ast_data:
        # struct の場合、fields の中に constraint が含まれている可能性がある
        if item.get("node_type") == "struct":
            for field in item.get("fields", []):
                attribute = field.get("attribute")
                if not isinstance(attribute, str):
                    attribute = ""

                # constraint 比較式の抽出
                new_edges.extend(extract_constraint_comparison_edges(attribute, token_graph))

                # payer / has_one のパース
                for key, target_label in parse_attribute(attribute):
                    for node in token_graph.get("nodes", []):
                        if node.get("label") == target_label:
                            new_edges.append({
                                "source": node["id"],
                                "target": node["id"],
                                "label": key
                            })

                # init チェック
                if "Account" in field.get("field_type", "") and re.search(r'\binit\b', attribute):
                    target_label = field["name"]
                    for node in token_graph.get("nodes", []):
                        if node.get("label") == target_label:
                            new_edges.append({
                                "source": node["id"],
                                "target": node["id"],
                                "label": "init"
                            })

        # body があればそこにも constraint 比較式があるか調べる
        if isinstance(item.get("body"), list):
            for stmt in item["body"]:
                new_edges.extend(extract_constraint_comparison_edges(stmt, token_graph))

    return new_edges


def main(ast_file, token_graph_file, output_file):
    print(f"AST読み込み開始: {ast_file}")
    with open(ast_file, 'r', encoding='utf-8') as f:
        ast_data = json.load(f)
    print(f"1. AST読み込み完了: {len(ast_data)} items loaded")

    print(f"トークングラフ読み込み開始: {token_graph_file}")
    with open(token_graph_file, 'r', encoding='utf-8') as f:
        token_graph = json.load(f)
    node_count = len(token_graph.get('nodes', []))
    edge_count = len(token_graph.get('edges', [])) if isinstance(token_graph.get('edges'), list) else 0
    print(f"2. トークングラフ読み込み完了: nodes={node_count}, edges={edge_count}")

    print("新規エッジ抽出処理開始...")
    new_edges = add_edges_from_ast(ast_data, token_graph)
    print(f"3. 新規エッジ抽出完了: {len(new_edges)} edges")

    token_graph.setdefault("edges", []).extend(new_edges)
    total_edges = len(token_graph.get('edges', []))
    print(f"4. 合計エッジ数: {total_edges}")

    print(f"出力ファイル書き出し: {output_file}")
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(token_graph, f, indent=2, ensure_ascii=False)
    print("5. 出力ファイル書き出し完了")


if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python second_joint_graph_generater.py <ast.json> <token.json> <out.json>")
        sys.exit(1)
    main(sys.argv[1], sys.argv[2], sys.argv[3])

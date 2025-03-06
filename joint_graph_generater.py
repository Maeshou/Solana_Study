import json
import sys
import re

def load_json(filepath):
    """ JSONファイルをロードする """
    with open(filepath, "r", encoding="utf-8") as f:
        return json.load(f)

def save_json(data, filepath):
    """ JSONデータを保存する """
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)

def extract_function_nodes(token_graph):
    """ 関数ごとのノードリストを作成 """
    function_nodes = {}
    current_function = None
    current_nodes = []

    for node in token_graph["nodes"]:
        if node["attributes"] == "function":
            if current_function:
                function_nodes[current_function] = current_nodes
            current_function = node["label"]
            current_nodes = []
        if current_function:
            current_nodes.append(node)
    
    if current_function:
        function_nodes[current_function] = current_nodes

    return function_nodes

def group_token_nodes_by_assignment(token_nodes):
    """
    tokenグラフのノード列から，属性が"operator"でラベル"="の出現位置を基点として，
    その直後から次の"="までのトークン群をひとつのグループとして抽出する．
    ＝が存在しなければ空のリストを返す．
    """
    groups = []
    current_group = None
    for token in token_nodes:
        if token["attributes"] == "operator" and token["label"] == "=":
            # 新たな代入文の開始
            if current_group is not None:
                groups.append(current_group)
            current_group = []  # "="以降のグループを初期化（"="そのものはグループに含めない）
        else:
            if current_group is not None:
                current_group.append(token)
    if current_group is not None and len(current_group) > 0:
        groups.append(current_group)
    return groups

def find_identifier_in_group(group, variable):
    """ グループ内から識別子(variableと完全一致かつ属性が"identifier")の最初のノードを探す """
    for token in group:
        if token["attributes"] == "identifier" and token["label"] == variable:
            return token
    return None

def integrate_dependency_edges(token_graph, pdg):
    """ 
    pdgの依存関係情報を元にToken Graphへエッジを追加する．
    pdgの各関数について，pdg内の「＝」を含むノードの出現順と
    tokenグラフ内の「＝」オペレーターの直後のトークン群（＝代入文グループ）の順序が対応するとみなし，
    各エッジのfrom/toに対応するグループ内から対象の識別子を探索してエッジを追加する．
    """
    # pdgがリストなら最初の要素を使用
    # if isinstance(pdg, list):
    #     pdg = pdg[0]

    function_nodes = extract_function_nodes(token_graph)
    edges = token_graph["edges"]

    for func in pdg:
        function_name = func["name"]
        pdg_nodes = func["nodes"]
        pdg_edges = func["edges"]

        if function_name not in function_nodes:
            continue  # Token Graphにこの関数がない場合はスキップ

        token_nodes = function_nodes[function_name]
        # tokenグラフ内で代入文に該当するグループを抽出
        token_groups = group_token_nodes_by_assignment(token_nodes)
        # pdg側で"="を含むノードのみを抽出し，その順序とtoken_groupsの順序が対応すると仮定する
        pdg_assignment_nodes = [node for node in pdg_nodes if "=" in node["label"]]
        # マッピング: pdgの代入ノードのid -> グループの順序インデックス
        pdg_group_mapping = {}
        for idx, node in enumerate(pdg_assignment_nodes):
            pdg_group_mapping[node["id"]] = idx

        # 各pdgエッジに対して依存関係エッジをtokenグラフに追加
        for edge in pdg_edges:
            source_pdg_id = edge["from"]
            target_pdg_id = edge["to"]
            label = edge["label"]

            # "def: vault" や "data_dep: vault" の形式を正規表現で抽出
            match = re.search(r"(def|data_dep):\s*(\w+)", label)
            if not match:
                continue
            dep_type, variable = match.groups()

            # pdg側の代入ノードとtokenグループの対応があるか確認
            if source_pdg_id not in pdg_group_mapping or target_pdg_id not in pdg_group_mapping:
                continue

            source_group_index = pdg_group_mapping[source_pdg_id]
            target_group_index = pdg_group_mapping[target_pdg_id]

            # token_groupsの数が十分かチェック
            if source_group_index >= len(token_groups) or target_group_index >= len(token_groups):
                continue

            source_group = token_groups[source_group_index]
            target_group = token_groups[target_group_index]

            src_token = find_identifier_in_group(source_group, variable)
            tgt_token = find_identifier_in_group(target_group, variable)
            if not src_token or not tgt_token:
                continue

            # もしdefの場合、sourceとtargetが同一になるはず
            edges.append({
                "source": src_token["id"],
                "target": tgt_token["id"],
                "label": f"{dep_type}: {variable}"
            })

    return token_graph

def main():
    if len(sys.argv) < 4:
        print("Usage: python joint_graph_generator.py <token_graph.json> <pdg_1.json> <output.json>")
        sys.exit(1)

    token_graph_path = sys.argv[1]
    pdg_path = sys.argv[2]
    output_path = sys.argv[3]

    token_graph = load_json(token_graph_path)
    pdg = load_json(pdg_path)
    # print(type(pdg))
    # for func in pdg:
    #     print(func)
    #     print(type(func))
    # 修正: pdg がリストである場合の処理
    # if isinstance(pdg, list) and len(pdg) > 0:
    #     pdg = pdg[0]

    # if not isinstance(pdg, dict):
    #     print("[ERROR] pdg_1.json のデータ構造がリストでも辞書でもありません。")
    #     sys.exit(1)

    updated_graph = integrate_dependency_edges(token_graph, pdg)
    save_json(updated_graph, output_path)

    print(f"Updated token graph saved to {output_path}")

if __name__ == "__main__":
    main()

import json
import sys
import re

def load_json(filepath):
    """JSONファイルをロードする"""
    with open(filepath, "r", encoding="utf-8") as f:
        return json.load(f)

def save_json(data, filepath):
    """JSONデータを保存する"""
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)

def get_node_by_id(graph, node_id):
    """IDでノードを検索する"""
    for node in graph["nodes"]:
        if node["id"] == node_id:
            return node
    return None

def add_edge(graph, source, target, label):
    """エッジを追加する"""
    graph["edges"].append({
        "source": source,
        "target": target,
        "label": label
    })

def extract_function_nodes(token_graph):
    """
    関数ごとにノードをグループ化する．
    トークングラフのノードリストは、関数ノード（attributesが"function"）が現れた以降のノードを
    その関数の一部とみなしてグループ化する．
    """
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
    tokenグラフのノード列から，
    属性が"operator"でラベル"="の出現位置を基点として，
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
            current_group = []  # "="以降のグループ（"="そのものは含まない）
        else:
            if current_group is not None:
                current_group.append(token)
    if current_group is not None and len(current_group) > 0:
        groups.append(current_group)
    return groups

def find_identifier_in_group(group, variable):
    """
    グループ内から、属性が"identifier"でラベルがvariableと完全一致する最初のノードを探す
    """
    for token in group:
        if token["attributes"] == "identifier" and token["label"] == variable:
            return token
    return None

def integrate_dependency_edges(token_graph, pdg):
    """
    PDGの依存関係情報をもとに、Token Graphへエッジを追加する。
    PDGの各関数について、PDG内の「=」を含むノードの出現順と
    Token Graph内の「=」オペレーターの直後のトークン群（代入文グループ）の順序が対応すると仮定し、
    各エッジのfrom/toに対応するグループ内から対象の識別子を探索してエッジを追加する。
    """
    function_nodes = extract_function_nodes(token_graph)
    edges = token_graph["edges"]

    for func in pdg:
        function_name = func["name"]
        pdg_nodes = func["nodes"]
        pdg_edges = func["edges"]

        if function_name not in function_nodes:
            continue  # Token Graphにこの関数がない場合はスキップ

        token_nodes = function_nodes[function_name]
        # Token Graph内で代入文に該当するグループを抽出
        token_groups = group_token_nodes_by_assignment(token_nodes)
        # PDG側で"="を含むノードのみを抽出し、その順序とtoken_groupsの順序が対応すると仮定する
        pdg_assignment_nodes = [node for node in pdg_nodes if "=" in node["label"]]
        # マッピング: PDGの代入ノードのid -> グループの順序インデックス
        pdg_group_mapping = {}
        for idx, node in enumerate(pdg_assignment_nodes):
            pdg_group_mapping[node["id"]] = idx

        # 各PDGエッジに対して依存関係エッジをToken Graphに追加
        for edge in pdg_edges:
            source_pdg_id = edge["from"]
            target_pdg_id = edge["to"]
            label = edge["label"]

            # "def: vault" や "data_dep: vault" の形式から変数名を抽出
            match = re.search(r"(def|data_dep):\s*(\w+)", label)
            if not match:
                continue
            dep_type, variable = match.groups()

            # PDG側の代入ノードとTokenグループの対応があるか確認
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

            # もしdefの場合、sourceとtargetが同一になるはずなのでエッジを追加
            edges.append({
                "source": src_token["id"],
                "target": tgt_token["id"],
                "label": f"{dep_type}: {variable}"
            })

    return token_graph

def process_ctx_accounts(token_graph):
    """
    Token Graph内で"ctx.accounts."パターンを検出し，
    ・その後ろにあるトークン（例："vault"）のラベルと一致する
      構造体フィールドノード（例：attributesが"Account", "Signer", "UncheckedAccount"等）を探す。
      → 見つかった場合、構造体フィールドノードから該当トークンノードへエッジ（"ctx_link"）を追加する。
    ・さらに、割り当て式("=")の子ノードのうち、IDが小さい方を左辺ノードとみなし，
      その左辺ノードと同じラベルの場合、構造体フィールドノードから左辺ノードへエッジ（"assign_link"）を追加する。
    ・もし"ctx.accounts."の後ろにトークンが存在しない場合は，
      関数の入力構造体ノードの中から、attributesが"structure"のノードを選び，
      左辺の変数ノードと接続するエッジを追加する。
    """
    functions = extract_function_nodes(token_graph)
    for func_name, tokens in functions.items():
        # tokensは関数内のノード列（追加順になっていると仮定）
        for i in range(len(tokens) - 3):
            if tokens[i]["label"] == "ctx" and tokens[i+1]["label"] == "." and \
               tokens[i+2]["label"] == "accounts" and tokens[i+3]["label"] == ".":
                # パターン "ctx . accounts ." が検出された
                if i+4 < len(tokens):
                    target_token = tokens[i+4]  # 例："vault"
                    # Token Graph全体から、同じラベルかつattributesが対象（例："Account", "Signer", "UncheckedAccount"）のノードを探す
                    field_node = None
                    for candidate in token_graph["nodes"]:
                        if candidate["label"] == target_token["label"] and candidate["attributes"] in ["Account", "Signer", "UncheckedAccount"]:
                            field_node = candidate
                            break
                    if field_node:
                        # 構造体フィールドノード（source）から、ctx.accounts.の後ろのトークン（target）へエッジ追加
                        add_edge(token_graph, field_node["id"], target_token["id"], "ctx_link")
                        # 同じ関数内の割り当て式("=")を探索して、左辺ノード（IDが小さい方）と比較
                        for token in tokens:
                            if token["label"] == "=":
                                lhs_ids = [edge["target"] for edge in token_graph["edges"] if edge["source"] == token["id"] and edge["label"] == "lhs"]
                                if lhs_ids:
                                    lhs_id = min(lhs_ids)
                                    lhs_node = get_node_by_id(token_graph, lhs_id)
                                    if lhs_node and lhs_node["label"] == target_token["label"]:
                                        add_edge(token_graph, field_node["id"], lhs_node["id"], "assign_link")
                else:
                    # "ctx.accounts." の後ろにトークンが存在しない場合
                    struct_node = None
                    for candidate in token_graph["nodes"]:
                        if candidate["attributes"] == "structure":
                            struct_node = candidate
                            break
                    if struct_node:
                        for token in tokens:
                            if token["label"] == "=":
                                lhs_ids = [edge["target"] for edge in token_graph["edges"] if edge["source"] == token["id"] and edge["label"] == "lhs"]
                                if lhs_ids:
                                    lhs_id = min(lhs_ids)
                                    lhs_node = get_node_by_id(token_graph, lhs_id)
                                    if lhs_node:
                                        add_edge(token_graph, struct_node["id"], lhs_node["id"], "ctx_link")

def main():
    if len(sys.argv) < 4:
        print("Usage: python joint_graph_generator.py <token_graph.json> <pdg_1.json> <output.json>")
        sys.exit(1)

    token_graph_path = sys.argv[1]
    pdg_path = sys.argv[2]
    output_path = sys.argv[3]

    token_graph = load_json(token_graph_path)
    pdg = load_json(pdg_path)

    # ctx.accountsパターンに基づくエッジの追加
    process_ctx_accounts(token_graph)
    # PDG依存関係に基づくエッジの追加
    integrate_dependency_edges(token_graph, pdg)
    save_json(token_graph, output_path)

    print(f"Updated token graph saved to {output_path}")

if __name__ == "__main__":
    main()

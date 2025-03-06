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

def get_node_by_id(token_graph, node_id):
    """ノードIDでノードを検索する"""
    for node in token_graph["nodes"]:
        if node["id"] == node_id:
            return node
    return None

def add_edge(token_graph, source, target, label):
    """エッジを追加する"""
    token_graph["edges"].append({
        "source": source,
        "target": target,
        "label": label
    })

def extract_function_nodes(token_graph):
    """
    関数ごとにノードをグループ化する。
    トークングラフのノードリストは、関数ノード（attributesが"function"）が現れた以降のノードを
    その関数の一部とみなしてグループ化する。
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
    """グループ内から、属性が"identifier"でラベルがvariableと一致する最初のノードを探す"""
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
        token_groups = group_token_nodes_by_assignment(token_nodes)
        pdg_assignment_nodes = [node for node in pdg_nodes if "=" in node["label"]]
        pdg_group_mapping = {}
        for idx, node in enumerate(pdg_assignment_nodes):
            pdg_group_mapping[node["id"]] = idx

        for edge in pdg_edges:
            source_pdg_id = edge["from"]
            target_pdg_id = edge["to"]
            label = edge["label"]

            match = re.search(r"(def|data_dep):\s*(\w+)", label)
            if not match:
                continue
            dep_type, variable = match.groups()

            if source_pdg_id not in pdg_group_mapping or target_pdg_id not in pdg_group_mapping:
                continue

            source_group_index = pdg_group_mapping[source_pdg_id]
            target_group_index = pdg_group_mapping[target_pdg_id]

            if source_group_index >= len(token_groups) or target_group_index >= len(token_groups):
                continue

            source_group = token_groups[source_group_index]
            target_group = token_groups[target_group_index]

            src_token = find_identifier_in_group(source_group, variable)
            tgt_token = find_identifier_in_group(target_group, variable)
            if not src_token or not tgt_token:
                continue

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
      ・さらに、代入式の左辺の候補ノード群を収集し、その中から対象トークンとラベルが一致するものに対して、
        構造体フィールドノードから「assign_link」エッジを追加する。
      ・候補ノード群は、(1) 定義トークンから「next」エッジで連なるidentifierノード、及び
        (2) data_dep エッジから、すでに候補にあるソースノードに対応するターゲットノードを抽出する。
      ・"ctx.accounts." の後ろにトークンが存在しない場合は、構造体名称ノード（attributesが"structure"）から
        代入式の左辺ノードへ接続する。
    """
    functions = extract_function_nodes(token_graph)
    for func_name, tokens in functions.items():
        for i in range(len(tokens) - 3):
            if tokens[i]["label"] == "ctx" and tokens[i+1]["label"] == "." and \
               tokens[i+2]["label"] == "accounts" and tokens[i+3]["label"] == ".":
                if i+4 < len(tokens):
                    target_token = tokens[i+4]  # 例："vault"
                    field_node = None
                    # 構造体フィールドノードを、対象のトークンと属性（"Account", "Signer", "UncheckedAccount"）で探索
                    for candidate in token_graph["nodes"]:
                        if candidate["label"] == target_token["label"] and candidate["attributes"] in ["Account", "Signer", "UncheckedAccount"]:
                            field_node = candidate
                            break
                    if field_node:
                        add_edge(token_graph, field_node["id"], target_token["id"], "ctx_link")
                        
                        # 候補ノードを収集する処理
                        lhs_candidate_nodes = []
                        # ① 定義トークンから、nextエッジで連なるidentifierノードを候補に追加
                        for node in tokens:
                            if node.get("attributes") == "define":
                                next_edges = [e for e in token_graph["edges"] if e["source"] == node["id"] and e["label"] == "next"]
                                for edge in next_edges:
                                    next_node = get_node_by_id(token_graph, edge["target"])
                                    if next_node and next_node.get("attributes") == "identifier":
                                        lhs_candidate_nodes.append(next_node)
                        
                        # ② data_dep エッジから、候補リストに既に含まれるソースノードに対応するターゲットノードを抽出
                        for edge in token_graph["edges"]:
                            if edge.get("label", "").startswith("data_dep:"):
                                src_node = get_node_by_id(token_graph, edge["source"])
                                if src_node and src_node.get("attributes") == "identifier":
                                    # 既に候補に含まれる場合のみ処理
                                    if any(candidate["id"] == src_node["id"] for candidate in lhs_candidate_nodes):
                                        tar_node = get_node_by_id(token_graph, edge["target"])
                                        if tar_node and tar_node.get("attributes") == "identifier":
                                            lhs_candidate_nodes.append(tar_node)
                        
                        # ユニークな候補ノードにする
                        unique_candidates = {node["id"]: node for node in lhs_candidate_nodes}.values()
                        # 候補ノードの中から、対象トークンのラベルと一致するものすべてに対してエッジを追加
                        for candidate in unique_candidates:
                            if candidate["label"] == target_token["label"]:
                                add_edge(token_graph, field_node["id"], candidate["id"], "assign_link")
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

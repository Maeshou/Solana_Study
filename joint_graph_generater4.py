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

def extract_structure_nodes_from_index(nodes, start_index):
    """
    ノードリスト nodes の start_index から連続する "structure" ノード群を抽出し、
    そのグループと、グループ終了後のインデックスを返す。
    """
    
    strucure_nodes = {}
    i = start_index
    while i < len(nodes):
        if nodes[i]["attributes"] == "structure":
            current_node = []
            key = nodes[i]["label"]
            current_node.append(nodes[i])
            strucure_nodes[key] = current_node
        else: 
            current_node = []
            current_node.append(nodes[i])
            strucure_nodes[key].extend(current_node)
        i += 1    
    # while i < len(nodes) and nodes[i]["attributes"] == "structure":
    #     group.append(nodes[i])
    #     i += 1
    #return group, i
    return strucure_nodes,i

def extract_function_and_structure_nodes(token_graph):
    """
    関数ノード群と構造体ノード群を分けてグループ化する。
    
    ・関数グループの開始は attributes が "function" のノードで行い、
      グループの終了は次の function ノードが現れるか、
      または "inputs" ノードに続くノードが連続して "structure" にならない場合に区切る。
    ・structure ノード群は、グループ終了時または関数グループが開始されていない状態で、
      連続する "structure" ノード群を抽出し、最初のノードの label（構造体名）をキーとして辞書に登録する。
    """
    function_nodes = {}
    structure_nodes = {}
    nodes = token_graph["nodes"]
    current_function = None
    current_function_group = []
    i = 0
    while i < len(nodes):
        node = nodes[i]
        # print(node)
        # print(f"動作確認1i={i}")
        # 新たな関数グループの開始
        if node["attributes"] == "function":
            print("true1")
            if current_function is not None:
                function_nodes[current_function] = current_function_group
            current_function = node["label"]
            current_function_group = [node]
            i += 1
            continue

        if current_function is not None:
            # structure ノードの場合
            if node["attributes"] == "structure":
                print("true2")
                print(f"nodes[i-1]['attributes'] = {nodes[i-1]['attributes']}")
                # 直前のノードが inputs でなければ、関数グループ終了と判断
                if i == 0 or nodes[i-1]["attributes"] != "inputs":
                    print(f"動作確認i={i}")
                    print(f"nodes[i-1]['attributes'] = {nodes[i-1]['attributes']}")
                    function_nodes[current_function] = current_function_group
                    current_function = None
                    current_function_group = []
                    # ここから連続する structure ノード群を抽出
                    #group, new_index = extract_structure_nodes_from_index(nodes, i)
                    structure_nodes, new_index = extract_structure_nodes_from_index(nodes, i)    

                    # if group:
                    #     key = group[0]["label"]
                    #     if key in structure_nodes:
                    #         structure_nodes[key].extend(group)
                    #     else:
                    #         structure_nodes[key] = group
                    i = new_index
                    continue

            # その他の場合は、現在の関数グループに追加
            current_function_group.append(node)
        else:
            # 関数グループが開始していない場合で、structure ノードが現れた場合
            if node["attributes"] == "structure":
                #group, new_index = extract_structure_nodes_from_index(nodes, i)
                structure_nodes, new_index = extract_structure_nodes_from_index(nodes, i)
                # if group:
                #     key = group[0]["label"]
                #     if key in structure_nodes:
                #         structure_nodes[key].extend(group)
                #     else:
                #         structure_nodes[key] = group
                i = new_index
                continue
        i += 1

    if current_function is not None:
        function_nodes[current_function] = current_function_group

    return function_nodes, structure_nodes

def group_token_nodes_by_assignment(token_nodes):
    """
    tokenグラフのノード列から，
    属性が "operator" でラベル "=" の出現位置を基点として，
    その直後から次の "=" までのトークン群をひとつのグループとして抽出する．
    "=" が存在しなければ空のリストを返す．
    """
    groups = []
    current_group = None
    for token in token_nodes:
        if token["attributes"] == "operator" and token["label"] == "=":
            if current_group is not None:
                groups.append(current_group)
            current_group = []  # "="そのものは含めず、以降のトークン群をグループ化
        else:
            if current_group is not None:
                current_group.append(token)
    if current_group is not None and len(current_group) > 0:
        groups.append(current_group)
    return groups

def find_all_identifiers_in_group(group, variable):
    """
    グループ内の全てのトークンのうち、"label" が変数名と一致するものを返す。
    """
    return [token for token in group if token.get("label") == variable]

def integrate_dependency_edges(token_graph, pdg):
    """
    PDG の依存関係情報をもとに、Token Graph へエッジを追加する。
    PDG の各関数について、PDG 内の "=" を含むノードの出現順と
    Token Graph 内の "=" オペレーターの直後のトークン群（代入文グループ）の順序が対応すると仮定し、
    各エッジの from/to に対応するグループ内から対象の識別子を探索してエッジを追加する。
    
    同一の PDG エッジが複数存在する場合、グループ内で対象の識別子が複数見つかれば、
    その出現順に従い、各エッジで異なるトークンをターゲット（またはソース）として使用する。
    """
    function_nodes, structure_nodes = extract_function_and_structure_nodes(token_graph)
    print(f"function_nodes: {function_nodes}")
    print(f"structure_nodes: {structure_nodes}")
    edges = token_graph["edges"]

    # 各グループ・変数組ごとの利用回数を記録する辞書
    src_counter = {}
    tgt_counter = {}

    for func in pdg:
        function_name = func["name"]
        pdg_nodes = func["nodes"]
        pdg_edges = func["edges"]

        if function_name not in function_nodes:
            continue  # Token Graph にこの関数がない場合はスキップ

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

            # グループ内の該当識別子を全件取得
            src_tokens = find_all_identifiers_in_group(source_group, variable)
            tgt_tokens = find_all_identifiers_in_group(target_group, variable)

            if not src_tokens or not tgt_tokens:
                continue

            # それぞれのグループ・変数組に対してこれまでに利用した回数を取得
            src_index = src_counter.get((source_group_index, variable), 0)
            tgt_index = tgt_counter.get((target_group_index, variable), 0)

            # 利用可能な件数内であれば該当するインデックスのトークンを、超えていれば最初のものを利用
            src_token = src_tokens[src_index] if src_index < len(src_tokens) else src_tokens[0]
            tgt_token = tgt_tokens[tgt_index] if tgt_index < len(tgt_tokens) else tgt_tokens[0]

            # カウンタをインクリメント
            src_counter[(source_group_index, variable)] = src_index + 1
            tgt_counter[(target_group_index, variable)] = tgt_index + 1

            edges.append({
                "source": src_token["id"],
                "target": tgt_token["id"],
                "label": f"{dep_type}: {variable}"
            })

    return token_graph


def assign_link_generater(field_nodes,lhs_candidate_nodes,token_graph):
    # (2) data_dep エッジから、既に候補に含まれるソースノードに接続するターゲットノードを追加
    for edge in token_graph["edges"]:
        if edge.get("label", "").startswith("data_dep:"):
            print(f"dep_edgeあり")
            src_node = get_node_by_id(token_graph, edge["source"])
            if src_node and src_node.get("attributes") == "identifier":
                if any(candidate["id"] == src_node["id"] for candidate in lhs_candidate_nodes):
                    tar_node = get_node_by_id(token_graph, edge["target"])
                    if tar_node and tar_node.get("attributes") == "identifier":
                        print(f"tar_node={tar_node}")
                        lhs_candidate_nodes.append(tar_node)
    print(f"lhs_candidate_nodes={lhs_candidate_nodes}")
    # ユニークな候補ノードにする
    unique_candidates = {node["id"]: node for node in lhs_candidate_nodes}.values()
    print(f"unique_candidates = {unique_candidates}")
    # 候補ノードの中から対象トークンのラベルに一致するものにエッジ追加
    for field_node in field_nodes:
        for candidate in unique_candidates:
            print(f"assign_linkのtarget={candidate}")
            add_edge(token_graph, field_node["id"], candidate["id"], "assign_link")


def process_ctx_accounts(token_graph):
    """
    Token Graph 内で "ctx.accounts." パターンを検出し，
      ・その後ろにあるトークン（例："vault"）のラベルと一致する
        構造体フィールドノード（例：attributes が "Account", "Signer", "UncheckedAccount" 等）を探す。
        → 見つかった場合、構造体フィールドノードから該当トークンノードへエッジ ("ctx_link") を追加する。
      ・さらに、代入式の左辺の候補ノード群を収集し、その中から対象トークンとラベルが一致するものに対して、
        構造体フィールドノードから "assign_link" エッジを追加する。
      ・候補ノード群は、(1) 定義トークンから "next" エッジで連なる identifier ノード、及び
        (2) data_dep エッジから、すでに候補にあるソースノードに対応するターゲットノードを抽出する。
      ・"ctx.accounts." の後ろにトークンが存在しない場合は、構造体名称ノード（attributes が "structure"）から
        代入式の左辺ノードへ接続する。
    """
    functions, structs = extract_function_and_structure_nodes(token_graph)
    for func_name, tokens in functions.items():
        print(func_name)
        groups = group_token_nodes_by_assignment(tokens)
        for group in groups:    
            for i in range(len(group) - 3):
                if group[i]["label"] == "ctx" and group[i+1]["label"] == "." and \
                group[i+2]["label"] == "accounts" and group[i+3]["label"] == ".":
                    if i+4 < len(group):
                        target_token = group[i+4]  # 例："vault"
                        field_nodes = []
                        # 対象トークンのラベルと属性 ("Account", "Signer", "UncheckedAccount") に一致する構造体フィールドノードを探索
                        for candidate in token_graph["nodes"]:
                            if candidate["label"] == target_token["label"] and candidate["attributes"] in ["Account", "Signer", "UncheckedAccount"]:
                                field_nodes.append(candidate)
                        if len(field_nodes) > 0:
                            for field_node in field_nodes:
                                add_edge(token_graph, field_node["id"], target_token["id"], "ctx_link")

                            # 候補ノード収集処理
                            lhs_candidate_nodes = []
                            # (1) 定義トークンから next エッジで連なる identifier ノードを追加
                            for node in group:
                                if node.get("attributes") == "define":
                                    next_edges = [e for e in token_graph["edges"] if e["source"] == node["id"] and e["label"] == "next"]
                                    for edge in next_edges:
                                        next_node = get_node_by_id(token_graph, edge["target"])
                                        if next_node and next_node.get("attributes") == "identifier":
                                            lhs_candidate_nodes.append(next_node)

                            assign_link_generater(field_nodes,lhs_candidate_nodes,token_graph)
                            

                elif group[i]["label"] == "ctx" and group[i+1]["label"] == "." and \
                    group[i+2]["label"] == "accounts" and group[i+3]["label"] == ";":
                    print("yes5")
                    # "ctx.accounts." の後ろにトークンが存在しない場合
                    input_node = None
                    j = 0
                    for token in tokens:
                        if token["attributes"] == "inputs":
                            print(f"動作確認tokens[i+2]['label'] = {group[i+2]['label']}")
                            input_node = tokens[j+1]
                            print(f"動作確認struct_node = {input_node}")
                            add_edge(token_graph, input_node["id"], group[i+2]["id"], "ctx_link")
                            for struct_name, struct in structs.items():
                                if struct_name == input_node["label"]:
                                    print("yes1")
                                    for struct_node in struct:
                                        if struct_node["label"] == input_node["label"]:
                                            print("yes2")
                                            add_edge(token_graph, struct_node["id"], group[i+2]["id"], "ctx_link")
                                
                                #break
                        j += 1
                    if input_node:
                        lhs_candidate_nodes = []
                        src_nodes=[]
                        for token in group:
                            if token["attributes"] == "define":
                                defined_node_ids = [edge["target"] for edge in token_graph["edges"] if edge["source"] == token["id"] and edge["label"] == "next"]
                                if defined_node_ids:
                                    defined_node_id = min(defined_node_ids)
                                    defined_node = get_node_by_id(token_graph, defined_node_id)
                                    if defined_node:
                                        lhs_candidate_nodes.append(defined_node)
                                        print(f"動作確認defined_node={defined_node}")
                                        for struct_name, struct in structs.items():
                                            if struct_name == input_node["label"]:
                                                for struct_node in struct:
                                                    if struct_node["label"] == input_node["label"]:
                                                        src_nodes.append(struct_node)
                                                        print("yes3")
                                                        print(f"struct_node['id']={struct_node['id']}")
                                                        assign_link_generater(src_nodes,lhs_candidate_nodes,token_graph)
                                                        #add_edge(token_graph, struct_node["id"],  defined_node["id"], "assign_link")

def main():
    if len(sys.argv) < 4:
        print("Usage: python joint_graph_generator.py <token_graph.json> <pdg_1.json> <output.json>")
        sys.exit(1)

    token_graph_path = sys.argv[1]
    pdg_path = sys.argv[2]
    output_path = sys.argv[3]

    token_graph = load_json(token_graph_path)
    pdg = load_json(pdg_path)

    # PDG 依存関係に基づくエッジの追加
    integrate_dependency_edges(token_graph, pdg)
    # ctx.accounts パターンに基づくエッジの追加
    process_ctx_accounts(token_graph)
    save_json(token_graph, output_path)

    print(f"Updated token graph saved to {output_path}")

if __name__ == "__main__":
    main()

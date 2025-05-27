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

def is_parameter_target(token_graph, node):
    """
    指定されたノードが、token_graph 内の parameter エッジの target ノードになっているかを判定する。
    """
    for edge in token_graph["edges"]:
        if edge.get("label") == "parameter" and edge.get("target") == node.get("id"):
            return True
    return False

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
    return strucure_nodes, i

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
        if node["attributes"] == "function":
            #print("true1")
            #print(f"動作確認A i={i}")
            if current_function is not None:
                function_nodes[current_function] = current_function_group
            current_function = node["label"]
            current_function_group = [node]
            i += 1
            continue

        if current_function is not None:
            if node["attributes"] == "structure":
                #print("true2")
                # 前のノードが parameter エッジの target になっているかを判定
                if i == 0 or not is_parameter_target(token_graph, nodes[i]):
                    #print(f"動作確認B i={i}")
                    #print(f"nodes[i-1]['attributes'] = {nodes[i-1]['attributes']}")
                    function_nodes[current_function] = current_function_group
                    current_function = None
                    current_function_group = []
                    structure_nodes, new_index = extract_structure_nodes_from_index(nodes, i)    
                    i = new_index
                    continue
            current_function_group.append(node)
        else:
            if node["attributes"] == "structure":
                #print(f"動作確認C i={i}")
                structure_nodes, new_index = extract_structure_nodes_from_index(nodes, i)
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
        if token["attributes"] == "expression" and token["label"] == "expression":
            current_group = []  
        elif token["attributes"] == "delimiter" and token["label"] == ";":
            if current_group is not None:
                groups.append(current_group)
            current_group = []    
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

###############################################################################
# 新たに追加する関数： add_inner_fields_edge
###############################################################################
def add_inner_fields_edge(token_graph, struct):
    """
    (1) ctx_link エッジの処理:
        - ターゲットノードから、next エッジで連なる member ノード（attributes=="member", label=="."）と、
          その直後の identifier ノードを取得する。
        - 元の ctx_link エッジのソースがフィールド値（attributes=="value"）の場合、ソースから field_inner エッジで
          内部フィールドの名前を取得し、struct 辞書で対応する構造体ノード群を調べる。
        - 各構造体ノードの "has" エッジでつながる子ノードのラベルが、取得した identifier ノードのラベルと一致すれば、
          その子ノード（構造体のフィールド値ノード）をソース、identifier ノードをターゲットとする新たなエッジ（"ctx_link"）を追加する.
    
    (2) "=" ノードと入力値の連結処理:
        - 属性が "operator" でラベルが "=" のノードから、"rhs" または next チェーンで identifier ノードを取得する。
        - その identifier ノードのラベル（変数名）と、token_graph 内のパラメータエッジの target ノードのラベルを比較し、
          一致する場合、source を parameter ノード、target を identifier ノードとするエッジ（"input_link"）を追加する.
        - ただし、対象ノードの属性が "structure" の場合はこの処理を行わない。
    
    (3) ".<field>" チェーンの処理（assign_link エッジ検知版）:
        - assign_link エッジのターゲットから、next エッジで dot ノード（attributes=="member", label=="."）を取得し、
          その後ろの identifier ノードを取得してフィールド名とする.
        - エッジのソースが構造体名の場合、ソースの "has" エッジからフィールド名と一致するフィールド値ノードを取得し、
          新たなエッジを追加する.
        - エッジのソースがフィールド値の場合は、ソースのラベルから抽出した変数名とフィールド名が一致すれば、
          そのフィールド値ノードと identifier ノードを結ぶエッジを追加する.
    """
    def get_next_node(source_id, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == "next" and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    def get_child_node(source_id, edge_label, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == edge_label and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    # (1) ctx_link エッジの処理
    for edge in list(token_graph["edges"]):
        if "ctx_link" in edge.get("label", ""):
            target = get_node_by_id(token_graph, edge["target"])
            if not target:
                continue
            
            member_node = get_next_node(target["id"], condition=lambda n: n.get("attributes") == "member" and n.get("label") == ".")
            if not member_node:
                continue
            identifier_node = get_next_node(member_node["id"], condition=lambda n: n.get("attributes") == "identifier")
            if not identifier_node:
                continue
            identifier_label = identifier_node.get("label", "").strip()
            source = get_node_by_id(token_graph, edge["source"])
            if not source:
                continue
            if source.get("attributes") == "Account":
                field_inner = get_child_node(source["id"], "inner_type")
                if field_inner:
                    inner_field_name = field_inner.get("label", "").strip()
                    if inner_field_name in struct:
                        for s_node in struct[inner_field_name]:
                            has_child = get_child_node(s_node["id"], "has", condition=lambda n: n.get("label") == identifier_label)
                            if has_child:
                                add_edge(token_graph, has_child["id"], identifier_node["id"], "field_link")
    # (2) "=" ノードと入力値の連結処理
    for eq_node in token_graph["nodes"]:
        if eq_node.get("label") == "=" and eq_node.get("attributes") == "operator":
            rhs_node = get_child_node(eq_node["id"], "rhs")
            if not rhs_node or rhs_node.get("attributes") != "identifier":
                rhs_node = get_next_node(eq_node["id"], condition=lambda n: n.get("attributes") == "identifier")
            if not rhs_node or rhs_node.get("attributes") != "identifier":
                continue
            identifier_name = rhs_node.get("label", "").strip()
            # token_graph内のエッジを探索して、parameterエッジのtargetノードを取得する
            for edge in token_graph["edges"]:
                if edge.get("label") == "parameter":
                    param_node = get_node_by_id(token_graph, edge["target"])
                    # 対象ノードの属性が "structure" の場合はこの処理をスキップ
                    if param_node.get("attributes") == "structure":
                        continue
                    # ノードのラベルをそのまま使用して比較
                    label = param_node.get("label", "").strip()
                    if label == identifier_name:
                        add_edge(token_graph, param_node["id"], rhs_node["id"], "input_link")
    # (3) ".<field>" チェーンの処理（assign_link エッジ検知版）
    for edge in list(token_graph["edges"]):
        label = edge.get("label", "")
        if "assign_link" in label:
            target = get_node_by_id(token_graph, edge["target"])
            if not target:
                continue
            dot_node = get_next_node(target["id"], condition=lambda n: n.get("attributes") == "member" and n.get("label") == ".")
            if not dot_node:
                continue
            field_identifier = get_next_node(dot_node["id"], condition=lambda n: n.get("attributes") == "identifier")
            if not field_identifier:
                continue
            field_name = field_identifier.get("label", "").strip()
            source = get_node_by_id(token_graph, edge["source"])
            if not source:
                continue
            if source.get("attributes") == "structure":
                field_child = get_child_node(source["id"], "has", condition=lambda n: n.get("label") == field_name)
                if field_child:
                    add_edge(token_graph, field_child["id"], field_identifier["id"], "Account_link")
            elif source.get("attributes") == "Account":
                var_name = source.get("label", "").split(":")[0].strip()
                if var_name == field_name:
                    add_edge(token_graph, source["id"], field_identifier["id"], "field_link")
    
    return token_graph

###############################################################################
# 新たに追加する関数： process_account_link_inner
###############################################################################
def process_account_link_inner(token_graph, struct):
    """
    Account_linkを持つエッジに対して、以下の処理を行う：
      1. Account_linkエッジのターゲットノードから、nextエッジをたどって、
         attributesが"member"でラベルが"."のノード、その後に属性が"identifier"のノード（変数ノード）が連なるか確認する。
      2. もし連なっていれば、その変数ノードのラベル（変数名）を取得する。
      3. 次に、Account_linkを持つノード（ソースノード）のラベルと同じ名前の構造体フィールドノードが、
         "inner_type"ラベルのエッジで、"field_inner"属性を持つノードと接続されているはずなのでそれを調べる。
      4. その"field_inner"属性を持つノードと同じ名前のstructure属性のノードが存在する場合、
         その構造体ノード群のうち、attributesが"value"のノード（フィールド値ノード）を抽出し、
         そのフィールド値ノードのラベル（":" の前の部分）と先ほど取得した変数ノードのラベルが一致すれば、
         inner_link というラベルのエッジを追加する。
    """
    def get_next_node(source_id, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == "next" and edge.get("source") == source_id:
                print(f"source_id = {source_id}")
                candidate = get_node_by_id(token_graph, edge.get("target"))
                print(f"candidate4 = {candidate}")
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    def get_child_node(source_id, edge_label, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == edge_label and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    for edge in list(token_graph["edges"]):
        if edge.get("label") == "Account_link":
            account_node = get_node_by_id(token_graph, edge["target"])
            if not account_node:
                continue
            member_node = get_next_node(account_node["id"], condition=lambda n: n.get("attributes")=="member" and n.get("label") == ".")
            #print(f"member_node2 = {member_node}")
            if not member_node:
                continue
            variable_node = get_next_node(member_node["id"], condition=lambda n: n.get("attributes")=="identifier")
            #print(f"variable_node = {variable_node}")
            if not variable_node:
                continue
            var_name = variable_node.get("label", "").strip()
            #print(f"account_node.get('label') = {account_node.get('label')}")
            field_node = get_node_by_id(token_graph, edge["source"])
            #print(f"field_node = {field_node}")
            inner_node = get_child_node(field_node["id"], "inner_type", condition=lambda n: n.get("attributes")=="field_inner" and n.get("label") in struct.keys())
            #print(f"inner_node={inner_node}")
            if inner_node is None:
                continue
            for s_name, s_nodes in struct.items():
                if s_name == inner_node["label"]:
                    for s_node in s_nodes:
                        if s_node["label"] == var_name:
                            #print("inner_linkあり")
                            add_edge(token_graph, s_node["id"], variable_node["id"], "inner_link")
    return token_graph

###############################################################################
# 新たに追加する関数： process_assign_link_inner
###############################################################################
def process_assign_link_inner(token_graph, struct):
    """
    assign_linkエッジに対して以下の処理を行う：
      1. assign_linkエッジのターゲットノードから、nextエッジをたどって、
         attributesが"member"でラベルが"."のノード、その後に属性が"identifier"のノード（変数ノード）が連なるか確認する。
      2. もし連なっていれば、その identifier ノードから変数名を取得する。
      3. 次に、assign_linkエッジのソースノードから、"inner_type" エッジで inner ノード（attributes=="field_inner"）を取得する。
      4. inner ノードのラベルに該当する構造体フィールドノード群（struct 辞書）から、
         変数名と一致するフィールドノードがあれば、該当するフィールドノードと identifier ノードの間に inner_link エッジを追加する.
    """
    def get_next_node(source_id, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == "next" and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    def get_child_node(source_id, edge_label, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == edge_label and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    for edge in list(token_graph["edges"]):
        if edge.get("label") == "assign_link":
            # assign_linkエッジの target ノードを取得
            assign_target = get_node_by_id(token_graph, edge["target"])
            if not assign_target:
                continue
            # assign_targetから、next エッジで member ノード（attributes=="member", label=="."）を取得
            member_node = get_next_node(assign_target["id"], condition=lambda n: n.get("attributes")=="member" and n.get("label") == ".")
            if not member_node:
                continue
            # member_nodeに続く、identifier ノードを取得
            variable_node = get_next_node(member_node["id"], condition=lambda n: n.get("attributes")=="identifier")
            if not variable_node:
                continue
            var_name = variable_node.get("label", "").strip()
            # assign_linkエッジのソースノードを取得
            source_node = get_node_by_id(token_graph, edge["source"])
            if not source_node:
                continue
            # ソースノードから、"inner_type" エッジで inner ノード（attributes=="field_inner"）を取得
            inner_node = get_child_node(source_node["id"], "inner_type", condition=lambda n: n.get("attributes")=="field_inner" and n.get("label") in struct.keys())
            if inner_node is None:
                continue
            for s_name, s_nodes in struct.items():
                if s_name == inner_node["label"]:
                    for s_node in s_nodes:
                        if s_node["label"] == var_name:
                            # inner_link エッジを追加
                            add_edge(token_graph, s_node["id"], variable_node["id"], "inner_link")
    return token_graph

###############################################################################
# 新たに追加する関数： process_ctx_link_inner
###############################################################################
def process_ctx_link_inner(token_graph, struct):
    """
    ctx_linkエッジに対して、以下の処理を行う：
      1. ctx_linkエッジのターゲットノードから、nextエッジをたどって、
         attributesが"member"でラベルが"."のノード、その後に属性が"identifier"のノード（変数ノード）が連なるか確認する。
      2. もし連なっていれば、その変数ノードのラベル（変数名）を取得する。
      3. 次に、ctx_linkを持つエッジのソースノードのラベルと同じ名前の構造体フィールドノードが、
         "inner_type"ラベルのエッジで、"field_inner"属性を持つノードと接続されているはずなのでそれを調べる。
      4. その"field_inner"属性を持つノードと同じ名前のstructure属性のノードが存在する場合、
         その構造体ノード群のうち、attributesが"value"のノード（フィールド値ノード）を抽出し、
         そのフィールド値ノードのラベル（":" の前の部分）と先ほど取得した変数ノードのラベルが一致すれば、
         inner_link というラベルのエッジを追加する.
    """
    def get_next_node(source_id, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == "next" and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    def get_child_node(source_id, edge_label, condition=None):
        for edge in token_graph["edges"]:
            if edge.get("label") == edge_label and edge.get("source") == source_id:
                candidate = get_node_by_id(token_graph, edge.get("target"))
                if candidate and (condition is None or condition(candidate)):
                    return candidate
        return None

    for edge in list(token_graph["edges"]):
        if edge.get("label") == "ctx_link":
            # 1. ctx_linkエッジのターゲットノードからnextエッジをたどる
            ctx_target = get_node_by_id(token_graph, edge["target"])
            if not ctx_target:
                continue
            member_node = get_next_node(ctx_target["id"],
                                        condition=lambda n: n.get("attributes") == "member" and n.get("label") == ".")
            if not member_node:
                continue
            identifier_node = get_next_node(member_node["id"],
                                            condition=lambda n: n.get("attributes") == "identifier")
            if not identifier_node:
                continue
            var_name = identifier_node.get("label", "").strip()
            # 3. ctx_linkエッジのターゲットノード（ctx_target）のラベルと一致する構造体フィールドノードを探す
            field_node = get_node_by_id(token_graph, edge["source"])
            inner_node = get_child_node(field_node["id"], "inner_type",
                                         condition=lambda n: n.get("attributes") == "field_inner" and n.get("label") in struct.keys())
            if inner_node is None:
                continue
            for s_name, s_nodes in struct.items():
                if s_name == inner_node["label"]:
                    for s_node in s_nodes:
                        if s_node["label"] == var_name:
                            add_edge(token_graph, s_node["id"], identifier_node["id"], "inner_link")
    return token_graph

###############################################################################
# 新たに追加する関数： remove_edges
###############################################################################
def remove_edges(token_graph):
    """
    ctx_link の target ノードとなっているノードをターゲットとする
    def エッジまたは data_dep エッジを削除する。
    """
    # まず、すべての ctx_link エッジのターゲットノードのIDを収集する
    ctx_targets = set()
    for edge in token_graph["edges"]:
        if "ctx_link" in edge.get("label", ""):
            ctx_targets.add(edge["target"])
    # 新しいエッジリストを作成（該当するエッジは除去する）
    new_edges = []
    for edge in token_graph["edges"]:
        label = edge.get("label", "")
        # ラベルが "def:" または "data_dep:" であり、かつそのターゲットが ctx_link のターゲットノードの場合は削除
        if (label.startswith("def:") or label.startswith("data_dep:")) and edge["target"] in ctx_targets:
            continue
        new_edges.append(edge)
    token_graph["edges"] = new_edges

###############################################################################
# PDG 依存関係に基づくエッジ追加処理
###############################################################################
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
    edges = token_graph["edges"]

    #src_counter = {}
    #tgt_counter = {}

    for func in pdg:
        function_name = func["name"]
        pdg_nodes = func["nodes"]
        pdg_edges = func["edges"]

        if function_name not in function_nodes:
            continue

        token_nodes = function_nodes[function_name]
        token_groups = group_token_nodes_by_assignment(token_nodes)
        #print(f"token_groups = {token_groups}")
        pdg_assignment_nodes = [node for node in pdg_nodes if ";" in node["label"]]
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

            src_tokens = find_all_identifiers_in_group(source_group, variable)
            tgt_tokens = find_all_identifiers_in_group(target_group, variable)

            if not src_tokens or not tgt_tokens:
                continue

            #src_index = src_counter.get((source_group_index, variable), 0)
            src_index = 0
            #tgt_index = tgt_counter.get((target_group_index, variable), 0)

            src_token = src_tokens[src_index] #if src_index < len(src_tokens) else src_tokens[0]
            #tgt_token = tgt_tokens[tgt_index] if tgt_index < len(tgt_tokens) else tgt_tokens[0]

            #src_counter[(source_group_index, variable)] = src_index + 1
            #tgt_counter[(target_group_index, variable)] = tgt_index + 1
            for tgt_token in tgt_tokens:
                edges.append({
                    "source": src_token["id"],
                    "target": tgt_token["id"],
                    "label": f"{dep_type}: {variable}"
                })

    return token_graph

###############################################################################
# ctx.accounts パターンに基づくエッジ追加処理
###############################################################################
def assign_link_generater(field_nodes, lhs_candidate_nodes, token_graph):
    for edge in token_graph["edges"]:
        if edge.get("label", "").startswith("data_dep:"):
            #print(f"dep_edgeあり")
            src_node = get_node_by_id(token_graph, edge["source"])
            if src_node and src_node.get("attributes") == "identifier":
                if any(candidate["id"] == src_node["id"] for candidate in lhs_candidate_nodes):
                    tar_node = get_node_by_id(token_graph, edge["target"])
                    if tar_node and tar_node.get("attributes") == "identifier":
                        #print(f"tar_node={tar_node}")
                        lhs_candidate_nodes.append(tar_node)
    #print(f"lhs_candidate_nodes={lhs_candidate_nodes}")
    unique_candidates = {node["id"]: node for node in lhs_candidate_nodes}.values()
    #print(f"unique_candidates = {unique_candidates}")
    for field_node in field_nodes:
        for candidate in unique_candidates:
            #print(f"assign_linkのtarget={candidate}")
            add_edge(token_graph, field_node["id"], candidate["id"], "assign_link")

def process_ctx_accounts(token_graph):
    """
    Token Graph 内で "ctx.accounts." パターンを検出し，
      ・その後ろにあるトークン（例："vault"）のラベルと一致する
        構造体フィールドノード（例：attributes が "Account", "Signer", "UncheckedAccount" 等）を探す。
        → 見つかった場合、構造体フィールドノードから該当トークンノードへエッジ ("ctx_link") を追加する。
      ・さらに、代入式の左辺の候補ノード群を収集し、その中から対象トークンとラベルが一致するものに対して，
        構造体フィールドノードから "assign_link" エッジを追加する。
      ・候補ノード群は、(1) 定義トークンから "next" エッジで連なる identifier ノード、及び
        (2) data_dep エッジから、すでに候補にあるソースノードに対応するターゲットノードを抽出する。
      ・"ctx.accounts." の後ろにトークンが存在しない場合は、構造体名称ノード（attributes が "structure"）から
        代入式の左辺ノードへ接続する。
    """
    functions, structs = extract_function_and_structure_nodes(token_graph)
    #print(f"(functions)={functions}")
    #print(f"(structs)={structs}")
    for func_name, tokens in functions.items():
        #print(f"func_name={func_name}")
        groups = group_token_nodes_by_assignment(tokens)
        for group in groups:    
            for i in range(len(group) - 3):
                if group[i]["label"] == "ctx" and group[i+1]["label"] == "." and \
                   group[i+2]["label"] == "accounts" and group[i+3]["label"] == ".":
                    if i+4 < len(group):
                        target_token = group[i+4]
                        field_nodes = []
                        for struct_items in structs.values():
                            for candidate in struct_items:
                                if candidate["label"] == target_token["label"]:
                                    field_nodes.append(candidate)
                        if len(field_nodes) > 0:
                            for field_node in field_nodes:
                                add_edge(token_graph, field_node["id"], target_token["id"], "ctx_link")
                            lhs_candidate_nodes = []
                            for node in group:
                                if node.get("attributes") == "define":
                                    next_edges = [e for e in token_graph["edges"] if e["source"] == node["id"] and e["label"] == "next"]
                                    for edge in next_edges:
                                        next_node = get_node_by_id(token_graph, edge["target"])
                                        if next_node and next_node.get("attributes") == "identifier":
                                            lhs_candidate_nodes.append(next_node)
                            assign_link_generater(field_nodes, lhs_candidate_nodes, token_graph)
                elif group[i]["label"] == "ctx" and group[i+1]["label"] == "." and \
                     group[i+2]["label"] == "accounts" and group[i+3]["label"] == ";":
                    #print("yes5")
                    input_node = None
                    j = 0
                    for token in tokens:
                        if token["attributes"] == "inputs":
                            #print(f"動作確認tokens[i+2]['label'] = {group[i+2]['label']}")
                            input_node = tokens[j+1]
                            #print(f"動作確認struct_node = {input_node}")
                            add_edge(token_graph, input_node["id"], group[i+2]["id"], "ctx_link")
                            for struct_name, struct in structs.items():
                                if struct_name == input_node["label"]:
                                    #print("yes1")
                                    for struct_node in struct:
                                        if struct_node["label"] == input_node["label"]:
                                            #print("yes2")
                                            add_edge(token_graph, struct_node["id"], group[i+2]["id"], "ctx_link")
                        j += 1
                    if input_node:
                        lhs_candidate_nodes = []
                        src_nodes = []
                        for token in group:
                            if token["attributes"] == "define":
                                defined_node_ids = [edge["target"] for edge in token_graph["edges"] if edge["source"] == token["id"] and edge["label"] == "next"]
                                if defined_node_ids:
                                    defined_node_id = min(defined_node_ids)
                                    defined_node = get_node_by_id(token_graph, defined_node_id)
                                    if defined_node:
                                        lhs_candidate_nodes.append(defined_node)
                                       #print(f"動作確認defined_node={defined_node}")
                                        for struct_name, struct in structs.items():
                                            if struct_name == input_node["label"]:
                                                for struct_node in struct:
                                                    src_nodes.append(struct_node)
                                                    #print("yes3")
                                                    #print(f"struct_node['id']={struct_node['id']}")
                                                    assign_link_generater(src_nodes, lhs_candidate_nodes, token_graph)

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
    # inner_fields エッジの処理（process_ctx_accounts の後）
    _, structure_nodes = extract_function_and_structure_nodes(token_graph)
    add_inner_fields_edge(token_graph, structure_nodes)
    # Account_link を持つノードに対する inner_link エッジの追加
    process_account_link_inner(token_graph, structure_nodes)
    # 追加：ctx_link を持つエッジに対する inner_link エッジの追加
    process_ctx_link_inner(token_graph, structure_nodes)
    # 追加：assign_link を持つエッジに対する inner_link エッジの追加
    process_assign_link_inner(token_graph, structure_nodes)
    # 追加：ctx_linkのtargetノードとなっているノードがtargetノードとなるdefエッジまたはdata_depエッジを削除する
    remove_edges(token_graph)
    
    save_json(token_graph, output_path)

    print(f"Updated token graph saved to {output_path}")

if __name__ == "__main__":
    main()

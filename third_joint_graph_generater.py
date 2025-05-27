import sys
import json

def load_json(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        return json.load(f)

def save_json(data, filepath):
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

def ensure_all_nodes_have_attributes(graph, default_attr=""):
    """
    グラフ内のすべてのノードに "attributes" キーが存在することを保証します。
    存在しない場合は、default_attr の値を設定します。
    """
    for node in graph.get("nodes", []):
        if "attributes" not in node:
            node["attributes"] = default_attr

def enhance_cfg_if_structure(cfg_data, start_new_id=100):
    """
    CFG内の各関数に対して、IF文のノード群を分割してグループ化し、
    さらにエッジを追加する変換を行います。
    各関数は "nodes" と "edges" を持つと仮定しています。
    
    IF文（ラベルが "if ..." で始まるノード）の場合、以下の処理を行います:
      1. IFノードのラベルを "IF" に変更
      2. ラベル中の条件部（括弧内の部分）を抽出し、"PREDICATE: <条件>" ノードを生成
      3. "TRUEBODY" と "FALSEBODY" ノードを生成
      4. IF ノードに対して、生成したノード群の情報を "if_group" キーで保持する
      5. 元々 IF ノードから出ていた "true"、"false" エッジは削除し、以下のエッジを追加します:
           - IFノード → PREDICATE（ラベル "predicate"）
           - IFノード → TRUEBODY（ラベル "true"）
           - TRUEBODY → 元の "true" エッジのターゲット（元のエッジラベルを維持）
           - IFノード → FALSEBODY（ラベル "false"）
           - FALSEBODY → 元の "false" エッジのターゲット（同様）
    """
    new_nodes = []
    new_edges = []
    new_id = start_new_id

    # cfg_dataは各関数オブジェクトのリストであると仮定
    for func in cfg_data:
        if "nodes" not in func or "edges" not in func:
            continue
        nodes = func["nodes"]
        edges = func["edges"]
        edges_to_remove = []
        for node in nodes:
            label = node.get("label", "").strip()
            # "if" で始まるか（大文字小文字を区別せず）
            if label.lower().startswith("if"):
                if_node_id = node["id"]
                # 条件部の抽出：最初の "(" と最後の ")" の間
                open_paren = label.find("(")
                close_paren = label.rfind(")")
                condition = label[open_paren+1:close_paren] if open_paren != -1 and close_paren != -1 else ""
                # IFノードのラベルを "IF" に変更
                node["label"] = "IF"
                # IFノードにグループ情報を追加
                node["if_group"] = {}

                # 新規ノードの生成（"attributes" キーを追加）
                predicate_node = {
                    "id": new_id,
                    "label": f"PREDICATE: {condition}",
                    "attributes": "predicate"
                }
                node["if_group"]["predicate"] = new_id
                new_id += 1

                truebody_node = {
                    "id": new_id,
                    "label": "TRUEBODY",
                    "attributes": "truebody"
                }
                node["if_group"]["truebody"] = new_id
                new_id += 1

                falsebody_node = {
                    "id": new_id,
                    "label": "FALSEBODY",
                    "attributes": "falsebody"
                }
                node["if_group"]["falsebody"] = new_id
                new_id += 1

                new_nodes.extend([predicate_node, truebody_node, falsebody_node])

                # 元々IFノードから出ている "true" および "false" エッジを収集
                true_edges = []
                false_edges = []
                for edge in edges:
                    if edge["from"] == if_node_id:
                        if edge["label"] == "true":
                            true_edges.append(edge)
                            edges_to_remove.append(edge)
                        elif edge["label"] == "false":
                            false_edges.append(edge)
                            edges_to_remove.append(edge)
                # 該当エッジを削除
                func["edges"] = [edge for edge in edges if edge not in edges_to_remove]
                edges = func["edges"]

                # 新たなエッジの追加
                new_edges.append({"from": if_node_id, "to": predicate_node["id"], "label": "predicate"})
                new_edges.append({"from": if_node_id, "to": truebody_node["id"], "label": "true"})
                for edge in true_edges:
                    new_edges.append({"from": truebody_node["id"], "to": edge["to"], "label": edge["label"]})
                new_edges.append({"from": if_node_id, "to": falsebody_node["id"], "label": "false"})
                for edge in false_edges:
                    new_edges.append({"from": falsebody_node["id"], "to": edge["to"], "label": edge["label"]})
    return new_nodes, new_edges

def merge_with_joint_graph(joint_graph, additional_nodes, additional_edges):
    """
    既存の Joint グラフ（{"nodes": [...], "edges": [...]}）に
    新たなノード・エッジを追加して統合します。
    """
    if "nodes" not in joint_graph:
        joint_graph["nodes"] = []
    if "edges" not in joint_graph:
        joint_graph["edges"] = []
    joint_graph["nodes"].extend(additional_nodes)
    joint_graph["edges"].extend(additional_edges)
    return joint_graph

def process_call_edges(token_graph):
    """
    CALLエッジを持つノードについて、エッジの先のノードのラベルから関数名を抽出し、
    同名の関数ノード（attributes が "function" のノード）と接続するエッジを追加します。

    例:
      ターゲットノードのラベルが "handle_invalid_admin ()" の場合、
      "handle_invalid_admin" という関数名と一致する関数ノードを探索し、
      その関数ノードへ "call_link" エッジで接続します。
    """
    # 関数ノードを label をキーとして辞書化（attributes が "function"）
    function_nodes = {}
    for node in token_graph.get("nodes", []):
        if node.get("attributes") == "function":
            function_nodes[node["label"]] = node["id"]

    new_edges = []
    # token_graph 内のすべてのエッジをチェック
    for edge in token_graph.get("edges", []):
        if edge.get("label") == "call":
            # CALLエッジのターゲットノードを取得
            target_node = next((n for n in token_graph.get("nodes", []) if n["id"] == edge["to"]), None)
            if target_node:
                # 関数呼び出し名を抽出（例："handle_invalid_admin ()" → "handle_invalid_admin"）
                func_name = target_node["label"].split("(")[0].strip()
                if func_name in function_nodes:
                    new_edge = {
                        "from": target_node["id"],
                        "to": function_nodes[func_name],
                        "label": "call_link"
                    }
                    new_edges.append(new_edge)
    token_graph["edges"].extend(new_edges)

def main():
    if len(sys.argv) < 4:
        print("Usage: python joint_graph_generator.py <cfg.json> <joint_graph.json> <output.json>")
        sys.exit(1)

    cfg_path = sys.argv[1]
    joint_graph_path = sys.argv[2]
    output_path = sys.argv[3]

    # CFGファイルの読み込み（IF文の分岐情報を含む）
    cfg_data = load_json(cfg_path)
    # Jointグラフファイルの読み込み
    joint_graph = load_json(joint_graph_path)

    # IF文の分岐強化を実施（IF ノードに属するサブノードのグループ情報も付与）
    new_nodes, new_edges = enhance_cfg_if_structure(cfg_data, start_new_id=100)
    # JointグラフにIF文強化で生成したノード・エッジを統合
    enhanced_graph = merge_with_joint_graph(joint_graph, new_nodes, new_edges)

    # CALLエッジを検知して、関数呼び出し関係を追加
    process_call_edges(enhanced_graph)

    # グラフ内のすべてのノードに "attributes" キーが存在することを保証
    ensure_all_nodes_have_attributes(enhanced_graph, default_attr="")

    # 結果を出力ファイルに保存
    save_json(enhanced_graph, output_path)
    print(f"Updated joint graph saved to {output_path}")

if __name__ == "__main__":
    main()

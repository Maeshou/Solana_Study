import json
import networkx as nx
import matplotlib
matplotlib.use('Agg')  # バックエンドを 'Agg' に設定
import matplotlib.pyplot as plt

def visualize_cfg(cfg_data):
    graph = nx.DiGraph()

    for function in cfg_data:
        func_name = function["name"]
        print(f"[INFO] Processing function: {func_name}")

        node_mapping = {}  # ノードIDとグラフノードの対応を保持

        # ノードの追加
        for node in function["nodes"]:
            node_id = f"{func_name}_node_{node['id']}"
            node_label = node["label"]
            graph.add_node(node_id, label=node_label)
            node_mapping[node["id"]] = node_id

        # エッジの追加
        for edge in function["edges"]:
            from_id = node_mapping[edge["from"]]
            to_id = node_mapping[edge["to"]]
            graph.add_edge(from_id, to_id)

    # グラフの描画
    pos = nx.spring_layout(graph)
    labels = nx.get_node_attributes(graph, "label")

    plt.figure(figsize=(12, 8))
    nx.draw(graph, pos, with_labels=True, labels=labels, node_size=2000, node_color="lightgreen",
            font_size=8, font_weight="bold", arrowsize=15)
    plt.title("Control Flow Graph (CFG)")
    plt.savefig("cfg_visualization.png")
    print("[INFO] CFG visualization saved as 'cfg_visualization.png'")
    # plt.show()  # WSL環境ではコメントアウト

def main():
    # CFGデータの読み込み
    input_file = "cfg.json"
    print(f"[INFO] Loading CFG data from {input_file}...")
    try:
        with open(input_file, "r") as f:
            cfg_data = json.load(f)
            print("[INFO] Successfully loaded CFG data.")
    except FileNotFoundError:
        print(f"[ERROR] File {input_file} not found.")
        return

    # CFGの視覚化
    visualize_cfg(cfg_data)

if __name__ == "__main__":
    main()

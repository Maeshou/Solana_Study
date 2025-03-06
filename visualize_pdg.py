import json
import networkx as nx
import matplotlib
matplotlib.use('Agg')  # 必要に応じてバックエンドを設定
import matplotlib.pyplot as plt

def visualize_pdg(pdg_data):
    graph = nx.DiGraph()

    for function in pdg_data:
        func_name = function["name"]
        print(f"[INFO] Processing function: {func_name}")

        node_mapping = {}

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
            edge_label = edge["label"]
            graph.add_edge(from_id, to_id, label=edge_label)

    # 中心性を計算
    degree_centrality = nx.degree_centrality(graph)
    
    pos = nx.kamada_kawai_layout(graph)  # グラフ構造を考慮したレイアウト

    # レイアウトの計算
    #pos = nx.spring_layout(graph, k=2.5, iterations=50)

    # ノードサイズと色を調整
    node_sizes = [v * 3000 for v in degree_centrality.values()]
    node_colors = [v for v in degree_centrality.values()]

    # グラフの描画
    plt.figure(figsize=(16, 12))
    nx.draw(graph, pos, with_labels=True, labels=nx.get_node_attributes(graph, "label"),
            node_size=node_sizes, node_color=node_colors, font_size=10, font_weight="bold",
            arrowsize=15, cmap=plt.cm.Blues)
    nx.draw_networkx_edge_labels(graph, pos, edge_labels=nx.get_edge_attributes(graph, "label"), font_size=8)
    plt.title("Program Dependency Graph (PDG) - Improved Layout")
    plt.savefig("pdg_visualization_centralized.png")
    print("[INFO] PDG visualization saved as 'pdg_visualization_centralized.png'")

def main():
    input_file = "pdg.json"
    print(f"[INFO] Loading PDG data from {input_file}...")
    try:
        with open(input_file, "r") as f:
            pdg_data = json.load(f)
            print("[INFO] Successfully loaded PDG data.")
    except FileNotFoundError:
        print(f"[ERROR] File {input_file} not found.")
        return

    visualize_pdg(pdg_data)

if __name__ == "__main__":
    main()

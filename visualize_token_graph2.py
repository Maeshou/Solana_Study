import json
import networkx as nx
import matplotlib.pyplot as plt
import sys

def load_json(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        return json.load(f)

def compute_custom_layout(G):
    """
    サイクルを含むグラフでも利用できるよう、まず
    強連結成分を Condensation（DAG）として求め、各成分のレベルを計算します。
    その後、各元ノードのレベルは所属する成分のレベルとなり、
    同一レベル内ではID昇順に並べ、左右に配置します。
    """
    # Condensation を作成（各ノードは強連結成分のIDに写像される）
    C = nx.condensation(G)
    comp_mapping = C.graph['mapping']  # 元ノード -> 成分IDの辞書

    # DAG C で各成分のレベルを計算
    comp_levels = {}
    for comp in nx.topological_sort(C):
        if C.in_degree(comp) == 0:
            comp_levels[comp] = 0
        else:
            comp_levels[comp] = max(comp_levels[p] for p in C.predecessors(comp)) + 1

    # 各元ノードにレベルを割り当てる（所属する成分のレベル）
    node_levels = {}
    for node in G.nodes():
        comp_id = comp_mapping[node]
        node_levels[node] = comp_levels[comp_id]

    # 同一レベルごとにノードをグループ化し、ID昇順にソート
    level_nodes = {}
    for node, lvl in node_levels.items():
        level_nodes.setdefault(lvl, []).append(node)
    for lvl in level_nodes:
        level_nodes[lvl].sort()

    # 座標を計算（同一レベルは同じ y、左右は均等に配置）
    pos = {}
    horizontal_spacing = 200
    vertical_spacing = 150
    for lvl, nodes in level_nodes.items():
        k = len(nodes)
        for i, node in enumerate(nodes):
            # 中央寄せのための x 座標計算
            x = (i - (k - 1) / 2) * horizontal_spacing
            y = -lvl * vertical_spacing
            pos[node] = (x, y)
    return pos

def visualize_token_graph(json_file, output_file="token_graph2.png"):
    # JSON ファイルの読み込み
    data = load_json(json_file)
    
    # 有向グラフを生成
    G = nx.DiGraph()
    
    # ノードを追加（ラベルと属性を結合して表示）
    for node in data.get("nodes", []):
        node_id = node["id"]
        label = node["label"]
        attr = node["attributes"]
        combined_label = f"{label}\n({attr})"
        G.add_node(node_id, label=combined_label)
    
    # エッジを追加
    for edge in data.get("edges", []):
        source = edge["source"]
        target = edge["target"]
        edge_label = edge["label"]
        G.add_edge(source, target, label=edge_label)
    
    # カスタムレイアウトの計算（ノードは固定、同一レベルは横並び）
    pos = compute_custom_layout(G)
    
    # 描画領域の設定
    plt.figure(figsize=(16, 12))
    
    node_labels = nx.get_node_attributes(G, 'label')
    nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=500)
    nx.draw_networkx_labels(G, pos, labels=node_labels, font_size=8)
    
    nx.draw_networkx_edges(G, pos, arrows=True)
    edge_labels = nx.get_edge_attributes(G, 'label')
    nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, font_color='red', font_size=8)
    
    plt.title("Token Graph Visualization (Custom Layout)")
    plt.axis('off')
    
    # 画像を保存
    plt.savefig(output_file, format="png", dpi=300)
    print(f"Graph saved as {output_file}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python visualize_graph.py <token_graph.json> [output.png]")
        sys.exit(1)
    
    json_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else "updated_token_graph.png"
    visualize_token_graph(json_file, output_file)

if __name__ == '__main__':
    main()

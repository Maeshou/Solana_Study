import json
import networkx as nx
import matplotlib.pyplot as plt
import sys

def visualize_token_graph(json_file, output_file="token_graph2.png"):
    # JSON ファイルを読み込む
    with open(json_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    # 有向グラフを生成
    G = nx.DiGraph()
    
    # ノードを追加
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
    
    # Graphviz の dot レイアウトを使用
    try:
        pos = nx.nx_pydot.graphviz_layout(G, prog="dot")
    except ImportError:
        print("Graphviz layout requires pydot. Please install it: pip install pydot")
        sys.exit(1)

    # 図のサイズを調整
    plt.figure(figsize=(16, 12))

    # ノード描画
    node_labels = nx.get_node_attributes(G, 'label')
    nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=500)
    nx.draw_networkx_labels(G, pos, labels=node_labels, font_size=8)

    # エッジ描画
    nx.draw_networkx_edges(G, pos, arrows=True)
    edge_labels = nx.get_edge_attributes(G, 'label')
    nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, font_color='red', font_size=8)

    # タイトルと出力
    plt.title("Token Graph Visualization (Hierarchical Layout)")
    plt.axis('off')

    # 画像を保存
    plt.savefig(output_file, format="png", dpi=300)
    print(f"Graph saved as {output_file}")

    # 表示（不要ならコメントアウト可）
    #plt.show()

def main():
    if len(sys.argv) < 2:
        print("Usage: python visualize_graph.py <token_graph.json> [output.png]")
        sys.exit(1)
    
    json_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else "updated_token_graph.png"
    visualize_token_graph(json_file, output_file)

if __name__ == '__main__':
    main()
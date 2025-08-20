#!/usr/bin/env python3
import json
import networkx as nx
import matplotlib.pyplot as plt
import sys

def visualize_token_graph(json_file, output_file="token_graph_layout.png"):
    # JSON 読み込み
    with open(json_file, 'r', encoding='utf-8') as f:
        data = json.load(f)

    G = nx.DiGraph()
    # ノード追加
    for node in data.get("nodes", []):
        nid = node.get("id")
        lbl = f"{node.get('label','')}\n({node.get('attributes','')})"
        G.add_node(nid, label=lbl)
    # エッジ追加
    for e in data.get("edges", []):
        G.add_edge(e.get("source"), e.get("target"), label=e.get("label",""))

    # Graphviz レイアウト属性
    G.graph.update(
        nodesep="2.0",
        ranksep="2.0",
        overlap="false",
        splines="true",
        sep="+5.0,+5.0"
    )

    # レイアウト取得: pygraphviz (nx_agraph) を優先
    try:
        pos = nx.nx_agraph.graphviz_layout(G, prog="sfdp")
    except (ImportError, nx.NetworkXException):
        try:
            pos = nx.nx_agraph.graphviz_layout(G, prog="sfdp", args="-Goverlap=false -Gsep=1.0")
        except Exception:
            print("pygraphviz が必要です。pip install pygraphviz を試してください。")
            sys.exit(1)

    # y 座標を id 順に調整 (小さい id を上に)
    max_id = max(G.nodes()) if G.nodes() else 0
    for n, (x, _) in pos.items():
        new_y = max_id - n
        pos[n] = (x, new_y)

    # 描画
    plt.figure(figsize=(100, 100))
    node_labels = nx.get_node_attributes(G, 'label')
    nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=1200)
    nx.draw_networkx_labels(G, pos, labels=node_labels, font_size=8)
    nx.draw_networkx_edges(G, pos, arrows=True, arrowstyle='-|>', arrowsize=10)
    edge_labels = nx.get_edge_attributes(G, 'label')
    nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, font_color='red', font_size=6)

    plt.title("Token Graph Visualization (Hierarchical Layout)")
    plt.axis('off')
    plt.tight_layout()
    plt.savefig(output_file, dpi=200)
    print(f"Graph saved as {output_file}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python visualize_token_graph.py <token_graph.json> [output.png]")
        sys.exit(1)
    jf = sys.argv[1]
    of = sys.argv[2] if len(sys.argv) > 2 else "token_graph_layout.png"
    visualize_token_graph(jf, of)

#!/usr/bin/env python3
import json
import networkx as nx
import matplotlib.pyplot as plt
import sys

def quote_if_needed(text):
    """
    渡された text を文字列に変換し、コロン (:) が含まれている場合、
    両端が引用符で囲まれていなければ引用符を追加する。
    """
    text_str = str(text)
    if ':' in text_str and not (text_str.startswith('"') and text_str.endswith('"')):
        return f'"{text_str}"'
    return text_str

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
        # ノード名とラベルそれぞれに引用符処理を適用
        node_id_quoted = quote_if_needed(node_id)
        combined_label_quoted = quote_if_needed(combined_label)
        G.add_node(node_id_quoted, label=combined_label_quoted)
    
    # エッジを追加
    for edge in data.get("edges", []):
        source = edge["source"]
        target = edge["target"]
        edge_label = edge["label"]
        # ソース・ターゲット・ラベルに引用符処理を適用
        source_quoted = quote_if_needed(source)
        target_quoted = quote_if_needed(target)
        edge_label_quoted = quote_if_needed(edge_label)
        G.add_edge(source_quoted, target_quoted, label=edge_label_quoted)
    
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
    plt.savefig(output_file, format="png", dpi=300)
    print(f"Graph saved as {output_file}")
    # plt.show()  # 必要に応じて有効化してください

def main():
    if len(sys.argv) < 2:
        print("Usage: python visualize_graph2.py <token_graph.json> [output.png]")
        sys.exit(1)
    
    json_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else "updated_token_graph.png"
    visualize_token_graph(json_file, output_file)

if __name__ == '__main__':
    main()

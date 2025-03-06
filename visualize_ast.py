import json
import networkx as nx
import matplotlib
matplotlib.use('Agg')  # バックエンドを 'Agg' に設定
import matplotlib.pyplot as plt
import sys  # ここを追加

def parse_json_to_graph(data):
    """
    Parse the AST JSON data and convert it into a directed graph.
    """
    graph = nx.DiGraph()
    print("[INFO] Starting AST processing...")

    # 全ノード数を予測
    total_nodes = count_nodes(data)
    processed_nodes = 0  # 処理済みノードのカウンタ
    node_counter = 0  # ノードIDのカウンター

    def add_node_and_edges(parent_id, node_data, level=0):
        """
        Recursively add nodes and edges to the graph.
        """
        nonlocal processed_nodes, node_counter

        indent = "  " * level  # Indentation for progress display

        # ノードIDを一意に生成
        node_counter += 1
        node_id = f"node_{node_counter}"

        # `node_type` を取得し、ない場合は `unknown` にする
        node_type = node_data.get("node_type", "unknown")

        # ノードをグラフに追加
        label = f"{node_data.get('name', 'unknown')} ({node_type})"
        graph.add_node(node_id, label=label)

        # エッジ追加
        if parent_id:
            graph.add_edge(parent_id, node_id)

        # ノード処理数を更新し進捗を表示
        processed_nodes += 1
        progress = (processed_nodes / total_nodes) * 100
        print(f"{indent}[INFO] Added node: {label} ({progress:.2f}%)")

        # 子要素の処理
        if node_type == "function":
            # 入力パラメータの処理
            for input_param in node_data.get("inputs", []):
                node_counter += 1
                input_id = f"node_{node_counter}"
                graph.add_node(input_id, label=f"Param: {input_param}")
                graph.add_edge(node_id, input_id)
                processed_nodes += 1
                print(f"{indent}[INFO] Added input node: {input_param} ({progress:.2f}%)")
            # ボディのステートメントの処理
            previous_node_id = node_id
            for stmt in node_data.get("body", []):
                node_counter += 1
                stmt_id = f"node_{node_counter}"
                graph.add_node(stmt_id, label=stmt)
                graph.add_edge(previous_node_id, stmt_id)
                previous_node_id = stmt_id
                processed_nodes += 1
                print(f"{indent}[INFO] Added statement node: {stmt} ({progress:.2f}%)")

        elif node_type == "struct":
            for field in node_data.get("fields", []):
                node_counter += 1
                field_id = f"node_{node_counter}"
                field_label = f"{field['name']}: {field['field_type']}"
                graph.add_node(field_id, label=field_label)
                graph.add_edge(node_id, field_id)
                processed_nodes += 1
                print(f"{indent}[INFO] Added field node: {field_label} ({progress:.2f}%)")

    # JSONデータを再帰的に処理
    for item in data:
        add_node_and_edges(None, item)

    print("[INFO] AST processing complete.")
    return graph

def count_nodes(data):
    """
    Count the total number of nodes in the JSON data for progress calculation.
    """
    count = 0

    def count_recursive(node):
        nonlocal count
        count += 1  # 現在のノードをカウント
        node_type = node.get("node_type", "unknown")
        if node_type == "function":
            count += len(node.get("inputs", []))
            count += len(node.get("body", []))
        elif node_type == "struct":
            count += len(node.get("fields", []))

    for item in data:
        count_recursive(item)
    return count

def visualize_graph(graph, output_file):
    """
    Visualize the directed graph using matplotlib and save as PNG.
    親ノードが上、子ノードが下になる階層レイアウト（dot レイアウト）を利用。
    """
    print("[INFO] Generating graph visualization...")
    try:
        pos = nx.nx_pydot.graphviz_layout(graph, prog="dot")
    except ImportError:
        print("Graphviz layout requires pydot. Please install it: pip install pydot")
        sys.exit(1)

    labels = nx.get_node_attributes(graph, "label")

    plt.figure(figsize=(36, 12))
    nx.draw(graph, pos, with_labels=True, labels=labels, node_size=2000, node_color="skyblue",
            font_size=8, font_weight="bold", arrowsize=15)
    plt.title("AST Visualization (Hierarchical Layout)")
    plt.axis('off')
    plt.savefig(output_file, format="png", dpi=300)
    print(f"[INFO] Graph visualization saved as '{output_file}'")
    plt.show()

def main():
    if len(sys.argv) < 3:
        print("Usage: python visualize_ast.py <input_json> <output_png>")
        return

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    print(f"[INFO] Loading JSON data from {input_file}...")
    try:
        with open(input_file, "r") as f:
            data = json.load(f)
            print("[INFO] Successfully loaded JSON data.")
    except FileNotFoundError:
        print(f"[ERROR] File {input_file} not found.")
        return

    print("[INFO] Parsing JSON data into graph...")
    graph = parse_json_to_graph(data)

    visualize_graph(graph, output_file)

if __name__ == "__main__":
    main()

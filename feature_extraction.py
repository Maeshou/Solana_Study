import argparse
import json
import networkx as nx
from itertools import combinations
import numpy as np

# --- 1. JSONファイルからグラフを生成 ---
def build_graph_from_file(file_path):
    """
    指定されたファイルパスからJSONデータを読み込み、グラフ（DiGraph）を構築する。
    ノードは 'label' と 'attributes' を属性として、エッジは 'label' を属性として保持する。
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    G = nx.DiGraph()
    for node in data['nodes']:
        G.add_node(node['id'], label=node['label'], attributes=node['attributes'])
    for edge in data['edges']:
        G.add_edge(edge['source'], edge['target'], label=edge['label'])
    return G

# --- 2. 汎用的なグラフレット特徴の抽出 ---
def extract_generic_graphlet_features(G, k=3):
    """
    グラフGからkノードの部分グラフ（グラフレット）を抽出し、
      ・部分グラフのエッジ数 ("g_edges_<数>")
      ・エッジラベル ("g_edge_label_<ラベル>")
      ・ノードラベル ("g_node_label_<ラベル>")
      ・ノード属性 ("g_node_attr_<属性>")
    をカウントして特徴として反映する。
    """
    count_dict = {}
    for nodes in combinations(G.nodes(), k):
        subG = G.subgraph(nodes)
        # エッジ数のカウント
        num_edges = subG.number_of_edges()
        key_edges = f"g_edges_{num_edges}"
        count_dict[key_edges] = count_dict.get(key_edges, 0) + 1
        # エッジラベルのカウント
        for u, v in subG.edges():
            lbl = subG.edges[u, v].get("label", "")
            key = f"g_edge_label_{lbl}"
            count_dict[key] = count_dict.get(key, 0) + 1
        # ノードラベルのカウント
        for node in subG.nodes():
            lbl = subG.nodes[node].get("label", "")
            key = f"g_node_label_{lbl}"
            count_dict[key] = count_dict.get(key, 0) + 1
        # ノード属性のカウント
        for node in subG.nodes():
            attr = subG.nodes[node].get("attributes", "")
            key = f"g_node_attr_{attr}"
            count_dict[key] = count_dict.get(key, 0) + 1
    return count_dict

# --- 3. ドメイン固有グループ特徴の抽出 ---
def extract_domain_group_features(G):
    """
    "next" エッジにより連なるグループ（コード行や関数単位と仮定）を抽出し、
      各グループ内でのノードラベル ("d_node_label_<ラベル>") と
      ノード属性 ("d_node_attr_<属性>") の出現回数をカウントする。
    """
    next_edges = [(u, v) for u, v, d in G.edges(data=True) if d.get("label") == "next"]
    next_map = {}
    for u, v in next_edges:
        next_map.setdefault(u, []).append(v)
    targets = {v for u, v in next_edges}
    start_nodes = [node for node in G.nodes() if node in next_map and node not in targets]
    
    groups = []
    for start in start_nodes:
        chain = [start]
        current = start
        while current in next_map and next_map[current]:
            next_node = next_map[current][0]
            chain.append(next_node)
            current = next_node
        groups.append(chain)
    
    features = {}
    for chain in groups:
        for node in chain:
            lbl = G.nodes[node].get("label", "")
            key_lbl = f"d_node_label_{lbl}"
            features[key_lbl] = features.get(key_lbl, 0) + 1
            attr = G.nodes[node].get("attributes", "")
            key_attr = f"d_node_attr_{attr}"
            features[key_attr] = features.get(key_attr, 0) + 1
    return features

# --- 4. ドメインパターン特徴の抽出 ---
def extract_domain_pattern_features(G):
    """
    グループ内（"next" エッジで連なるグループ）で、
    identifier属性ノードが「has_one」「ctx_link」「assign_link」「input_link」
    などのエッジを介して接続されるパターンの出現回数をカウントする。
    キーは "pattern_<エッジラベル>" の形式。
    """
    next_edges = [(u, v) for u, v, d in G.edges(data=True) if d.get("label") == "next"]
    next_map = {}
    for u, v in next_edges:
        next_map.setdefault(u, []).append(v)
    targets = {v for u, v in next_edges}
    start_nodes = [node for node in G.nodes() if node in next_map and node not in targets]
    
    groups = []
    for start in start_nodes:
        chain = [start]
        current = start
        while current in next_map and next_map[current]:
            next_node = next_map[current][0]
            chain.append(next_node)
            current = next_node
        groups.append(chain)
    
    pattern_edge_types = {"has_one", "ctx_link", "assign_link", "input_link"}
    pattern_features = {}
    for group in groups:
        group_set = set(group)
        for u, v, d in G.edges(data=True):
            if u in group_set and v in group_set:
                edge_lbl = d.get("label", "")
                if edge_lbl in pattern_edge_types:
                    # 少なくとも u または v が identifier 属性ならパターンとする
                    if (G.nodes[u].get("attributes") == "identifier" or 
                        G.nodes[v].get("attributes") == "identifier"):
                        key = f"pattern_{edge_lbl}"
                        pattern_features[key] = pattern_features.get(key, 0) + 1
    return pattern_features

# --- 5. 複数の特徴辞書を統合 ---
def combine_features(*dicts):
    """
    可変個数の特徴辞書を統合し、全キーに基づく固定長の特徴ベクトルとキーリストを返す。
    """
    combined = {}
    for d in dicts:
        for k, v in d.items():
            combined[k] = combined.get(k, 0) + v
    sorted_keys = sorted(combined.keys())
    feature_vector = np.array([combined[k] for k in sorted_keys])
    return feature_vector, sorted_keys

# --- 6. メイン処理 ---
if __name__ == "__main__":
    # コマンドライン引数で入力ファイルのパスを受け取る
    parser = argparse.ArgumentParser(description="グラフ特徴抽出プログラム")
    parser.add_argument("--input", type=str, required=True, help="jointグラフのJSONファイルパス")
    args = parser.parse_args()
    
    # 入力ファイルからグラフを構築
    G = build_graph_from_file(args.input)
    
    # 特徴抽出
    generic_features = extract_generic_graphlet_features(G, k=3)
    domain_group_features = extract_domain_group_features(G)
    domain_pattern_features = extract_domain_pattern_features(G)
    
    # 特徴の統合
    feature_vector, feature_keys = combine_features(generic_features, domain_group_features, domain_pattern_features)
    
    # 結果出力
    print("【汎用的グラフレット特徴】")
    print(generic_features)
    print("\n【ドメイン固有グループ特徴】")
    print(domain_group_features)
    print("\n【ドメインパターン特徴】")
    print(domain_pattern_features)
    print("\n【統合された特徴ベクトル】")
    print(feature_vector)
    print("【特徴キー】")
    print(feature_keys)

#!/usr/bin/env python3
"""
make_ft_vec.py
------------------------------------
複数プロジェクトのグラフをまとめて一度に処理し、ノード埋め込みベクトルを各入力グラフディレクトリ内に保存するスクリプト。

【設定セクション】
以下の変数を書き換えることで、入力グラフのルートディレクトリ一覧、FastTextモデル保存先、
埋め込み次元数やエポック数をスクリプト内で指定できます。
"""
import json
import os
import random
import glob
import numpy as np
from gensim.models import FastText
from tqdm import tqdm

# ==== 設定セクション ====
# 入力グラフのディレクトリ一覧を追加
INPUT_GRAPH_DIRS = [
    "./programs/Projects/new_6/dataset/graphs",
    "./programs/Projects/Project6/dataset/graphs",
    # その他のプロジェクトパスをここに追記
]
# 学習済 FastText モデルの保存パス
MODEL_PATH = "fasttext_all.bin"
# FastText 埋め込みの次元数
DIM = 64
# FastText 学習のエポック数
EPOCH = 20


def load_graph(path):
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def graph_to_sentences(g, n_walks=10, walk_len=6):
    id2node = {n['id']: n for n in g['nodes']}
    sentences = [[n['label'], n.get('attributes', '')] for n in g['nodes']]
    adj = {}
    for e in g['edges']:
        adj.setdefault(e['source'], []).append(e['target'])
    for v in id2node:
        for _ in range(n_walks):
            cur, walk = v, []
            for _ in range(walk_len):
                walk.append(id2node[cur]['label'])
                if cur not in adj:
                    break
                cur = random.choice(adj[cur])  # ランダムウォーク
            sentences.append(walk)
    return sentences


def train_fasttext(sentences, dim, epoch):
    return FastText(
        vector_size=dim,
        window=5,
        min_count=1,
        sg=1,
        epochs=epoch,
        sentences=sentences
    )


def save_node_vectors(g, model, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    vecs = []
    for n in g['nodes']:
        toks = [n['label'], n.get('attributes', '')]
        token_vecs = [model.wv[t] for t in toks if t in model.wv]
        if token_vecs:
            vec = np.mean(token_vecs, axis=0)
        else:
            # モデルに 'unk' がない場合はゼロベクトル
            vec = model.wv['unk'] if 'unk' in model.wv else np.zeros(model.vector_size)
        vecs.append(vec)
    out_path = os.path.join(output_dir, 'vectors.npy')
    np.save(out_path, np.vstack(vecs))
    print(f"Saved node vectors to {out_path}")


def main():
    # 1) third_joint_graph.json をまとめて集める
    files = []
    for gd in INPUT_GRAPH_DIRS:
        pattern = os.path.join(gd, 'case_*', 'third_joint_graph.json')
        files += glob.glob(pattern)
    files = sorted(files)
    if not files:
        print(f"⚠️  ファイルが見つかりません: {INPUT_GRAPH_DIRS}")
        return

    # 2) コーパス生成
    print("📚 Generating corpus …")
    sentences = []
    for p in tqdm(files):
        g = load_graph(p)
        sentences += graph_to_sentences(g)
    sentences.append(["unk"])  # unk トークン

    # 3) FastText 学習
    print("🚀 Training FastText …")
    ft = train_fasttext(sentences, DIM, EPOCH)
    ft.save(MODEL_PATH)
    print(f"✅ model saved to {MODEL_PATH}")

    # 4) 各グラフごとにベクトル保存（入力ディレクトリと同じ場所）
    print("💾 Saving node vectors …")
    for p in tqdm(files):
        g = load_graph(p)
        # 入力パスの case ディレクトリをそのまま出力先に使用
        output_dir = os.path.dirname(p)
        save_node_vectors(g, ft, output_dir)

    print("✅ All done.")

if __name__ == "__main__":
    main()

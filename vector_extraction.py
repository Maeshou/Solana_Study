#!/usr/bin/env python3
"""
make_ft_vec.py
------------------------------------
• graphs 配下の各 case_* ディレクトリ内の third_joint_graph.json を一括処理
  - コーパス（トークン列）作成
  - gensim.FastText 学習
  - ノードベクトルを .npy で保存（dataset/graphs/<graph_name>/vectors.npy）

使い方:
    python make_ft_vec.py \
        --graphdir ./graphs \
        --model fasttext_solana.bin \
        --dim 128 --epoch 20
"""
import argparse
import json
import os
import random
import glob

import numpy as np
from gensim.models import FastText
from tqdm import tqdm


def load_graph(path):
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def graph_to_sentences(g, n_walks=10, walk_len=6):
    id2node = {n['id']: n for n in g['nodes']}
    sents = [[n['label'], n.get('attributes', '')] for n in g['nodes']]
    adj = {}
    for e in g['edges']:
        adj.setdefault(e['source'], []).append(e['target'])
    for v in id2node:
        for _ in range(n_walks):
            cur, walk = v, []
            for _ in range(walk_len):
                walk.append(id2node[cur]['label'])
                if cur not in adj: break
                cur = random.choice(adj[cur])
            sents.append(walk)
    return sents


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


def save_labels(output_dir,label):
    os.makedirs(output_dir,exist_ok=True)
    out_path = os.path.join(output_dir, 'label.json')
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(label, f)
    print(f"Save graph label to {out_path}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--graphdir",
        default="./graphs",
        help="case_* ディレクトリを含む graphs ルートパス"
    )
    ap.add_argument(
        "--model",
        default="fasttext_solana.bin",
        help="学習済 FastText モデルの保存パス"
    )
    ap.add_argument(
        "--vecdir",
        default="./dataset/graphs",
        help="出力ベクトルのルートディレクトリ"
    )
    ap.add_argument("--dim",   type=int, default=64, help="FastText の次元")
    ap.add_argument("--epoch", type=int, default=20,  help="FastText の epoch")
    ap.add_argument("--label", type=list, default=[0,0,0,0,1,0,0,0,0,0],  help="sample data の ラベル")
    args = ap.parse_args()

    # 1) third_joint_graph.json をまとめて集める
    pattern = os.path.join(args.graphdir, 'case_*', 'third_joint_graph.json')
    
    files = sorted(glob.glob(pattern))
    if not files:
        print(f"⚠️  ファイルが見つかりません: {pattern}")
        return

    # 2) コーパス生成
    print("📚 Generating corpus …")
    sentences = []
    for p in tqdm(files):
        g = load_graph(p)
        sentences += graph_to_sentences(g)
    # unk トークン
    sentences.append(["unk"])

    # 3) FastText 学習
    print("🚀 Training FastText …")
    ft = train_fasttext(sentences, args.dim, args.epoch)
    ft.save(args.model)
    print(f"✅ model saved to {args.model}")

    # 4) 各グラフごとにベクトル保存
    print("💾 Saving node vectors …")
    for p in tqdm(files):
        g      = load_graph(p)
        case_name = os.path.basename(os.path.dirname(p))
        outdir = os.path.join(args.vecdir, case_name)
        save_node_vectors(g, ft, outdir)
        save_labels(outdir,args.label)


    print("✅ All done.")


if __name__ == "__main__":
    main()

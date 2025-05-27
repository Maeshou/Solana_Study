import os
import glob
import json
import argparse
import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
import dgl
from dgl.nn import GatedGraphConv
from torch.utils.data import Dataset, DataLoader, random_split, WeightedRandomSampler
from sklearn.metrics import classification_report

# ─────────────────────────────────────────────────────────────────────────────
# 1) Dataset 準備：graph json から ホモグラフ + ノード特徴 + multi-hot ラベルを返す
#    ・全グラフで同じメタグラフ（all_etypes）を使用
#    ・ホモグラフ化してバッチ可能に
# ─────────────────────────────────────────────────────────────────────────────
class GraphDataset(Dataset):
    def __init__(self, items, all_etypes):
        """
        items: [(folder_path, label_vector), ...] のリスト
        all_etypes: 全グラフで共通のエッジラベル一覧 (sorted list)
        """
        self.items = items
        self.all_etypes = all_etypes

    def __len__(self):
        return len(self.items)

    def __getitem__(self, idx):
        folder, label_vec = self.items[idx]
        
        # --- JSON 読み込み ---
        g_json = json.load(open(os.path.join(folder, "third_joint_graph.json")))

        # --- ノード数を len(nodes) に設定 ---
        num_nodes = len(g_json["nodes"])

        # --- 全エッジラベルでメタグラフを初期化 ---
        data_dict = {}
        for et in self.all_etypes:
            data_dict[("node", et, "node")] = ([], [])

        # --- JSON のエッジを追加 ---
        for e in g_json["edges"]:
            tup = ("node", e["label"], "node")
            srcs, dsts = data_dict[tup]
            srcs.append(e["source"] - 1)
            dsts.append(e["target"] - 1)

        # --- ヘテログラフ生成 ---
        g = dgl.heterograph(data_dict, num_nodes_dict={"node": num_nodes})
        
        #print(f"hetero_graph = {g}")
        # --- エッジタイプID (_TYPE) を各エッジに設定 ---
        etype2id = {et: i for i, et in enumerate(self.all_etypes)}
        for rel in g.canonical_etypes:
            label = rel[1]
            n_edges = g.num_edges(rel)
            g.edges[rel].data['_TYPE'] = torch.full((n_edges,), etype2id[label], dtype=torch.int64)

        # --- ホモグラフに変換 & エッジ属性を継承 ---
        g = dgl.to_homogeneous(g, edata=['_TYPE'])
        
        #print(f"homo_graph = {g}")
        # --- ノード特徴量読み込み ---
        feats = np.load(os.path.join(folder, "vectors.npy"))
        feats = torch.from_numpy(feats).float()

        # --- ラベルベクトル ---
        y = torch.tensor(label_vec, dtype=torch.float32)
        return g, feats, y

# ─────────────────────────────────────────────────────────────────────────────
# 2) モデル：GatedGraphConv + Readout + Multi-label 分類ヘッド
# ─────────────────────────────────────────────────────────────────────────────
class GGNNClassifier(nn.Module):
    def __init__(self, in_dim, hid_dim, n_steps, n_etypes, num_labels):
        super().__init__()
        self.linear_in = nn.Linear(in_dim, hid_dim)
        self.ggnn = GatedGraphConv(in_feats=hid_dim,
                                   out_feats=hid_dim,
                                   n_steps=n_steps,
                                   n_etypes=n_etypes)
        self.classify = nn.Sequential(
            nn.Linear(hid_dim, hid_dim // 2),
            nn.ReLU(),
            nn.Linear(hid_dim // 2, num_labels)
        )

    def forward(self, g, h):
        h = self.linear_in(h)  # (総ノード数, hid_dim)
        # Dataset でホモグラフ化済み
        g_homo = g
        #print(f"g={g}")
        etype = g_homo.edata['_TYPE']
        #print(f"etype = {etype}")
        # メッセージパッシング
        h = self.ggnn(g_homo, h, etype)
        # Readout (平均プーリング)
        with g_homo.local_scope():
            g_homo.ndata['h'] = h
            hg = dgl.mean_nodes(g_homo, 'h')
        return self.classify(hg)

# ─────────────────────────────────────────────────────────────────────────────
# 3) collate_fn
# ─────────────────────────────────────────────────────────────────────────────
def collate_fn(batch):
    gs, fs, ys = map(list, zip(*batch))
    bg = dgl.batch(gs)
    hf = torch.cat(fs, dim=0)
    ly = torch.stack(ys)
    return bg, hf, ly

# ─────────────────────────────────────────────────────────────────────────────
# 4) 評価関数
# ─────────────────────────────────────────────────────────────────────────────
@torch.no_grad()
def evaluate(model, loader, device):
    model.eval()
    all_preds, all_labels = [], []
    for bg, hf, ly in loader:
        bg, hf = bg.to(device), hf.to(device)
        logits = model(bg, hf)
        probs = torch.sigmoid(logits).cpu().numpy()
        preds = (probs >= 0.5).astype(int)
        all_preds.append(preds)
        all_labels.append(ly.numpy())
    y_pred = np.vstack(all_preds)
    y_true = np.vstack(all_labels)
    report = classification_report(
        y_true, y_pred, output_dict=False, zero_division=0
    )
    print("=== Test set performance ===")
    print(report)

# ─────────────────────────────────────────────────────────────────────────────
# 5) 学習＋評価ループ
# ─────────────────────────────────────────────────────────────────────────────
def train_and_evaluate():
    ap = argparse.ArgumentParser()
    ap.add_argument("--project", default="./programs/Projects")
    args = ap.parse_args()

    # --- 生ラベル読み込み ---
    raw_items = []
    for proj in os.listdir(args.project):
        graph_dir = os.path.join(args.project, proj, "dataset", "graphs")
        for folder in sorted(glob.glob(os.path.join(graph_dir, 'case_*'))):
            raw_label = json.load(open(os.path.join(folder, 'label.json')))
            raw_items.append((folder, raw_label))

    # --- all_items 作成 ---
    all_items = []
    for folder, raw_label in raw_items:
        if isinstance(raw_label, list) and raw_label and all(x in (0,1) for x in raw_label):
            vec = raw_label
        else:
            raise ValueError("ラベルのフォーマットが違います")
        all_items.append((folder, vec))
    num_labels = len(all_items[0][1])
    print(f"num_labels = {num_labels}")   # → 10 になります

    # --- 全エッジラベルを先に集める ---
    all_etypes = set()
    for folder, _ in all_items:
        g_json = json.load(open(os.path.join(folder, 'third_joint_graph.json')))
        all_etypes |= {e['label'] for e in g_json['edges']}
    all_etypes = sorted(all_etypes)

       # --- Dataset, train/test split -----------------------------------
    ds = GraphDataset(all_items, all_etypes)
    n_train = int(len(ds) * 0.8)
    n_test  = len(ds) - n_train
    train_ds, test_ds = random_split(ds, [n_train, n_test])

    # --- WeightedRandomSampler の準備（修正版） --------------------------
    # Subset のインデックスを取得
    train_indices = train_ds.indices  # Subset には .indices 属性があります

    # 訓練サブセット上の「主ラベル」を argmax で取得
    train_labels = []
    for orig_idx in train_indices:
        _, vec = all_items[orig_idx]
        train_labels.append(int(np.argmax(vec)))

    # 各クラスの出現数を訓練サブセット上でカウント
    class_counts = np.bincount(train_labels, minlength=num_labels)
    # 出現ゼロのクラスはサンプラーに含めない or 小さな値を入れる
    # ここでは 0-count クラスの分だけ無視して、weight=0 とする
    class_weights = np.zeros_like(class_counts, dtype=float)
    nonzero = class_counts > 0
    class_weights[nonzero] = 1.0 / class_counts[nonzero]

    # 各サンプルの重みリスト (長さ == len(train_ds))
    sample_weights = [ class_weights[label] for label in train_labels ]

    sampler = WeightedRandomSampler(
        weights=sample_weights,
        num_samples=len(sample_weights),
        replacement=True
    )

    # --- DataLoader -----------------------------------------------
    train_loader = DataLoader(
        train_ds,
        batch_size=32,
        sampler=sampler,
        collate_fn=collate_fn,
        drop_last=True
    )
    test_loader = DataLoader(
        test_ds,
        batch_size=32,
        shuffle=False,
        collate_fn=collate_fn
    )


    # --- モデル／最適化設定 ---
    in_dim = 64
    hid_dim = 128
    n_steps = 8
    n_etypes = len(all_etypes)
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')

    model = GGNNClassifier(in_dim, hid_dim, n_steps, n_etypes, num_labels).to(device)
    opt = optim.Adam(model.parameters(), lr=1e-3)
    loss_fn = nn.BCEWithLogitsLoss()

    # --- 学習ループ ---
    for epoch in range(1, 51):
        model.train()
        total_loss = 0.0
        for bg, hf, ly in train_loader:
            bg, hf, ly = bg.to(device), hf.to(device), ly.to(device)
            logits = model(bg, hf)
            loss = loss_fn(logits, ly)
            opt.zero_grad()
            loss.backward()
            opt.step()
            total_loss += loss.item() * ly.size(0)
        avg_loss = total_loss / n_train
        print(f"Epoch {epoch:02d} | Train Loss {avg_loss:.4f}")

        if epoch % 10 == 0:
            evaluate(model, test_loader, device)

if __name__ == "__main__":
    train_and_evaluate()
import os
import glob
import json
import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
import dgl
from dgl.nn import GatedGraphConv
from torch.utils.data import Dataset, DataLoader, random_split, WeightedRandomSampler
from sklearn.metrics import classification_report, precision_recall_fscore_support

# ─────────────────────────────────────────────────────────────────────────────
# 設定セクション：ここで対象プロジェクトを指定
# ─────────────────────────────────────────────────────────────────────────────
ROOT_DIR = "./programs/Projects"
TARGET_PROJECTS = [
    "new_6",
    "Project6"
]

def get_zero_edge_folders(projects, root=ROOT_DIR, mode='effective'):
    """
    mode:
      - 'effective' : ノード範囲内に入る“有効エッジ”が1本もないものを検出
      - 'raw'       : JSONの edges 配列が0本のものを検出
    戻り値: [(folder_path, reason), ...]
    """
    zero = []
    for proj in projects:
        graph_dir = os.path.join(root, proj, "dataset", "graphs")
        if not os.path.isdir(graph_dir):
            continue
        for folder in sorted(glob.glob(os.path.join(graph_dir, "case_*"))):
            try:
                g_json = json.load(open(os.path.join(folder, "third_joint_graph.json")))
            except Exception as e:
                zero.append((folder, f"json read error: {e}"))
                continue

            nodes = len(g_json.get("nodes", []))
            edges = g_json.get("edges", []) or []

            if mode == 'raw':
                if len(edges) == 0:
                    zero.append((folder, "raw: edges==0"))
                continue

            # mode == 'effective'
            has_valid = False
            for e in edges:
                s = e.get("source"); t = e.get("target")
                if s is None or t is None:
                    continue
                s -= 1; t -= 1
                if 0 <= s < nodes and 0 <= t < nodes:
                    has_valid = True
                    break
            if not has_valid:
                zero.append((folder, "effective: no in-range edges"))
    return zero


def abort_if_zero_edge(projects, root=ROOT_DIR, mode='effective'):
    zero = get_zero_edge_folders(projects, root, mode=mode)
    if zero:
        print("=== ZERO-EDGE GRAPHS DETECTED ===")
        for path, reason in zero:
            print(f"  {path} | {reason}")
        raise SystemExit("Zero-edge graphs found. Aborting.")

# ─────────────────────────────────────────────────────────────────────────────
# 1) Dataset 準備：graph json から ホモグラフ + ノード特徴 + multi-hot ラベルを返す
# ─────────────────────────────────────────────────────────────────────────────
class GraphDataset(Dataset):
    def __init__(self, items, all_etypes):
        """
        items: [(folder_path, label_vector), ...]
        all_etypes: 全グラフで共通のエッジラベル一覧 (sorted list)
        """
        self.items = items
        self.all_etypes = all_etypes

    def __len__(self):
        return len(self.items)

    def __getitem__(self, idx):
        folder, label_vec = self.items[idx]
        g_json = json.load(open(os.path.join(folder, "third_joint_graph.json")))
        num_nodes = len(g_json["nodes"])

        # heterograph 構築用の空データ構造準備
        data_dict = {("node", et, "node"): ([], []) for et in self.all_etypes}
        for e in g_json["edges"]:
            tup = ("node", e["label"], "node")
            src = e["source"] - 1
            dst = e["target"] - 1
            # 範囲チェック（0 ≤ idx < num_nodes）
            if not (0 <= src < num_nodes and 0 <= dst < num_nodes):
                continue
            srcs, dsts = data_dict[tup]
            srcs.append(src)
            dsts.append(dst)

        # ヘテログラフ作成
        g = dgl.heterograph(data_dict, num_nodes_dict={"node": num_nodes})

        # エッジタイプを整数ラベル化
        etype2id = {et: i for i, et in enumerate(self.all_etypes)}
        for rel in g.canonical_etypes:
            label = rel[1]
            n_edges = g.num_edges(rel)
            g.edges[rel].data['_TYPE'] = torch.full((n_edges,), etype2id[label], dtype=torch.int64)

        # homogeneous graph に変換
        g = dgl.to_homogeneous(g, edata=['_TYPE'])

        # ノード特徴ベクトル読み込み
        feats = np.load(os.path.join(folder, "vectors.npy"))
        feats = torch.from_numpy(feats).float()

        # 特徴量行数とノード数の整合性チェック
        if feats.shape[0] != g.number_of_nodes():
            raise ValueError(
                f"【不整合検出】フォルダ `{folder}` のノード数"
                f"({g.number_of_nodes()}) と特徴量行数"
                f"({feats.shape[0]}) が異なります。"
            )

        # ラベル（multi-hot ベクトル）
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
        h = self.linear_in(h)
        etype = g.edata['_TYPE']
        h = self.ggnn(g, h, etype)
        with g.local_scope():
            g.ndata['h'] = h
            hg = dgl.mean_nodes(g, 'h')
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
# 4) 評価関数 (Precision/Recall/F1, support, TP/FP/TN/FN を出力)
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

    print("=== Test set performance ===")
    print(classification_report(y_true, y_pred, zero_division=0))

    report_dict = classification_report(y_true, y_pred, output_dict=True, zero_division=0)
    print("\n=== Detailed metrics (dict) ===")
    print(json.dumps(report_dict, indent=2, ensure_ascii=False))

    precisions, recalls, f1s, supports = precision_recall_fscore_support(
        y_true, y_pred, zero_division=0
    )
    num_labels = y_true.shape[1]
    print("\n=== Manual per-label metrics ===")
    for i in range(num_labels):
        print(f"Label {i:2d} | precision: {precisions[i]:.4f} | recall: {recalls[i]:.4f} "
              f"| f1-score: {f1s[i]:.4f} | support: {supports[i]}")

    print("\n=== Confusion matrix components per label ===")
    for i in range(num_labels):
        tp = int(np.logical_and(y_true[:, i] == 1, y_pred[:, i] == 1).sum())
        fp = int(np.logical_and(y_true[:, i] == 0, y_pred[:, i] == 1).sum())
        tn = int(np.logical_and(y_true[:, i] == 0, y_pred[:, i] == 0).sum())
        fn = int(np.logical_and(y_true[:, i] == 1, y_pred[:, i] == 0).sum())
        print(f"Label {i:2d} | TP={tp:5d} | FP={fp:5d} | TN={tn:5d} | FN={fn:5d}")


# ─────────────────────────────────────────────────────────────────────────────
# 5) 学習＋評価ループ＋10エポックごとの予測一覧出力
# ─────────────────────────────────────────────────────────────────────────────
def train_and_evaluate():
    abort_if_zero_edge(TARGET_PROJECTS, ROOT_DIR, mode='effective')  # 必要なら 'raw' に変更

    # --- 生ラベル読み込み ---
    raw_items = []
    for proj in TARGET_PROJECTS:
        graph_dir = os.path.join(ROOT_DIR, proj, "dataset", "graphs")
        if not os.path.isdir(graph_dir):
            raise FileNotFoundError(f"{proj} のグラフディレクトリが見つかりません: {graph_dir}")
        for folder in sorted(glob.glob(os.path.join(graph_dir, 'case_*'))):
            raw_label = json.load(open(os.path.join(folder, 'label.json')))
            raw_items.append((folder, raw_label))

    # --- ラベル形式を統一 ---
    all_items = []
    for folder, raw_label in raw_items:
        if isinstance(raw_label, list):
            if not raw_label or not all(x in (0, 1) for x in raw_label):
                raise ValueError("リストラベルのフォーマットが違います")
            label_list = raw_label
        elif isinstance(raw_label, (int, float)):
            label_list = [raw_label]
        else:
            raise ValueError("ラベルのフォーマットが違います")
        all_items.append((folder, label_list))

    num_labels = len(all_items[0][1])

    # --- エッジ数 0 のグラフをリストアップ ---
    empty_graphs = []
    for folder, _ in all_items:
        g_json = json.load(open(os.path.join(folder, 'third_joint_graph.json')))
        if len(g_json.get("edges", [])) == 0:
            empty_graphs.append(folder)
    if empty_graphs:
        print("=== エッジ数 0 のグラフ一覧 ===")
        for path in empty_graphs:
            print("  ", path)
        print("※ 上記のケースは学習／評価時にスキップするかご確認ください\n")

    # --- 全エッジラベルを先に集める ---
    all_etypes = set()
    for folder, _ in all_items:
        g_json = json.load(open(os.path.join(folder, 'third_joint_graph.json')))
        all_etypes |= {e['label'] for e in g_json['edges']}
    all_etypes = sorted(all_etypes)
    print(f"all_etypes={all_etypes}")

    # --- Dataset／DataLoader 準備 ---
    ds = GraphDataset(all_items, all_etypes)
    n_train = int(len(ds) * 0.8)
    train_ds, test_ds = random_split(ds, [n_train, len(ds) - n_train])

    train_indices = train_ds.indices
    train_labels  = [int(np.argmax(all_items[idx][1])) for idx in train_indices]
    class_counts  = np.bincount(train_labels, minlength=num_labels)
    class_weights = np.zeros_like(class_counts, dtype=float)
    nonzero       = class_counts > 0
    class_weights[nonzero] = 1.0 / class_counts[nonzero]
    sampler = WeightedRandomSampler(
        weights=[class_weights[l] for l in train_labels],
        num_samples=len(train_labels),
        replacement=True
    )

    train_loader = DataLoader(train_ds, batch_size=32, sampler=sampler,
                              collate_fn=collate_fn, drop_last=True)
    test_loader  = DataLoader(test_ds,  batch_size=32, shuffle=False,
                              collate_fn=collate_fn)

    # --- モデル・最適化準備 ---
    in_dim, hid_dim, n_steps = 64, 128, 8
    device   = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    model    = GGNNClassifier(in_dim, hid_dim, n_steps, len(all_etypes), num_labels).to(device)
    opt      = optim.Adam(model.parameters(), lr=1e-3)
    loss_fn  = nn.BCEWithLogitsLoss()

    # --- 学習＆評価ループ ---
    for epoch in range(1, 51):
        model.train()
        total_loss = 0.0
        for bg, hf, ly in train_loader:
            bg, hf, ly = bg.to(device), hf.to(device), ly.to(device)
            logits = model(bg, hf)
            loss   = loss_fn(logits, ly)
            opt.zero_grad()
            loss.backward()
            opt.step()
            total_loss += loss.item() * ly.size(0)
        print(f"Epoch {epoch:02d} | Train Loss {total_loss / n_train:.4f}")

        if epoch % 10 == 0:
            evaluate(model, test_loader, device)
            print(f"\n=== Epoch {epoch:02d} Predictions ===")
            for idx in test_ds.indices:
                folder, true_label = all_items[idx]
                g, feats, _ = ds[idx]
                g, feats = g.to(device), feats.to(device)
                with torch.no_grad():
                    logits = model(g, feats)
                    probs = torch.sigmoid(logits).cpu().numpy()
                    pred = (probs >= 0.5).astype(int).tolist()
                print(f"{folder} | pred: {pred} | true: {true_label}")

    # --- 最終評価 ---
    evaluate(model, test_loader, device)


if __name__ == '__main__':
    train_and_evaluate()




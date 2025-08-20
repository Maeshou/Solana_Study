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
from torch.utils.data import Dataset, DataLoader, WeightedRandomSampler
from sklearn.metrics import classification_report, precision_recall_fscore_support

class GraphDataset(Dataset):
    def __init__(self, items, all_etypes):
        self.items = items
        self.all_etypes = all_etypes

    def __len__(self):
        return len(self.items)

    def __getitem__(self, idx):
        folder, label_vec = self.items[idx]
        g_json = json.load(open(os.path.join(folder, "third_joint_graph.json")))

        num_nodes = len(g_json["nodes"])
        data_dict = {("node", et, "node"): ([], []) for et in self.all_etypes}
        for e in g_json["edges"]:
            tup = ("node", e["label"], "node")
            srcs, dsts = data_dict[tup]
            srcs.append(e["source"] - 1)
            dsts.append(e["target"] - 1)

        g = dgl.heterograph(data_dict, num_nodes_dict={"node": num_nodes})
        etype2id = {et: i for i, et in enumerate(self.all_etypes)}
        for rel in g.canonical_etypes:
            label = rel[1]
            n_edges = g.num_edges(rel)
            g.edges[rel].data['_TYPE'] = torch.full((n_edges,), etype2id[label], dtype=torch.int64)
        g = dgl.to_homogeneous(g, edata=['_TYPE'])

        feats = torch.from_numpy(np.load(os.path.join(folder, "vectors.npy"))).float()
        y = torch.tensor(label_vec, dtype=torch.float32)
        return g, feats, y

class GGNNClassifier(nn.Module):
    def __init__(self, in_dim, hid_dim, n_steps, n_etypes, num_labels):
        super().__init__()
        self.linear_in = nn.Linear(in_dim, hid_dim)
        self.ggnn      = GatedGraphConv(in_feats=hid_dim, out_feats=hid_dim,
                                        n_steps=n_steps, n_etypes=n_etypes)
        self.classify = nn.Sequential(
            nn.Linear(hid_dim, hid_dim // 2),
            nn.ReLU(),
            nn.Linear(hid_dim // 2, num_labels)
        )

    def forward(self, g, h):
        h = self.linear_in(h)
        g.ndata['h'] = self.ggnn(g, h, g.edata['_TYPE'])
        return self.classify(dgl.mean_nodes(g, 'h'))

def collate_fn(batch):
    gs, fs, ys = map(list, zip(*batch))
    bg = dgl.batch(gs)
    hf = torch.cat(fs, dim=0)
    ly = torch.stack(ys)
    return bg, hf, ly

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
        print(f"Label {i:2d} | precision: {precisions[i]:.4f} "
              f"| recall: {recalls[i]:.4f} "
              f"| f1-score: {f1s[i]:.4f} "
              f"| support: {supports[i]}")

    print("\n=== Confusion matrix components per label ===")
    for i in range(num_labels):
        tp = int(np.logical_and(y_true[:, i] == 1, y_pred[:, i] == 1).sum())
        fp = int(np.logical_and(y_true[:, i] == 0, y_pred[:, i] == 1).sum())
        tn = int(np.logical_and(y_true[:, i] == 0, y_pred[:, i] == 0).sum())
        fn = int(np.logical_and(y_true[:, i] == 1, y_pred[:, i] == 0).sum())
        print(f"Label {i:2d} | TP={tp:5d} | FP={fp:5d} | TN={tn:5d} | FN={fn:5d}")

def train_and_evaluate():
    ap = argparse.ArgumentParser()
    ap.add_argument("--train_paths", nargs='+', default=[
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project0",
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project1",
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project2",
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project3",
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project4",
        "/home/shogo-maeda/Solana_Study/programs/Projects/Project5",
    ])
    ap.add_argument("--test_paths", nargs='+', default=[
        "/home/shogo-maeda/Solana_Study/programs/Projects/test_rev*",
    ])
    args = ap.parse_args()

    # --- 学習用プロジェクトの取得 ---
    train_dirs = []
    for path in args.train_paths:
        train_dirs += glob.glob(path)

    raw_items = []
    for proj in sorted(train_dirs):
        graph_dir = os.path.join(proj, "dataset", "graphs")
        for folder in sorted(glob.glob(os.path.join(graph_dir, 'case_*'))):
            raw_label = json.load(open(os.path.join(folder, 'label.json')))
            raw_items.append((folder, raw_label))

    train_items = []
    for folder, raw_label in raw_items:
        if isinstance(raw_label, list) and raw_label and all(x in (0,1) for x in raw_label):
            train_items.append((folder, raw_label))
        else:
            raise ValueError("ラベルのフォーマットが違います")
    num_labels = len(train_items[0][1])

    # --- テスト用プロジェクトの取得 ---
    test_dirs = []
    for path in args.test_paths:
        test_dirs += glob.glob(path)

    test_items = []
    for proj in sorted(test_dirs):
        graph_dir = os.path.join(proj, "dataset", "graphs")
        for folder in sorted(glob.glob(os.path.join(graph_dir, 'case_*'))):
            label_path = os.path.join(folder, 'label.json')
            if os.path.exists(label_path):
                raw_label = json.load(open(label_path))
                if isinstance(raw_label, list) and raw_label and all(x in (0,1) for x in raw_label):
                    test_items.append((folder, raw_label))
                else:
                    raise ValueError(f"{folder}: テストラベルの形式が不正です")

    # 全てのグラフフォルダ一覧を取得
    all_folders = [f for f,_ in train_items] + [f for f,_ in test_items]

    all_etypes = set()
    for folder in all_folders:
        g_json = json.load(open(os.path.join(folder, "third_joint_graph.json")))
        all_etypes |= {e['label'] for e in g_json['edges']}
    all_etypes = sorted(all_etypes)

    train_ds = GraphDataset(train_items, all_etypes)
    test_ds  = GraphDataset(test_items, all_etypes)

    train_loader = DataLoader(train_ds, batch_size=32, shuffle=True,
                              collate_fn=collate_fn, drop_last=True)
    test_loader  = DataLoader(test_ds,  batch_size=32, shuffle=True,
                              collate_fn=collate_fn)

    in_dim, hid_dim, n_steps, n_etypes = 64, 128, 8, len(all_etypes)
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    model  = GGNNClassifier(in_dim, hid_dim, n_steps, n_etypes, num_labels).to(device)
    opt    = optim.Adam(model.parameters(), lr=1e-3)
    loss_fn= nn.BCEWithLogitsLoss()

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
        print(f"Epoch {epoch:02d} | Train Loss {total_loss / len(train_ds):.4f}")

        if epoch % 10 == 0:
            evaluate(model, test_loader, device)
            print(f"\n=== Epoch {epoch:02d} Predictions ===")
            for i, (folder, true_label) in enumerate(test_items):
                g, feats, _ = test_ds[i]
                g, feats = g.to(device), feats.to(device)
                with torch.no_grad():
                    logits = model(g, feats)
                    probs  = torch.sigmoid(logits).cpu().numpy()
                    pred   = (probs >= 0.5).astype(int).tolist()
                print(f"{folder} | pred: {pred} | true: {true_label}")

if __name__ == "__main__":
    train_and_evaluate()

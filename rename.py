#!/usr/bin/env python3
import os
import re

# リネーム対象のディレクトリ
base_dir = "/home/shogo-maeda/Solana_Study/programs/Projects/new_8/dataset/source_codes"

# case_### もしくは case_###.ext の形式を判定する正規表現（拡張子あり／なしを含む）
pattern = re.compile(r"^case_\d{3}(?:\..+)?$")

# ディレクトリ内のエントリを取得（ファイルもフォルダも含む）→ソート
entries = os.listdir(base_dir)
entries.sort()

# 現在ディレクトリに存在する名前をセットとして保持しておく
used_names = set(entries)

for idx, name in enumerate(entries, start=1):
    old_path = os.path.join(base_dir, name)

    # 拡張子を取得（フォルダなら ext='' になる）
    stem, ext = os.path.splitext(name)

    # 基本となるインデックス
    temp_idx = idx

    # 衝突回避のため、新しい名前がユニーク（かつ自分自身ではない）になるまでループ
    while True:
        candidate = f"case_{temp_idx:03d}" + ext  # フォルダなら ext='' のまま、ファイルなら ext は ".rs"などを保持
        # 「候補が自分自身の名前であれば OK」、「used_names になければ OK」
        if candidate == name or candidate not in used_names:
            break
        temp_idx += 1

    # もし候補が自分自身の名前なら、すでに正しい名前 or 正しい番号が振られている状態 → スキップ
    if candidate == name:
        print(f"[SKIP] {name} はすでにユニーク／正しい形式です")
        continue

    # 新しいパスを作成
    new_path = os.path.join(base_dir, candidate)

    # リネーム実行
    print(f"Renaming {old_path!r} → {new_path!r}")
    os.rename(old_path, new_path)

    # used_names を更新
    used_names.remove(name)
    used_names.add(candidate)

print("=== リネーム処理 完了 ===")

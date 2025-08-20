#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import json

def update_numeric_labels(base_dir: str, new_value):
    """
    base_dir 以下の case_001 ～ case_199 フォルダ内にある
    label.json をすべて new_value（数値）で上書き or 新規作成する。
    """
    for i in range(1, 101):
        dirname = f"case_{i:03d}"
        case_dir = os.path.join(base_dir, dirname)
        # フォルダが存在しなければスキップ
        if not os.path.isdir(case_dir):
            print(f"ディレクトリが見つかりません、スキップ → {case_dir}")
            continue

        json_path = os.path.join(case_dir, "label.json")
        # 'w' モードならファイルがなければ自動で作ってくれる
        with open(json_path, 'w', encoding='utf-8') as f:
            json.dump(new_value, f)
        print(f"Updated → {json_path}")

# ───────────────────────────────────────────────────────
# ここで一括更新するディレクトリと新しい数値を指定
BASE_DIR = "/home/shogo-maeda/Solana_Study/programs/Projects/new_6/dataset/graphs"
NEW_LABEL_VALUE = 1    # 置き換えたい数値
# ───────────────────────────────────────────────────────

if __name__ == "__main__":
    update_numeric_labels(BASE_DIR, NEW_LABEL_VALUE)

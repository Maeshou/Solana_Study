#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import json
import glob

# 入力グラフのディレクトリ一覧
INPUT_GRAPH_DIRS = [
    "./programs/Projects/new_0/dataset/graphs",
    "./programs/Projects/new_1/dataset/graphs",
    # その他のプロジェクトパスをここに追記
]

def find_graphs_with_from():
    problematic = []
    for graph_root in INPUT_GRAPH_DIRS:
        # 各 case_* ディレクトリ内の third_joint_graph.json を対象
        pattern = os.path.join(graph_root, 'case_*', 'third_joint_graph.json')
        for filepath in glob.glob(pattern):
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                edges = data.get('edges', [])
                # edges の中に 'from' キーを含むエッジがあるかチェック
                if any(isinstance(e, dict) and 'from' in e for e in edges):
                    problematic.append(filepath)
            except Exception as e:
                print(f"⚠️ ファイル読み込み中にエラー ({filepath}): {e}")
    return problematic

def main():
    bad_files = find_graphs_with_from()
    if bad_files:
        print("以下のファイルに 'from' キーが含まれています:")
        for p in bad_files:
            print(f"  - {p}")
    else:
        print("⚙️ 問題のあるグラフデータは見つかりませんでした。")

if __name__ == "__main__":
    main()

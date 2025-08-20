#!/usr/bin/env python3
import os
import subprocess
import logging
import argparse

logging.basicConfig(level=logging.INFO, format='[%(levelname)s] %(message)s')

# auto.py のあるディレクトリ
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

# Projects ディレクトリへの相対パス
BASE_DIR = os.path.join(SCRIPT_DIR, "programs", "Projects")

# 各種ツールへの相対パス
#EXTRACT_AST    = os.path.join(SCRIPT_DIR, "programs/Projects/src", "extract_ast_1")
#EXTRACT_CFG    = os.path.join(SCRIPT_DIR, "programs/Projects/src", "extract_cfg.rs")
#EXTRACT_PDG    = os.path.join(SCRIPT_DIR, "programs/Projects/src", "extract_pdg.rs")
TOKENIZE_AST   = os.path.join(SCRIPT_DIR, "tokenize_ast2.py")#change to tokenize_ast 8.1
JOINT1         = os.path.join(SCRIPT_DIR, "joint_graph_generater2.py")
JOINT2         = os.path.join(SCRIPT_DIR, "second_joint_graph_generater.py")
JOINT3         = os.path.join(SCRIPT_DIR, "third_joint_graph_generater.py")#change to third_joint_graph_generater_copy.py

def process_file(input_path, graph_dir):
    """1ファイル分の一連処理を実行"""
    name = os.path.splitext(os.path.basename(input_path))[0]

    # 出力ファイルパス
    ast_json   = os.path.join(graph_dir, "ast.json")
    cfg_json   = os.path.join(graph_dir, "cfg.json")
    pdg_json   = os.path.join(graph_dir, "pdg.json")
    token_json = os.path.join(graph_dir, "token.json")
    joint1     = os.path.join(graph_dir, "first_joint_graph.json")
    joint2     = os.path.join(graph_dir, "second_joint_graph.json")
    joint3     = os.path.join(graph_dir, "third_joint_graph.json")

    steps = [
    (
      ["cargo", "run", "--release", "--bin", "extract_ast_1", "--",
       input_path, ast_json],
      "AST生成"
    ),
    (
      ["cargo", "run", "--release", "--bin", "extract_cfg", "--",
       input_path, cfg_json],
      "CFG生成"
    ),
    (
      ["cargo", "run", "--release", "--bin", "extract_pdg", "--",
       input_path, pdg_json],
      "PDG生成"
    ),
        ( ["python3", TOKENIZE_AST, ast_json, token_json],         "トークン化"  ),
        ( ["python3", JOINT1, token_json, pdg_json, joint1],       "第一結合"   ),
        ( ["python3", JOINT2, ast_json, joint1, joint2],           "第二結合"   ),
        ( ["python3", JOINT3, cfg_json,  joint2, joint3],           "第三結合"   ),
    ]

    for cmd, desc in steps:
        logging.info(f"{name}: {desc} -> {cmd[-1]}")
        subprocess.run(cmd, check=True)

def main():
    parser = argparse.ArgumentParser(
        description="指定したプロジェクトの dataset/source_codes 以下のファイルだけを処理します"
    )
    parser.add_argument(
        "project",
        help="Projects 配下のプロジェクト名 (例: Project01) またはフルパス"
    )
    args = parser.parse_args()

    # プロジェクトのパスを解決
    if os.path.isabs(args.project):
        work_root = args.project
    else:
        work_root = os.path.join(BASE_DIR, args.project)

    if not os.path.isdir(work_root):
        logging.error(f"プロジェクトディレクトリが見つかりません: {work_root}")
        return

    dataset_dir = os.path.join(work_root, "dataset")
    if not os.path.isdir(dataset_dir):
        logging.error(f"{work_root} に dataset フォルダがありません")
        return

    source_codes = os.path.join(dataset_dir, "source_codes")
    if not os.path.isdir(source_codes):
        logging.error(f"{dataset_dir} に source_codes フォルダがありません")
        return

    graphs_root = os.path.join(dataset_dir, "graphs")
    os.makedirs(graphs_root, exist_ok=True)

    for fname in os.listdir(source_codes):
        input_path = os.path.join(source_codes, fname)
        if not os.path.isfile(input_path):
            continue

        name, _ = os.path.splitext(fname)
        graph_dir = os.path.join(graphs_root, name)
        os.makedirs(graph_dir, exist_ok=True)

        try:
            process_file(input_path, graph_dir)
        except subprocess.CalledProcessError as e:
            logging.error(f"{name}: エラー発生 -> {e}")
            continue

if __name__ == "__main__":
    main()
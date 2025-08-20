#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import re

# ─────────────────────────────────────────────────────
# 修正対象ディレクトリ（必要に応じて書き換えてください）
TARGET_DIR = "/home/shogo-maeda/Solana_Study/programs/Projects/Project8/dataset/source_codes"
# ─────────────────────────────────────────────────────

# STEP1: 「declare_id(」で、直前に「!」がないパターンを検出
pattern_missing_bang   = re.compile(r'(?<!\!)\bdeclare_id\s*\(')
# STEP2: すでに正しく「declare_id!(」と書かれているかどうか
pattern_with_bang      = re.compile(r'\bdeclare_id!\s*\(')
# STEP3 用：構造体リテラルの二重波括弧検知パターン
pattern_struct_open    = re.compile(r'\b(Transfer|External|MyEvent)\s*\{\{')
pattern_struct_close   = re.compile(r'\}\}(?=\s*[),;])')
# STEP4: 属性誤記修正パターン (@[program] -> #[program])
pattern_program_attr   = re.compile(r'@\[\s*program\s*\]')
# ★ 新規追加 STEP5: 属性誤記修正パターン (@[derive(...)] -> #[derive(...)]）
pattern_derive_attr    = re.compile(r'@\[\s*derive\(([^)]*)\)\s*\]')

# 挿入するプログラムID
PROGRAM_ID = 'f6775a6c718db589b2d3b884076ff4f2'
# 挿入用の declare_id! 文
INSERT_LINE = f'declare_id!("{PROGRAM_ID}");\n'

def process_rs_file(path: str):
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    text = ''.join(lines)
    modified = False

    # STEP4: @[program] を #[program] に修正
    new_lines = []
    n_attr_fix = 0
    for line in lines:
        new_line, count = pattern_program_attr.subn('#[program]', line)
        if count:
            n_attr_fix += count
        new_lines.append(new_line)
    if n_attr_fix:
        lines = new_lines
        text = ''.join(lines)
        modified = True
        print(f"[FIX] {path} — @[program] を #[program] に {n_attr_fix} 箇所修正")

    # ★ STEP5: @[derive(...)] を #[derive(...)] に修正
    new_lines = []
    n_derive_fix = 0
    for line in lines:
        # キャプチャした中身をそのまま残す
        new_line, count = pattern_derive_attr.subn(r'#[derive(\1)]', line)
        if count:
            n_derive_fix += count
        new_lines.append(new_line)
    if n_derive_fix:
        lines = new_lines
        text = ''.join(lines)
        modified = True
        print(f"[FIX] {path} — @[derive(...)] を #[derive(...)] に {n_derive_fix} 箇所修正")

    # STEP1: 「!」抜け修正
    new_text, n_fix = pattern_missing_bang.subn("declare_id!(", text)
    if n_fix:
        text = new_text
        modified = True
        print(f"[FIX] {path} — declare_id! へ {n_fix} 箇所修正")

    # STEP2: declare_id! 未登場なら挿入
    if not pattern_with_bang.search(text):
        for idx, line in enumerate(lines):
            if line.strip().startswith("#[program]"):
                lines.insert(idx, INSERT_LINE + "\n")
                modified = True
                print(f"[INS] {path} — declare_id! を挿入")
                break
        text = ''.join(lines)

    # STEP3: 特定の構造体リテラルだけ二重波括弧修正
    text, n_open = pattern_struct_open.subn(r'\1 {', text)
    text, n_close = pattern_struct_close.subn('}', text)
    if n_open or n_close:
        modified = True
        print(f"[FIX] {path} — 二重括弧開きを {n_open}箇所、閉じ括弧を {n_close}箇所修正")

    # 変更があれば上書き保存
    if modified:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(text)

def main():
    for root, _, files in os.walk(TARGET_DIR):
        for fn in files:
            if fn.endswith(".rs"):
                process_rs_file(os.path.join(root, fn))

if __name__ == "__main__":
    main()

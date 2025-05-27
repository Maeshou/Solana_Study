#!/usr/bin/env python3
import os

# リネーム対象のディレクトリ
base_dir = "programs/Projects/Project5/dataset/source_codes"

# ディレクトリ内のエントリを取得（ファイルもフォルダも含む）→ソート
entries = os.listdir(base_dir)
entries.sort()

# 順番に case_001, case_002, ... とリネーム
for idx, name in enumerate(entries, start=1):
    old_path = os.path.join(base_dir, name)
    # 新しい名前の接頭辞
    prefix = f"case_{idx:03d}"
    
    if os.path.isdir(old_path):
        # フォルダなら case_001 のまま
        new_name = prefix
    else:
        # ファイルなら拡張子を保持
        stem, ext = os.path.splitext(name)
        new_name = prefix + ext

    new_path = os.path.join(base_dir, new_name)
    print(f"Renaming {old_path!r} → {new_path!r}")
    os.rename(old_path, new_path)

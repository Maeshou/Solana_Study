#!/usr/bin/env python3
import os
import re
import sys

def process_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Remove #[account(signer)] attributes
    content_new = re.sub(
        r'(?m)^\s*#\s*\[\s*account\s*\(\s*signer\s*\)\s*\]\s*\n',
        '',
        content
    )

    # Replace all occurrences of Signer<'info> (even if followed by comma, semicolon, etc.)
    content_new = re.sub(
        r"Signer<'info>",
        "AccountInfo<'info>",
        content_new
    )

    if content_new != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content_new)
        print(f"Updated: {file_path}")

def process_directory(root_dir):
    for dirpath, _, filenames in os.walk(root_dir):
        for filename in filenames:
            if filename.endswith('.rs'):
                file_path = os.path.join(dirpath, filename)
                process_file(file_path)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: python modify_signer_to_accountinfo.py <directory>')
        sys.exit(1)

    root_directory = sys.argv[1]
    process_directory(root_directory)
    print('Done processing all .rs files.')

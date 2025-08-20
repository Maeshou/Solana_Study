#!/usr/bin/env python3
import json
import re
import sys

# ステートメントをトークン化

def tokenize_statement(stmt):
    token_pattern = re.compile(
        # &mut、二文字／三文字演算子、複合代入演算子、整数リテラル、
        # 識別子、単一文字の記号、その他の１文字
        r'&mut'
      + r'|==|!=|<=|>=|&&|\|\||<<|>>'
      + r'|\+=|\-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>='
      + r'|\d+'
      + r'|[A-Za-z_][A-Za-z0-9_]*'
      + r'|[<>{}()\[\].,;:=+\-*/&|!~^%]'
      + r'|\S'
    )
    tokens = token_pattern.findall(stmt)

    merged = []
    i = 0
    while i < len(tokens):
        if tokens[i] == '&' and i+1 < len(tokens) and tokens[i+1] == 'mut':
            merged.append('&mut')
            i += 2
        else:
            merged.append(tokens[i])
            i += 1
    return merged

# トークン属性マップ
TOKEN_ATTR_MAP = {
    'let': 'define', 'fn': 'function', 'mod': 'module',
    '==':'operator','!=':'operator','<=':'operator','>=':'operator',
    '&&':'operator','||':'operator','=':'operator',
    '+=':'operator','-=':'operator','*=':'operator','/=':'operator','%=':'operator',
    '&=':'operator','|=':'operator','^=':'operator','<<=':'operator','>>=':'operator',
    '&mut':'mut', '.':'member', '(': 'delimiter', ')':'delimiter',
    '[':'delimiter', ']':'delimiter', '{':'delimiter', '}':'delimiter',
    ',':'delimiter',';':'delimiter','<':'delimiter','>':'delimiter'
}

def get_token_attribute(tok):
    return TOKEN_ATTR_MAP.get(tok, 'identifier')

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage: python3 tokenize_ast.py <input_json> <output_json>')
        sys.exit(1)

    # JSON 形式の AST データ読み込み
    with open(sys.argv[1], encoding='utf-8') as f:
        ast_data = json.load(f)

    graph = {'nodes': [], 'edges': []}
    nid = 1

    def add_node(label, attr):
        global nid
        node = {'id': nid, 'label': label, 'attributes': attr}
        graph['nodes'].append(node)
        nid += 1
        return node['id']

    def add_edge(src, tgt, lbl):
        graph['edges'].append({'source': src, 'target': tgt, 'label': lbl})

    # 代入演算子リスト
    assignment_ops = ['=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=']

    # 比較演算子リスト
    comparison_ops = ['==', '!=', '<=', '>=', '<', '>']

    if_or_loops = ['if','for','while']

    # モジュールノード
    mod_id = add_node('mod', 'module')

    # 関数ノード処理
    for node in ast_data:
        if node.get('node_type') == 'function':
            func_id = add_node(node['name'], 'function')
            add_edge(mod_id, func_id, 'contains')

            # 引数ノード処理
            inputs_id = add_node('inputs', 'inputs')
            add_edge(func_id, inputs_id, 'has')
            for p in node.get('inputs', []):
                m = re.search(r'<\s*([A-Za-z0-9_]+)\s*>', p)
                name, typ = (m.group(1), 'structure') if m else (p, 'value')
                pid = add_node(name, typ)
                add_edge(inputs_id, pid, 'parameter')

            # 文ごとのリンク用変数
            prev_expr = None
            prev_head = None
            pre_bl_head_id = None#分岐やループのid

            for stmt in node.get('body', []):
                toks = tokenize_statement(stmt)

                # "{" のみ、"}" のみの行はノードのみ追加し、エッジはスキップ
                if len(toks) == 1 and toks[0] in ('{', '}'):
                    brace_id = add_node(toks[0], get_token_attribute(toks[0]))
                    prev_expr = None
                    pre_bl_head_id = None
                    prev_head = None
                    continue
                
                # ジェネリクス用の <...> を除去 (空 <> も含む)
                skip_idx = set()
                stack = []
                for idx, t in enumerate(toks):
                    if t == '<':
                        stack.append(idx)
                    elif t == '>' and stack:
                        start = stack.pop()
                        inner = toks[start+1:idx]
                        # 中身が識別子とカンマだけならジェネリクスと判断
                        if all(re.match(r'^[A-Za-z0-9_]+$|^,$', x) for x in inner):
                            skip_idx.add(start)
                            skip_idx.add(idx)
                toks = [t for i, t in enumerate(toks) if i not in skip_idx]

                # 代入行の検出
                found = [op for op in assignment_ops if toks.count(op) == 1]
                has_assign = len(found) == 1

                 # 比較演算子検出
                found_cmp = [op for op in comparison_ops if toks.count(op) == 1]
                has_cmp = len(found_cmp) == 1

                # 制御文行の検出
                has_ctrl = any(t in ('if', 'for', 'while') for t in toks)

                if has_assign:
                    # 単一の代入演算子処理
                    op = found[0]
                    idx = toks.index(op)
                    # expression ノード生成
                    expr_id = add_node('expression', 'expression')
                    add_edge(func_id, expr_id, 'has')
                    if prev_expr is not None:
                        add_edge(prev_expr, expr_id, 'next')

                    if pre_bl_head_id is not None:
                        print("aida")

                    if prev_head is not None:
                        add_edge(prev_head, expr_id, 'next')
                            
                    prev_expr = expr_id
                    # head リンクをリセット
                    prev_head = None

                    # operator ノード生成
                    op_id = add_node(op, 'operator')
                    add_edge(expr_id, op_id, 'contains')
                    # lhs トークン処理
                    prev = None
                    for t in toks[:idx]:
                        tid = add_node(t, get_token_attribute(t))
                        if prev is None:
                            add_edge(op_id, tid, 'lhs')
                        else:
                            add_edge(prev, tid, 'next')
                        prev = tid
                    # rhs トークン処理
                    prev = None
                    for t in toks[idx+1:]:
                        tid = add_node(t, get_token_attribute(t))
                        if prev is None:
                            add_edge(op_id, tid, 'rhs')
                        else:
                            add_edge(prev, tid, 'next')
                        prev = tid

                elif has_ctrl:
                    # if/for/while 行
                    head_tok = toks[0]
                    head_id = add_node(head_tok, get_token_attribute(head_tok))
                    # if prev_expr is not None:
                    #     add_edge(prev_expr, head_id, 'next')
                    # if prev_head is not None:
                    #     add_edge(prev_head, head_id, 'next')
                    pre_bl_head_id = head_id
                    prev = head_id
                    prev_expr = None
                    prev_head = None

                    if has_cmp:
                        # --- 追加：比較演算子処理 ---\
                        op = found_cmp[0]
                        idx = toks.index(op)
                        op_id = add_node(op, 'operator')
                        # lhs トークン処理
                        prev = None
                        for t in toks[1:idx]:
                            tid = add_node(t, get_token_attribute(t))
                            if prev is None:
                                add_edge(op_id, tid, 'lhs')
                            else:
                                add_edge(prev, tid, 'next')
                            prev = tid
                        # rhs トークン処理
                        prev = None
                        for t in toks[idx+1:]:
                            tid = add_node(t, get_token_attribute(t))
                            if prev is None:
                                add_edge(op_id, tid, 'rhs')
                            else:
                                add_edge(prev, tid, 'next')
                            prev = tid
                    else:        
                        for t in toks[1:]:
                            tid = add_node(t, get_token_attribute(t))
                            add_edge(prev, tid, 'next')
                            prev = tid
                        
                    
                else:
                    # その他の行
                    head_tok = toks[0]
                    head_id = add_node(head_tok, get_token_attribute(head_tok))
                    if prev_expr is not None:
                        add_edge(prev_expr, head_id, 'next')
                    if prev_head is not None:
                        add_edge(prev_head, head_id, 'next')
                    prev_head = head_id
                    prev = head_id
                    prev_expr = None
                    pre_bl_head_id = None
                    for t in toks[1:]:
                        tid = add_node(t, get_token_attribute(t))
                        add_edge(prev, tid, 'next')
                        prev = tid

    # 構造体ノード処理（変更なし）
    for node in ast_data:
        if node.get('node_type') == 'struct':
            struct_id = add_node(node['name'], 'structure')
            add_edge(mod_id, struct_id, 'contains')
            for field in node.get('fields', []):
                ftype = field.get('field_type', '')
                main = re.split(r'<', ftype, maxsplit=1)[0].strip() or 'field'
                fid = add_node(field['name'], main)
                add_edge(struct_id, fid, 'has')
                if '<' in ftype and '>' in ftype:
                    inner = ftype.split('<', 1)[1].rsplit('>', 1)[0]
                    for part in [p.strip() for p in inner.split(',')]:
                        iid = add_node(part, 'field_inner')
                        add_edge(fid, iid, 'inner_type')

    # 結果出力
    with open(sys.argv[2], 'w', encoding='utf-8') as f:
        json.dump(graph, f, ensure_ascii=False, indent=2)
    print(f'Token graph saved to {sys.argv[2]}')

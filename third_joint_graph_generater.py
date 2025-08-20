#!/usr/bin/env python3
import json
import sys
from collections import OrderedDict

# ブロック範囲検出: start_idx以降で最初の'{'と対応する'}'の位置を返す
def find_block_range(labels, start_idx):
    brace_start = None
    for i in range(start_idx, len(labels)):
        if labels[i].strip() == '{':
            brace_start = i
            break
    if brace_start is None:
        return None, None
    depth = 0
    for j in range(brace_start, len(labels)):
        tok = labels[j].strip()
        if tok == '{':
            depth += 1
        elif tok == '}':
            depth -= 1
            if depth == 0:
                return brace_start, j
    return brace_start, None
# ループ開始トークンからブロック開始 '{' までを丸ごと削除
def remove_loop_headers(nodes, labels):
    new_nodes = []
    new_labels = []
    skip = False
    detected = False
    print("remove_loop_headers")
    for n, l in zip(nodes, labels):
        if l in ('for','while'):
            print(f"for or while node = {n}")
        if not skip and l in ('for', 'while') and not detected:
            # 'for' または 'while' を見つけたらスキップ開始
            print("for ari")
            skip = True
            detected = True
            continue
        if skip:
            # ブロック開始 'Loop Start' が出たらノード追加
            if l.strip() == 'Loop Start':
                new_nodes.append(n)
                new_labels.append(l)
            # ブロック開始 '{' が出たらスキップ終了して再開
            if l.strip() == '{':
                new_nodes.append(n)
                new_labels.append(l)
                skip = False
            continue
        # スキップ中でなければそのままキープ
        new_nodes.append(n)
        new_labels.append(l)
    return new_nodes, new_labels


def insert_cfg_nodes(joint_graph, cfg_data):
    cfg_nodes = cfg_data['nodes']
    print(cfg_nodes)
    cfg_if_ids = {n['id'] for n in cfg_nodes if n['label'] == 'if statement'}
    False_count = 0
    # CFGのエッジ辞書
    succs = {}
    for e in cfg_data['edges']:
        succs.setdefault(e['from'], []).append((e['label'], e['to']))

    # joint_nodes に _old_id を付与
    joint_nodes = []
    for n in joint_graph['nodes']:
        nd = n.copy()
        nd['_old_id'] = nd['id']
        if nd['_old_id'] == 61:
            print(f"saved old61")
        joint_nodes.append(nd)
    # joint_labels の初期化
    joint_labels = [n['label'] for n in joint_nodes]
    false_body_stack = []
    loop_stack = []
    # --- if statement + predicate + True/False body の挿入 ---
    for idx_cfg, node in enumerate(cfg_nodes):
        if node['label'] == 'predicate':
            if idx_cfg != 0 or cfg_nodes[idx_cfg-1]['label'] == 'if statement':
                if_id, pred_id = cfg_nodes[idx_cfg-1]['id'], node['id']
                print(f"if_id = {if_id}")
                print(f"pred_id = {pred_id}")

                # 元の "if" ノード位置を探す
                try:
                    idx_if = next(i for i,n in enumerate(joint_nodes) if n['label']=='if')
                except StopIteration:
                    continue


                # if statement ノード挿入
                if_nd = {
                    'id':idx_if,
                    '_old_id': None,
                    'label': 'if statement',
                    'attributes': 'conditional branch'
                }

                joint_nodes.insert(idx_if, if_nd)
                print(f"idx_if = {idx_if}")
                joint_labels.insert(idx_if, if_nd['label'])
                print(f"if_nd['label'] = {if_nd['label']}")

                # predicate ノード挿入
                pred_nd = {
                    'id':idx_if+1,
                    '_old_id': None,
                    'label': 'predicate',
                    'attributes': 'predicate'
                }
                joint_nodes.insert(idx_if+1, pred_nd)
                joint_labels.insert(idx_if+1, pred_nd['label'])
                print(f"pred_nd['label'] = {pred_nd['label']}")

                # 真偽ブロック挿入位置
                brace_start, _ = find_block_range(joint_labels, idx_if+1)
                print(f"joint_labels[brace_start] = {joint_labels[brace_start]}")
                print(f"joint_nodes[brace_start] = {joint_nodes[brace_start]}")
                insert_pos = brace_start if brace_start is not None else idx_if+1


                # True/False body ノード挿入
                True_nd = {
                    'id':brace_start,
                    '_old_id':None,
                    'label': 'True body',
                    'attributes': 'branch target'
                }
                print(f"insert_pos = {insert_pos}")
                joint_nodes.insert(insert_pos, True_nd)
                joint_labels.insert(insert_pos, True_nd['label'])
                # ① 元の 'if' トークンを削除
                
                for i, n in enumerate(joint_nodes):
                    if i == idx_if + 2:
                        print(f"mode i == idx_if + 2 :{n}")

                for i, n in enumerate(joint_labels):
                    if i == idx_if + 2:
                        print(f"label i == idx_if + 2 :{n}")

                joint_nodes = [
                    n for i, n in enumerate(joint_nodes)
                    if not (i == idx_if + 2 and n['label'] == 'if')
                ]
                joint_labels = [
                    l for i, l in enumerate(joint_labels)
                    if not (i == idx_if + 2)
                ]
                print(f"true len(joint_nodes)={len(joint_nodes)}")
                print(f"true len(joint_labels)={len(joint_labels)}")
                print(f"joint_nodes[idx_if] = {joint_nodes[idx_if]}")    

        if node['label'] == 'False body':
            _, brace_end = find_block_range(joint_labels, idx_if+1)#chokkanndesounyu
            print(f"joint_nodes[idx_if+1] = {joint_nodes[idx_if+1]}")  
            False_count +=1
            print(f"idx_cfg={idx_cfg}")
            print(f"node['label'] = {node['label']}")
            print(f"False body detected False_count:{False_count}")
            # 真偽ブロック挿入位置
            print(f"joint_labels[brace_end] = {joint_labels[brace_end]}")
            insert_pos = brace_end if brace_end is not None else insert_pos+1
            if insert_pos == brace_end:
                print("same")
            print(f"brace_end = {brace_end}")
            print(f"joint_nodes[brace_end] = {joint_nodes[brace_end]}")  
            
            #print(f"joint_nodes[189] = {joint_nodes[189]}")   
            insert_pos += 1
            False_nd = {
                'id':insert_pos,
                '_old_id':None,
                'label': 'False body',
                'attributes': 'branch target'
            }

            joint_nodes.insert(insert_pos, False_nd)
            joint_labels.insert(insert_pos, False_nd['label'])

            false_body_stack.append(insert_pos+1)

            if cfg_nodes[idx_cfg+1]['label'] == 'No-op':
                insert_pos +=1
                noop_nd = {
                'id':insert_pos+1,
                '_old_id':None,
                'label': 'No-op',
                'attributes': 'branch content'
                }
                
                print(f"noop insert_pos+1 = {insert_pos+1}")
                joint_nodes.insert(insert_pos, noop_nd)
                joint_labels.insert(insert_pos, noop_nd['label'])
                noop_close_nd = {
                'id':insert_pos+2,
                '_old_id':None,
                'label': ';',
                'attributes': 'delimiter'
                }
                joint_nodes.insert(insert_pos+1, noop_close_nd)
                joint_labels.insert(insert_pos+1, noop_close_nd['label'])
            print(f"brace_end = {brace_end}")    
            print(f"joint_labels[brace_end] = {joint_labels[brace_end]}")
            print(f"joint_nodes[brace_end] = {joint_nodes[brace_end]}")       
            print(f"joint_nodes[insert_pos+1] = {joint_nodes[insert_pos+1]}")   
            print(f"false len(joint_nodes)={len(joint_nodes)}")
            print(f"false len(joint_labels)={len(joint_labels)}")

            # ① 元の 'else' トークンを削除
            joint_nodes = [
                n for i, n in enumerate(joint_nodes)
                if not (i == insert_pos + 1 and n['label'] == 'else')
            ]
            joint_labels = [
                l for i, l in enumerate(joint_labels)
                if not (i == insert_pos+ 1 and l == 'else')
            ]
       
    # --- merge + Loop End 挿入 ---
        if node['label'] == 'merge':
            if false_body_stack:
                insert_pos = false_body_stack.pop()
            else:
                print("stack is empty")    
            if cfg_nodes[idx_cfg-1]['label'] == 'No-op':
                insert_pos += 1
                merge_nd = {
                'id':insert_pos+1,
                '_old_id':None,
                'label': 'merge',
                'attributes': 'branch end'
                }
                print(f"merge insert_pos+1 = {insert_pos+1}")
                joint_nodes.insert(insert_pos+1, merge_nd)
                joint_labels.insert(insert_pos+1, merge_nd['label'])

            else:
                _, brace_end = find_block_range(joint_labels, insert_pos)
                print(f"mergejoint_labels[insert_pos-1]_ = {joint_nodes[insert_pos-1]}")
                print(f"mergejoint_labels[insert_pos]_ = {joint_nodes[insert_pos]}")
                print(f"mergejoint_labels[insert_pos+1]_ = {joint_nodes[insert_pos+1]}")
                print(f"mergejoint_labels[brace_end] = {joint_labels[brace_end]}")
                insert_pos = brace_end if brace_end is not None else insert_pos+1
                print(f"mergejoint_labels[insert_pos] = {joint_nodes[insert_pos+1]}")
                merge_nd = {
                'id':insert_pos,
                '_old_id':None,
                'label': 'merge',
                'attributes': 'branch end'
                }
                # merge
                joint_nodes.insert(insert_pos+1, merge_nd)
                joint_labels.insert(insert_pos+1, merge_nd['label'])
            

    # --- Loop Start 挿入 ---
        if node['label'] == 'Loop Start':
            print("Loop start detected")
            print(f"idx_cfg = {idx_cfg}")
            try:
                idx_loop = next(i for i,n in enumerate(joint_nodes) if n['label']=='for' or n['label']=='while')
            except StopIteration:
                print("deleted")
                continue
            print(f"idx_loop = {idx_loop}") 
            loop_stack.append(idx_loop)       
            loop_start, _ = find_block_range(joint_labels, idx_loop)
            insert_pos = loop_start if loop_start is not None else idx_loop+1
            print(f"loop_insert_pos = {insert_pos}")
            loop_start_nd = {
                'id':insert_pos,
                '_old_id':None,
                'label': 'Loop Start',
                'attributes': 'Loop Start'
                }
           
            joint_nodes.insert(insert_pos, loop_start_nd)
            joint_labels.insert(insert_pos, loop_start_nd['label'])
             # 使い方（挿入処理の最後のほうで呼び出す）
            joint_nodes, joint_labels = remove_loop_headers(joint_nodes, joint_labels)

        if node['label'] == 'Loop End':   
            print("loop end detected")
            print(f"idx_cfg ={idx_cfg}")
            if loop_stack:
                idx_loop = loop_stack.pop()
            else:
                print("Warning: Loop End に対応する Loop Start が見つかりません")
                idx_loop = idx_cfg  # とりあえず現在位置を代用
            print(f"popped idx_loop = {idx_loop}")    
            _, loop_end = find_block_range(joint_labels, idx_loop)
            insert_pos = loop_end if loop_end is not None else idx_loop+1
            print(f"joint_labels[insert_pos] = {joint_labels[insert_pos]}")
            if loop_end:
                print("loop end ari")
            print(f"loop_end_insert_pos = {insert_pos}")
            loop_end_nd = {
                'id':insert_pos+1,
                '_old_id':None,
                'label': 'Loop End',
                'attributes': 'Loop End'
                }
            joint_nodes.insert(insert_pos+1, loop_end_nd)
            joint_labels.insert(insert_pos+1, loop_end_nd['label'])

           
    # --- ID再連番 & エッジリマップ ---
    mapping = {}
    new_nodes = []
    for new_id, n in enumerate(joint_nodes):
        old = n.pop('_old_id', None)
        if old == 178:
            print("old178 detected")
            print(f"new_id = {new_id}")
        if old == 177:
            print("old177 detected")
            print(f"new_id = {new_id}")
        if old is not None:
            mapping[old] = new_id
        n['id'] = new_id
        new_nodes.append(n)

    new_edges = []
    for e in joint_graph['edges']:
        src = e.get('source', e.get('from'))
        tgt = e.get('target', e.get('to'))
        
        try:
            new_edges.append({
            'source': mapping[src],
            'target': mapping[tgt],
            'label': e['label']
            })

        except KeyError:
            continue    
        
     # 'if' ノードにかかっていた 'next' エッジを 'if statement' へリダイレクト
    if_stmt = next((n for n in new_nodes if n['label']=='if statement'), None)
    redirected = []
    for e in new_edges:
        if e['label']=='next' and if_stmt:
            src_lbl = new_nodes[e['source']]['label']
            tgt_lbl = new_nodes[e['target']]['label']
            if src_lbl=='if':
                e['source'] = if_stmt['id']
            if tgt_lbl=='if':
                print("if node detected")
                e['target'] = if_stmt['id']
        redirected.append(e)
    new_edges = redirected


    return {'nodes': new_nodes, 'edges': new_edges}

    
# エッジ挿入フェーズ
def insert_cfg_edges(joint_graph):
    nodes  = joint_graph['nodes']
    
    edges  = list(joint_graph['edges'])
    labels = [n['label'] for n in nodes]

    def scan_block(start, end):
        i = start
        while i < end:
            lbl = labels[i]
            if lbl == 'if statement':
                if end == 398:
                    print(f"start={start}")
                    print("here")
                i = process_if(i, end)
                continue
            if lbl == 'Loop Start':
                i = process_loop(i, end)
                continue
            i += 1

    def process_if(idx, end):
        pred_i = true_i = false_i = merge_i = None

        # predicate→True body 検出
        i = idx + 1
        while i < end:
            lbl = labels[i]
            if lbl == 'predicate' and pred_i is None:
                pred_i = i
            if lbl == 'True body':
                true_i = i
                break
            i += 1

        # true エッジ追加
        if true_i is not None and true_i > 0:
            src = nodes[idx]['id']
            edges.append({
                'source': src,
                'target': nodes[true_i]['id'],
                'label': 'true'
            })
            j = true_i + 1 
            truebody_next = None
            while j < len(labels) and labels[j] != ';':
                if labels[j] == 'Loop Start':
                    truebody_next = j
                    break
                elif labels[j] == 'if statement':
                    truebody_next = j
                    break
                elif labels[j] == 'expression':
                    truebody_next = j
                    break
                j += 1
            # ターゲットを決定: 見つかればそのexpressionノード、なければmerge直後
            target_idx = truebody_next if truebody_next is not None else true_i + 2 #if があればTrue bodyは必ず存在するため＋２とする
            if target_idx < len(nodes):
                edges.append({
                    'source': nodes[true_i]['id'],
                    'target': nodes[target_idx]['id'],
                    'label': 'next'
                })


        # False body 検出
        j = (true_i+1 if true_i is not None else idx+1)
        print(f"j = {j}")
        print(f"end = {end}")
        while j < end:
            lbl = labels[j]
            print(lbl)
            if lbl == 'False body':
                false_i = j
                break
            j += 1
        print(f"false_i = {false_i}")
        # false エッジ追加
        if true_i is not None and false_i is not None:
            src = nodes[idx]['id']
            edges.append({
                'source': src,
                'target': nodes[false_i]['id'],
                'label': 'false'
            })

            j = false_i + 1 
            falsebody_expr_i = None
            noop_i = None
            while j < len(labels) and labels[j] != ';':
                if labels[j] == 'expression':
                    falsebody_expr_i = j
                    break
                elif labels[j] == 'Loop Start':
                    falsebody_expr_i = j
                    break
                elif labels[j] == 'if statement':
                    falsebody_expr_i = j
                    break
                elif labels[j] == 'No-op':
                    noop_i =j
                    break
                j += 1
            # ターゲットを決定: 見つかればそのexpressionノード、なければmerge直後
            #target_idx = falsebody_expr_i if falsebody_expr_i is not None else false_i + 1
            if falsebody_expr_i is not None:
                target_idx = falsebody_expr_i
                print(1)
            elif noop_i is not None:
                target_idx = noop_i
                print(2)
            else:
                print(3)
                target_idx = false_i + 2

            print(f"false body nodes[false_i]={nodes[false_i]}")    
            print(f"false body nodes[target_idx]={nodes[target_idx]}")
            if target_idx < len(nodes):
                edges.append({
                    'source': nodes[false_i]['id'],
                    'target': nodes[target_idx]['id'],
                    'label': 'next'
                })

        # merge 検出
        k = (false_i+1 if false_i is not None
            else (true_i+1 if true_i is not None else idx+1))
        while k < end:
            lbl = labels[k]
            if lbl == 'merge':
                merge_i = k
                break
            k += 1

        # merge→直後ノード next
        if merge_i is not None and merge_i+1 < len(nodes):
            edges.append({
                'source': nodes[true_i]['id'],
                'target': nodes[merge_i]['id'],
                'label': 'next'
            })
            edges.append({
                'source': nodes[false_i]['id'],
                'target': nodes[merge_i]['id'],
                'label': 'next'
            })
             # ';'まで探索して最初のexpressionノードを探す
            j = merge_i + 1
            expr_i = None
            while j < len(labels) and labels[j] not in (';', '}'):
                if labels[j] == 'expression':
                    expr_i = j
                    break
                elif labels[j] == 'Loop Start':
                    expr_i = j
                    break
                elif labels[j] == 'if statement':
                    expr_i = j
                    break
                elif labels[j] == 'Loop End':
                    expr_i = j
                    break
                j += 1
            # ターゲットを決定: 見つかればそのexpressionノード、なければmerge直後
            target_idx = expr_i if expr_i is not None else merge_i + 1
            while labels[target_idx] in (';', '}'):
                target_idx += 1

            print(f"merge labels[target_idx]={labels[target_idx]}")
            if target_idx < len(nodes):
                edges.append({
                    'source': nodes[merge_i]['id'],
                    'target': nodes[target_idx]['id'],
                    'label': 'next'
                })

        # --- ここから新規追加 ---
        # if statement と predicate 周りの next エッジ
        if pred_i is not None:
            # ② if statement→predicate
            edges.append({
                'source': nodes[idx]['id'],
                'target': nodes[pred_i]['id'],
                'label': 'next'
            })
            # ③ predicate→直後ノード後で削除
            j = pred_i + 1
            ope_i = None
            while j < len(labels) and labels[j] != '{':
                print(f"predicate~ {nodes[j]}")
                if nodes[j]['attributes'] == 'operator':
                    ope_i = j
                    break
                j += 1
            # ターゲットを決定: 見つかればそのexpressionノード、なければmerge直後
            target_idx = ope_i if ope_i is not None else pred_i + 1
            if target_idx < len(nodes):
                edges.append({
                    'source': nodes[pred_i]['id'],
                    'target': nodes[target_idx]['id'],
                    'label': 'next'
                })
                print(f"nodes[target_idx] = {nodes[target_idx]}")
            
        # --- ここまで新規追加 ---

        # branchネスト分岐走査
        if true_i is not None and false_i is not None and true_i+1 < false_i:
            if false_i == 398:
                print("here2")
            scan_block(true_i+1, false_i)
        if false_i is not None and merge_i is not None and false_i+1 < merge_i:
            if merge_i == 398:
                print("here3")
            scan_block(false_i+1, merge_i)

        return (merge_i+1) if merge_i is not None else (idx+1)


    def process_loop(idx, end):
        end_i = None
        j = idx + 1
        while j < end:
            lbl = labels[j]
            if lbl == 'Loop Start':
                j = process_loop(j, end)
                continue
            if lbl == 'Loop End':
                end_i = j
                break
            j += 1

        if end_i is not None:
            # ループバック用エッジ
            edges.append({
                'source': nodes[idx]['id'],
                'target': nodes[end_i]['id'],
                'label': 'loop'
            })

            # Loop Start → ';'まで探索し、最初の'expression'ノードへ、なければ直後ノードへ
            j = idx + 1
            expr_i = None
            while j < len(labels) and labels[j] != ';':
                if labels[j] == 'expression':
                    expr_i = j
                    break
                elif labels[j] == 'Loop Start':
                    
                    expr_i = j
                    break
                elif labels[j] == 'if statement':
                    expr_i = j
                    break
                j += 1
            target_i = expr_i if expr_i is not None else (idx + 2)
            if target_i < len(nodes):
                edges.append({
                    'source': nodes[idx]['id'],
                    'target': nodes[target_i]['id'],
                    'label': 'next'
                })

            # ループ本体再帰走査
            if idx + 1 < end_i:
                if end_i == 398:
                    print("here3")
                scan_block(idx + 1, end_i)

            # Loop End → ';'まで探索し、最初の'expression'ノードへ、なければ直後ノードへ
            j = end_i + 1
            expr_i = None
            while j < len(labels) and labels[j] != ';':
                if labels[j] == 'expression':
                    expr_i = j
                    break
                elif labels[j] == 'Loop Start':
                    expr_i = j
                    break
                elif labels[j] == 'if statement':
                    expr_i = j
                    break
                j += 1
            target_i = expr_i if expr_i is not None else (end_i + 1)
            if target_i < len(nodes):
                edges.append({
                    'source': nodes[end_i]['id'],
                    'target': nodes[target_i]['id'],
                    'label': 'next'
                })
                
            return end_i + 1

        return idx+1
    
    # --- 新規: if statement と Loop Start ,'Loop End'に対する backward-scan 次ノード接続 ---
    for target_label in ('if statement', 'Loop Start','Loop End'):
        for idx, lbl in enumerate(labels):
            if lbl != target_label:
                continue
            target_idx = idx
            # 直前の ';' を探す
            prev_semi = next((j for j in range(target_idx-1, -1, -1) if labels[j] == ';'), None)
            if prev_semi is None:
                continue
            # さらにその前の ';' を探し、探索区間を決定
            prev_prev_semi = next((j for j in range(prev_semi-1, -1, -1) if labels[j] == ';'), None)
            start = prev_prev_semi + 1 if prev_prev_semi is not None else 0
            end = prev_semi - 1
            # 区間内を後ろから前に走査して最初の 'expression' ノードを探す
            source_idx = next((j for j in range(end, start-1, -1) if labels[j] == 'expression'), None)
            # 見つからなければ、区間の先頭ノードをソースに
            if source_idx is None:
                print(f"nodes[start] = {nodes[start]}")
                print(f"target_label = {target_label}")
                while nodes[start]['label'] == "{" or nodes[start]['label'] == "}" or nodes[start]['label'] == "Loop Start" or nodes[start]['label'] == "Loop End" or nodes[start]['label'] == "False body" or nodes[start]['label'] == "No-op" or nodes[start]['label'] == ";": 
                    start += 1
                source_idx = start
            print(f"nodes[source_idx]_1 = {nodes[source_idx]}")    
            # next エッジ追加
            edges.append({
                'source': nodes[source_idx]['id'],
                'target': nodes[target_idx]['id'],
                'label': 'next'
            })

    source_label = 'Loop End'
    for idx, lbl in enumerate(labels):
        if lbl != source_label:
            continue
        source_idx = idx
        print(f"lbl = {lbl}") 
        print(f"nodes[source_idx]_3 = {nodes[source_idx]}") 
        # ';' を探す
        next_semi = next((j for j in range(source_idx+1, len(labels)) if labels[j] == ';'), None)
        if next_semi is None:
            continue
        end = source_idx + 1
        # 区間内を後ろから前に走査して最初の 'expression' ノードを探す
        target_idx = next((j for j in range(source_idx, next_semi) if labels[j] == 'expression'), None)
        # 見つからなければ、区間の先頭ノードをソースに
        if target_idx is None:
            print(f"nodes[start] = {nodes[start]}")
            print(f"target_label = {target_label}")
            while nodes[end]['label'] == "{": 
                end += 1
            target_idx = end
        print(f"nodes[source_idx]_2 = {nodes[source_idx]}")    
        print(f"nodes[target_idx]_2 = {nodes[target_idx]}")  
        # next エッジ追加
        edges.append({
            'source': nodes[source_idx]['id'],
            'target': nodes[target_idx]['id'],
            'label': 'next'
        })

    # 全体走査
    scan_block(0, len(nodes))
    return {'nodes':nodes, 'edges':edges}

# --- メイン処理 ---
def main():
    if len(sys.argv) != 4:
        print(f"Usage: {sys.argv[0]} <cfg_list.json> <joint.json> <out.json>")
        sys.exit(1)

    # CFG JSON を読み込み（リスト or 単一）
    with open(sys.argv[1], 'r', encoding='utf-8') as f:
        raw = json.load(f)
    all_cfgs = raw if isinstance(raw, list) else [raw]

    # --- 変更点: 複数関数のCFGを1つにマージ ---
    merged_cfg = {'nodes': [], 'edges': []}
    offset = 0
    for cfg in all_cfgs:
        # ノードをオフセットして追加
        for n in cfg.get('nodes', []):
            merged_cfg['nodes'].append({
                'id': n['id'] + offset,
                'label': n['label'],
                'attributes': n.get('attributes', '')
            })
        # エッジをオフセットして追加
        for e in cfg.get('edges', []):
            merged_cfg['edges'].append({
                'from': e['from'] + offset,
                'to': e['to'] + offset,
                'label': e['label']
            })
        # 次の関数用にIDオフセット更新
        max_id = max(n['id'] for n in merged_cfg['nodes']) if merged_cfg['nodes'] else -1
        offset = max_id + 1

    # Joint-graph 読み込み
    with open(sys.argv[2], 'r', encoding='utf-8') as f:
        joint = json.load(f)

    # 1つにまとめたCFGで処理
    first = insert_cfg_nodes(joint, merged_cfg)
    out_graph = insert_cfg_edges(first)

     # OrderedDict でキー順を固定し出力
    out = OrderedDict([
        ('nodes', [OrderedDict([
            ('id',   n['id']),
            ('label',n['label']),
            ('attributes',n.get('attributes',''))
        ]) for n in out_graph['nodes']]),
        ('edges', [OrderedDict([
            ('source',e['source']),
            ('target',e['target']),
            ('label', e['label'])
        ]) for e in out_graph['edges']])
    ])

    with open(sys.argv[3], 'w', encoding='utf-8') as f:
        json.dump(out, f, ensure_ascii=False, indent=2)

    print(f"Merged graph written to {sys.argv[3]}")

if __name__ == '__main__':
    main()

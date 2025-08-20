import json
import re
import os

def parse_ast(ast_data):
    """
    ASTデータを解析し、必要な情報を収集します。
    """
    structs = []
    for item in ast_data:
        if item.get("node_type") == "struct":
            struct_name = item["name"]
            fields = item.get("fields", [])
            structs.append({
                "struct_name":struct_name,
                "fields": fields,
                "is_init_struct": any("init" in (field.get("attribute") or "") for field in fields)
            })
    return structs

def find_has_one_field(structs):
    """
    has_one= を含むフィールドを探し、対応するフィールドが存在するか確認します。
    """
    has_one_field_names = []
    for struct in structs:
        struct_name = struct["struct_name"]
        for field in struct["fields"]:
            attribute = field.get("attribute", "")
            field_type = field.get("field_type","")
            if isinstance(attribute, str):  # 文字列型か確認
                match = re.search(r"has_one\s*=\s*(\w+)", attribute)
                if match:
                    has_one_field_name = match.group(1)
                    if has_one_field_name in [f["name"] for f in struct["fields"]]:
                        has_one_field_names.append([
                            f for f in struct["fields"] if f["name"] == has_one_field_name
                        ])
                    else:
                        print(f"[WARNING] Related struct '{has_one_field_name}' not found for field '{field['name']}' in struct '{struct_name}'.")
            else:
                print(f"[INFO] Skipping non-string attribute: {attribute}")


    return has_one_field_names

def check_field_updates(pdg_data, all_pdg_files, all_has_one_relations, init_structs, i, define, update, depend,all_structs,account_name,field_name):
    """
    AST,PDGデータを解析し、指定したフィールドが更新されているか確認します。
    """
    fields = []  # 初期化
    print(f"i: {i}")
    structs = all_structs[i]
    #各ファイルのhas_oneの有無を確認 
    has_one_relation = all_has_one_relations[i]
    print(f"struct: {structs}")

    related_field = None  # 更新されたフィールド名を追跡
    print("動作確認1")
    for function in pdg_data:
        nodes = function.get("nodes", [])
        edges = function.get("edges", [])
        inputs = function.get("inputs", [])

        # inputsがリストの場合、要素を結合して文字列に変換
        if isinstance(inputs, list):
            inputs_string = " ".join(inputs)
        else:
            inputs_string = str(inputs)

        struct_name_match = re.search(r"Context\s*<\s*(\w+)\s*>", inputs_string)
        struct_name = struct_name_match.group(1) if struct_name_match else None
        
        for st_dict in structs:#関数の入力値となる構造体を探す
            print(f"st_dict{st_dict}:struct_name{struct_name}")
            if struct_name == st_dict["struct_name"]:
                fields = st_dict["fields"]
                print(f"fields(動作確認): {fields}")

        print(f"fields: {fields}")
        print(f"struct_name: {struct_name}")
        print(f"init_structs:{init_structs}")

        # init_structsをフラット化
        flat_init_structs = [item for sublist in init_structs for item in sublist]
        print(f"flat_init_structs: {flat_init_structs}")

        if struct_name and struct_name not in [flat_init["struct_name"] for flat_init in flat_init_structs]:
            print(f"struct_name1: {struct_name}")
            for node in nodes:
                label = node.get("label", "")
                node_id = node.get("id")
                if edges:
                    for edge in edges:
                        if "to" in edge and node_id == edge["to"]:
                            print(rf"動作確認9:node_id= {node_id}")
                            print(rf"動作確認9:edge['to'] = {edge['to']}")

                            if "def" in edge["label"]:#defineの値は構造体の値と対応している？(1/6)
                                defined_val = re.search(r"def:\s*(\w+)", edge["label"]).group(1)
                                print(rf"defined_val(動作確認7)= {defined_val}")
                                if defined_val not in define:
                                    define.append(defined_val)
                                    
                                    # 正規表現で `ctx . accounts .` の後のフィールド名を抽出
                                    match = re.search(r"ctx\s*\.\s*accounts\s*\.\s*(\w+)", label)
                                    if match:
                                        print(f"Extracted field: {match.group(1)}")
                                        extracted_field = match.group(1)

                                    else:
                                        print("No match found")


                                    for field in fields:#`ctx . accounts .` の後のフィールド名(match.group(1))が構造体に存在する場合→edgeを追加したいところ(1/31)
                                        if match.group(1) == field["name"]:
                                            field_type = field.get("field_type", "")
                                            match_type = re.search(r"<\s*'info\s*,\s*(\w+)\s*>", field_type)
                                            if match_type:
                                                account_name = match_type.group(1)  # < 'info ,  > の部分
                                                print(f"Extracted account type: {account_name}")
                                            else:
                                                account_name = None
                                                print("No account type found in field_type.") 

                                    print(rf"define(動作確認8)= {define}")

                            elif "data_dep" in edge["label"]:
                                depended_val = re.search(r"data_dep:\s*(\w+)", edge["label"]).group(1)
                                if depended_val in define:
                                    print("動作確認2")
                                    print(f"depended_val: {depended_val}")
                                    #for has_one_relation in all_has_one_relations:

                                    #for field_name, items in has_one_relation.items():

                                    patterns = [
                                        rf"{field_name}\s*=",
                                        rf"{depended_val}\s*\.\s*to_account_info\s*\(\s*\)\s*\.\s*try_borrow_mut_lamports\s*\(\s*\)\s*\?",
                                        rf"{depended_val}\s*\.\s*to_account_info\s*\(\s*\)\s*\.\s*try_borrow_data\s*\(\s*\)",
                                        rf"{depended_val}\s*\.\s*key\s*\(\s*\)",
                                        rf"{depended_val}\s*\.\s*owner",
                                        rf"{depended_val}\s*\.\s*lamports\s*\(\s*\)",
                                        rf"{depended_val}\s*\.\s*\w+\s*\(\s*\)"
                                    ]

                                    for pattern in patterns:
                                        print(f"Checking pattern: {pattern} against label: {label}")

                                        if re.search(pattern, label):
                                            print("動作確認3")
                                            update = True
                                            related_field = field_name
                                            print(f"update(動作確認3) = {update}, related_field = {related_field}")
                                            print(pattern)
                                            return update, related_field 
                                            
                                        else:
                                            print("動作確認5")
                                            depend,account_name = analysis(label, depended_val, depend,fields)
                                            if depend and i + 1 < len(all_pdg_files):
                                                print(f"depend(動作確認5) = {depend}")
                                                update, related_field = check_field_updates(all_pdg_files[i + 1], all_pdg_files, all_has_one_relations, init_structs, i + 1,define, update, depend,all_structs,account_name,field_name)
                                                if update == True:
                                                    return update, related_field

                            if account_name:
                                for st_dict2 in structs:
                                    if st_dict2["struct_name"] == account_name: #<>内にあるアカウント名と一致するアカウントの内部を調べる
                                        fields2 = st_dict2["fields"]
                                        for field2 in fields2:
                                            if field2["field_type"] == "Pubkey":
                                                field_name = field2["name"]  
                                                print(f"field_name(動作確認) = {field_name}")                              



                else:
                    #エッジがない場合の処理(後々カスタマイズはする)
                    depend,account_name = analysis(label,None,None,fields)
                    related_field = depend[0]

                

                            #update =True

    return update, related_field


def analysis(label, depended_val, depend,fields):
    """
    ctx.accounts.の右側にある構造体名を抽出して反映します。

    """

    account_name = None

    if "=" in label:
        left, right = label.split("=")
        left_list = left.split(".")
        print(rf"label = {label}")
        print(rf"left_list[1]={left_list[1]}")
        print(rf"depended_val={depended_val}")
        print(rf"left_list[0].strip()={left_list[0].strip()}")
        if left_list[0].strip() == depended_val:
            if "program" in left_list[1]:#programと書かれていない場合もある
                print("動作確認4")
                print(left_list[1])
                depend = True

        if "ctx . accounts" in left:#機能が被っていそうなところが118行目以降にある(1/31)
            #depend = left_list[2:]
            match = re.search(r"ctx\s*\.\s*accounts\s*\.\s*(\w+)",left)
            for field in fields:
                if match.group(1) == field["name"]:
                    field_type = field.get("field_type", "")
                    match_type = re.search(r"<\s*'info\s*,\s*(\w+)\s*>", field_type)
                    if match_type:
                        account_name = match_type.group(1)  # AdminConfig の部分
                        print(f"Extracted account type: {account_name}")
                    else:
                        print("No account type found in field_type.")

    return depend,account_name

# def check_signer(ast_data, field_name,signer_check):
#     """
#     ASTデータを解析し、指定したフィールドに `signer` が付いているか確認します。
#     """
#     for item in ast_data:
#         if item.get("node_type") == "struct":
#             for field in item.get("fields", []):
#                 if field["name"] == field_name:
#                     attribute = field.get("attribute") or ""
#                     if "signer" in attribute or "Signer" in field.get("field_type", ""):
#                        signer_check = True
                
#     return signer_check

def check_signer(all_structs, field_name,signer_check):
    #各ファイルのstructを処理
    for structs in all_structs:
        #構造体を一つずつ処理
        for struct in structs:
            for field in struct["fields"]:
                if field_name and field["name"] == field_name:
                    attribute = field.get("attribute") or ""
                    if "signer" in attribute or "Signer" in field.get("field_type",""):
                        signer_check = True
                        print(f"signer_check(動作確認) = {signer_check}, field_type(動作確認) = {field['field_type']}") 
                        return signer_check
                    print(f"signer_check(動作確認) = {signer_check}, field_type(動作確認) = {field['field_type']}") 

                #print(f"field_name(動作確認): {field_name}")#2/3

    return signer_check


# def check_owner(ast_data, field_name,owner_check):
#     for item in ast_data:
#         if item.get("node_type") == "struct":
#             for field in item.get("fields", []):
#                 if "AccountInfo" in field["field_type"]:
#                     owner_check = True 

#                 print(f"owner_check(動作確認) = {owner_check}, field_type(動作確認) = {field['field_type']}")  
                        
#     return owner_check


def check_owner(all_structs, field_name,owner_check):
    for structs in all_structs:
        #構造体を一つずつ処理
        for struct in structs:
            for field in struct["fields"]:
                if "AccountInfo" in field["field_type"]:
                    owner_check = True 
                    print(f"owner_check(動作確認) = {owner_check}, field_type(動作確認) = {field['field_type']}") 
                    return owner_check        
                print(f"owner_check(動作確認) = {owner_check}, field_type(動作確認) = {field['field_type']}") 
    return owner_check
    

import os

def load_json_files(directory, keyword):
    """指定されたディレクトリから、特定のキーワードを含むJSONファイルを昇順で読み込む"""
    files = []
    for filename in os.listdir(directory):
        if keyword in filename and filename.endswith(".json"):
            filepath = os.path.join(directory, filename)
            files.append(filepath)
    
    # ファイル名の数値部分を考慮してソート
    files.sort(key=lambda x: int(''.join(filter(str.isdigit, os.path.basename(x))) or 0))
    
    return files


def parse_all_files(ast_files, pdg_files):
    """すべてのファイルを解析して構造を格納"""
    all_structs = []
    all_has_one_relations = []
    all_pdg_files = []
    init_structs = []
    update = False
    has_one = False
    depend = None

    for ast_file, pdg_file in zip(ast_files, pdg_files):
        with open(ast_file, "r") as f:
            ast_data = json.load(f)
        with open(pdg_file, "r") as f:
            pdg_data = json.load(f)

        structs = parse_ast(ast_data)
        #structsはリスト構造ですが、一つ一つの要素は辞書です
        #all_structs.append({ast_file:structs})
        #リスト型に変更
        all_structs.append(structs)
        all_pdg_files.append(pdg_data)
        init_structs.append([init_struct for init_struct in structs if init_struct["is_init_struct"]])
        has_one_relations = find_has_one_field(structs)
        #has_one_relationsには1ASTファイルのhas_oneが付いたfieldを格納
        all_has_one_relations.append(has_one_relations)
        #all_has_one_relationsに全ファイルのhas_oneが付いたfieldを格納

    update, related_field = check_field_updates(all_pdg_files[0], all_pdg_files, all_has_one_relations, init_structs, 0,[], update, depend,all_structs,None,None)
    print(f"update(動作確認) = '{update}'" )
    
    if related_field in all_has_one_relations:
        has_one = True
    
    is_signer = check_signer(all_structs, related_field,False)
    is_owner = check_owner(all_structs, related_field,False)
    print(f"all_has_one_relations:{all_has_one_relations}")
    print(f"has_one:{has_one}")
    if not is_signer and update and has_one:
        print(f"[INFO] Field '{related_field}' is updated.")
        print(f"[WARNING]脆弱性を検知しました。 Field '{related_field}' が 'signer' attributeを正常に付与されていません.")
    
    elif is_owner:
        print(f"[WARNING]脆弱性を検知しました。AccountInfoが存在します。これは所有者チェックの不備を招く可能性があります。確認してください。")   

    elif is_signer and update and not has_one:
        print(f"[WARNING]脆弱性を検知しました。 Field '{related_field}' が 'has_one' attributeを正常に付与されていません.")    
        
    else:
        print(f"[INFO]脆弱性はありません")
        print(f"update(動作確認) is '{update}'" )
        print(f"is_owner(動作確認) is '{is_owner}'" )
        
    
    return all_structs, all_has_one_relations

def main():
    project1_dir = "./programs/insecure_update_project2/Project1"
    ast_files = load_json_files(project1_dir, "ast")
    pdg_files = load_json_files(project1_dir, "pdg")
    print(f"ast_files:{ast_files}")
    print(f"pdg_files:{pdg_files}")
    all_structs, all_has_one_relations = parse_all_files(ast_files, pdg_files)

    # print("[INFO] All structs:")
    # for filename, structs in all_structs.items():
    #     print(f"{filename}: {structs}")

    # print("[INFO] All has_one relations:")
    # for has_one in all_has_one_relations:
    #     print(has_one)

if __name__ == "__main__":
    main()

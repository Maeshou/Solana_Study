import json
import re
import os

def parse_ast(ast_data):
    """
    ASTデータを解析し、必要な情報を収集します。
    """
    structs = {}
    for item in ast_data:
        if item.get("node_type") == "struct":
            struct_name = item["name"]
            fields = item.get("fields", [])
            structs[struct_name] = {
                "fields": fields,
                "is_init_struct": any("init" in (field.get("attribute") or "") for field in fields)
            }
    return structs

def find_patterns(structs):
    """
    has_one= を含むフィールドを探し、対応するフィールドが存在するか確認します。
    """
    related_field_names = {}
    for struct_name, struct_info in structs.items():
        for field in struct_info["fields"]:
            attribute = field.get("attribute", "")
            field_type = field.get("field_type","")
            if isinstance(attribute, str):  # 文字列型か確認
                match = re.search(r"has_one\s*=\s*(\w+)", attribute)
                if match:
                    related_field_name = match.group(1)
                    if related_field_name in [f["name"] for f in struct_info["fields"]]:
                        related_field_names[field["name"]] = [
                            f for f in struct_info["fields"] if f["name"] == related_field_name
                        ]
                    else:
                        print(f"[WARNING] Related struct '{related_field_name}' not found for field '{field['name']}' in struct '{struct_name}'.")
            else:
                print(f"[INFO] Skipping non-string attribute: {attribute}")



            


    return related_field_names

def check_field_updates(pdg_data, all_pdg_files, all_has_one_relations, init_structs, i, define, update, depend):
    """
    PDGデータを解析し、指定したフィールドが更新されているか確認します。
    """
    
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

        print(f"struct_name: {struct_name}")
        print(f"init_structs:{init_structs}")

        # init_structsをフラット化
        flat_init_structs = [item for sublist in init_structs for item in sublist]
        print(f"flat_init_structs: {flat_init_structs}")
        if struct_name and struct_name not in flat_init_structs:
            print(f"struct_name1: {struct_name}")
            for node in nodes:
                label = node.get("label", "")
                node_id = node.get("id")
                if edges:
                    for edge in edges:
                        if "to" in edge and node_id == edge["to"]:
                            print(rf"動作確認9:node_id= {node_id}")
                            print(rf"動作確認9:edge['to'] = {edge['to']}")
                            if "def" in edge["label"]:
                                defined_val = re.search(r"def:\s*(\w+)", edge["label"]).group(1)
                                print(rf"defined_val(動作確認7)= {defined_val}")
                                if defined_val not in define:
                                    define.append(defined_val)
                                    print(rf"define(動作確認8)= {define}")
                            elif "data_dep" in edge["label"]:
                                depended_val = re.search(r"data_dep:\s*(\w+)", edge["label"]).group(1)
                                if depended_val in define:
                                    print("動作確認2")
                                    print(f"depended_val: {depended_val}")
                                    for has_one_relation in all_has_one_relations:
                                        for field_name, items in has_one_relation.items():
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
                                                    depend = analysis(label, depended_val, depend)
                                                    if depend == "depends" and i + 1 < len(all_pdg_files):
                                                        print(f"depend(動作確認5) = {depend}")
                                                        update, related_field = check_field_updates(all_pdg_files[i + 1], all_pdg_files, all_has_one_relations, init_structs, i + 1,define, update, depend)
                                                        if update == True:
                                                            return update, related_field

                else:
                    #エッジがない場合の処理(後々カスタマイズはする)
                    depend = analysis(label,"none","none")
                    related_field = depend[0]

    return update, related_field


def analysis(label, depended_val, depend):
    """
    ctx.accounts.の右側にある構造体名を抽出して反映します。
    """
    if "=" in label:
        left, right = label.split("=")
        left_list = left.split(".")
        print(rf"label = {label}")
        print(rf"left_list[1]={left_list[1]}")
        print(rf"depended_val={depended_val}")
        print(rf"left_list[0].strip()={left_list[0].strip()}")
        if left_list[0].strip() == depended_val:
            if "program" in left_list[1]:
                print("動作確認4")
                print(left_list[1])
                depend = "depends"

        if "ctx . accounts" in left:
            depend = left_list[2:]


    return depend

def check_signer(ast_data, field_name,signer_check,owner_check):
    """
    ASTデータを解析し、指定したフィールドに `signer` が付いているか確認します。
    """
    for item in ast_data:
        if item.get("node_type") == "struct":
            for field in item.get("fields", []):
                if field["name"] == field_name:
                    attribute = field.get("attribute") or ""
                    if "signer" in attribute or "Signer" in field.get("field_type", ""):
                       signer_check = True
                if "AccountInfo" in field["field_type"]:
                    owner_check = True 

                print(f"owner_check(動作確認) = {owner_check}, field_type(動作確認) = {field['field_type']}")  
                        
    return signer_check,owner_check

def load_json_files(directory, keyword):
    """指定されたディレクトリから、特定のキーワードを含むJSONファイルを読み込む"""
    files = []
    for filename in os.listdir(directory):
        if keyword in filename and filename.endswith(".json"):
            filepath = os.path.join(directory, filename)
            files.append(filepath)
    return files

def parse_all_files(ast_files, pdg_files):
    """すべてのファイルを解析して構造を格納"""
    all_structs = {}
    all_has_one_relations = []
    all_pdg_files = []
    init_structs = []
    update = False
    depend = "none"
    for ast_file, pdg_file in zip(ast_files, pdg_files):
        with open(ast_file, "r") as f:
            ast_data = json.load(f)
        with open(pdg_file, "r") as f:
            pdg_data = json.load(f)

        structs = parse_ast(ast_data)
        all_structs[ast_file] = structs
        all_pdg_files.append(pdg_data)

        init_structs.append([name for name, info in structs.items() if info["is_init_struct"]])
        has_one_relations = find_patterns(structs)
        all_has_one_relations.append(has_one_relations)

    update, related_field = check_field_updates(all_pdg_files[0], all_pdg_files, all_has_one_relations, init_structs, 0,[], update, depend)
    print(f"update(動作確認) = '{update}'" )
    
    for ast_file, structs in all_structs.items():
        is_signer,is_owner = check_signer(json.load(open(ast_file, "r")), related_field,False, False)
        if not is_signer and update:
            print(f"[INFO] Field '{related_field}' is updated.")
            print(f"[WARNING]脆弱性を検知しました。 Field '{related_field}' が 'signer' attributeを正常に付与されていません.")
        
        elif is_owner:
            print(f"[WARNING]脆弱性を検知しました。AccountInfoが存在します。これは所有者チェックの不備を招く可能性があります。確認してください。")   
            
        else:
            print(f"[INFO]脆弱性はありません")
            print(f"update(動作確認) is '{update}'" )
            print(f"is_owner(動作確認) is '{is_owner}'" )
            
    
    return all_structs, all_has_one_relations

def main():
    project1_dir = "./programs/insecure_update_project2/Project3"
    ast_files = load_json_files(project1_dir, "ast")
    pdg_files = load_json_files(project1_dir, "pdg")

    all_structs, all_has_one_relations = parse_all_files(ast_files, pdg_files)

    # print("[INFO] All structs:")
    # for filename, structs in all_structs.items():
    #     print(f"{filename}: {structs}")

    # print("[INFO] All has_one relations:")
    # for has_one in all_has_one_relations:
    #     print(has_one)

if __name__ == "__main__":
    main()

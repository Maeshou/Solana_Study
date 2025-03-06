import json
import re

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

def find_has_one_relations(structs):
    """
    `has_one=` を含むフィールドを探し、対応するフィールドが存在するか確認します。
    """
    related_field_names = {}
    related_field_name = "none"
    for struct_name, struct_info in structs.items():
        for field in struct_info["fields"]:
            attribute = field.get("attribute") or ""
            match = re.search(r"has_one\s*=\s*(\w+)", attribute)
            if match:
                related_field_name = match.group(1)
                # 対応する構造体（例: 'Vault'）に同名のフィールドが存在するか確認
                print(related_field_name)
            
                if related_field_name in [f["name"] for f in struct_info["fields"]]:
                    print("exist")
                    
                    # フィールド名のリストを取得
                    related_field_names[field["name"]] = [f for f in struct_info["fields"] if f["name"] == related_field_name] 
                    
                    print(related_field_names)
                else:
                    print(f"[WARNING] Related struct '{related_field_name}' not found for field '{field['name']}' in struct '{struct_name}'.")

            # else:   
            #     #他のauthorityフィールドもhas_one_relationsに追加
            #     # if related_field_name != "none":
            #     #     if related_field_names[related_field_name] != [f for f in struct_info["fields"]] and related_field_name in [f["name"] for f in struct_info["fields"]]:
            #     #         related_field_names[related_field_name].append(f)
            #     #         print(related_field_names[related_field_name])
            #     else:
            #         continue    


    return related_field_names

def check_field_updates(pdg_data, field_name):
    """
    PDGデータを解析し、指定したフィールドが更新されているか確認します。
    """
    updates = []
    for function in pdg_data:
        function_name = function["name"]
        nodes = function.get("nodes", [])
        for node in nodes:
            label = node.get("label", "")

            
            if re.search(rf"{field_name}\s*=", label):
                # field_name が左辺（更新される側）として使われている場合
                update = True
                
    return update

def check_signer(ast_data, field_name):
    """
    ASTデータを解析し、指定したフィールドに `signer` が付いているか確認します。
    """
    for item in ast_data:
        if item.get("node_type") == "struct":
            for field in item.get("fields", []):
                if field["name"] == field_name:
                    attribute = field.get("attribute") or ""
                    if "signer" in attribute or "Signer" in field.get("field_type", ""):
                        return True

                        
    return False

   
def main():
    # JSONデータを読み込み
    with open("ast_lib.json", "r") as f:
        ast_data = json.load(f)
    with open("pdg_lib.json", "r") as f:
        pdg_data = json.load(f)

    # ASTデータを解析
    structs = parse_ast(ast_data)

    # 初期化構造体を特定
    init_structs = [name for name, info in structs.items() if info["is_init_struct"]]
    print(f"[INFO] Initialization structs: {init_structs}")

    # has_one= の関係を特定 (辞書型データ)
    has_one_relations = find_has_one_relations(structs)

    # has_one= の右側の値を更新している箇所を確認し、signer が付いているか確認
    # has_one_relations は { field_name: [related_fields], ... } の形を想定
    for field_name, related_fields in has_one_relations.items():
        if not related_fields:
            continue
        # related_fieldsは関連するフィールド情報のリスト
        # ここでは先頭要素を使用
        
        related_field = related_fields[0]["name"]

        # フィールドが更新されているか確認
        update = check_field_updates(pdg_data, related_field)
        if update:
            print(f"[INFO] Field '{related_field}' is updated in the following locations:")
            
            # signer が付いているか確認
            is_signer = check_signer(ast_data, related_field)
            if is_signer:
                print(f"[INFO]脆弱性はありません。 Field '{related_field}' が 'signer' attributeを正常に付与されています.")
            else:
                print(f"[WARNING]脆弱性を検知しました。 Field '{related_field}' が 'signer' attributeを正常に付与されていません.")
        else:
            print(f"[INFO] (Field '{related_field}' is not updated in the code.")
 
if __name__ == "__main__":
    main()

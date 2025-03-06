import json
import numpy as np
from gensim.models import Word2Vec

# JSONファイルを読み込む関数
def load_json(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        return json.load(f)

# トークン化
def tokenize(text):
    return text.split()  # 空白でトークン化（簡易版）

# 単語ごとにベクトルを取得
def vectorize_label(label, model):
    tokens = tokenize(label)
    word_vectors = [
        model.wv[token].tolist() if token in model.wv else [0.0] * model.vector_size
        for token in tokens
    ]
    return word_vectors

# JSONデータをベクトル化
def vectorize_json(data, model):
    def traverse_and_vectorize(obj):
        if isinstance(obj, dict):
            for key, value in obj.items():
                if key == "label" and isinstance(value, str):
                    obj[key] = vectorize_label(value, model)  # 単語ごとにベクトル化
                elif isinstance(value, (dict, list)):
                    traverse_and_vectorize(value)
        elif isinstance(obj, list):
            for item in obj:
                traverse_and_vectorize(item)

    traverse_and_vectorize(data)
    return data

# JSONデータを保存
def save_json(file_path, data):
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=4, ensure_ascii=False)

# メイン処理
def main():
    input_path = "/home/maeshou/insecure_update_project2/programs/insecure_update_project2/Project2/pdg1.json"
    output_path = "/home/maeshou/insecure_update_project2/programs/insecure_update_project2/Project2/pdg1_vectorized.json"
    # JSONデータのロード
    data = load_json(input_path)

    # サンプルデータからWord2Vecモデルを学習
    texts = [
        "let account_data = ctx . accounts . admin_config . try_borrow_data () ? ;",
        "let mut account_data_slice : & [u8] = & account_data ;",
        "let account_state = AdminConfig :: try_deserialize (& mut account_data_slice) ? ;",
        "if account_state . admin != ctx . accounts . admin . key () { return Err (ProgramError :: InvalidArgument . into ()) ; }"
    ]
    tokenized_texts = [tokenize(text) for text in texts]
    model = Word2Vec(sentences=tokenized_texts, vector_size=3, window=5, min_count=1, workers=4)

    # JSONデータをベクトル化
    vectorized_data = vectorize_json(data, model)

    # ベクトル化されたJSONを保存
    save_json(output_path, vectorized_data)
    print(f"ベクトル化されたJSONを保存しました: {output_path}")

if __name__ == "__main__":
    main()

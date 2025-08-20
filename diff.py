import numpy as np, json


ROOT_DIR = "./programs/Projects"
for case in ["case_020", "case_005"]:
    f0 = np.load(f"{ROOT_DIR}/new_0/dataset/graphs/{case}/vectors.npy")
    f1 = np.load(f"{ROOT_DIR}/new_1/dataset/graphs/{case}/vectors.npy")
    print(case, "vectors identical?", np.allclose(f0, f1), "|| L2 diff:", np.linalg.norm(f0 - f1))

    j0 = json.load(open(f"{ROOT_DIR}/new_0/dataset/graphs/{case}/third_joint_graph.json"))
    j1 = json.load(open(f"{ROOT_DIR}/new_1/dataset/graphs/{case}/third_joint_graph.json"))
    print(case, "graph json identical?", j0 == j1)

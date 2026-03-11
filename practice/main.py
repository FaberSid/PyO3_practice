from dataclasses import dataclass
import practice

# 1. dataclass を定義（Rust側の構造体とフィールド名を合わせるだけ）
@dataclass
class User:
    id: int
    name: str

# 2. 普通にインスタンス化
user_data = User(id=123, name="Gopher-kun")

# 3. Rustの関数を呼び出す（引数の型は自動で UserData に変換される）
result = practice.check_user(user_data)
print(result)

# 4. 実は辞書(dict)でも通る（pythonizeを使っている場合）
result_from_dict = practice.check_user({"id": 456, "name": "Rust-acean"})
print(result_from_dict)

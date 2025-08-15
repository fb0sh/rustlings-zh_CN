#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).
// TODO: 修复编译错误，仅通过添加或移除引用符号（字符 `&`），不修改其他内容。

// Shouldn't take ownership
// 不应该获取所有权
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 应该获取所有权
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

#![allow(clippy::ptr_arg)]

// TODO: 修复编译器错误，只能添加或删除引用（字符 `&`）。

// 不应该获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

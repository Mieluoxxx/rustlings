// 这个函数的调用应该被替换为调用 `string_slice` 或 `string`。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 这里有一堆值 - 有些是 `String`，有些是 `&str`。
// 你的任务是根据你认为每个值的类型，将 `placeholder(…)` 替换为
// `string_slice(…)` 或 `string(…)`。
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // 警告：这是字节索引，不是字符索引。
    // 字符索引可以使用 `s.chars().nth(INDEX)` 来完成。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

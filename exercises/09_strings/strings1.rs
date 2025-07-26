// TODO: 在不改变函数签名的情况下修复编译错误。
fn current_favorite_color() -> String {
    String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}

// Rust 编译器需要知道如何检查提供的引用是否有效，
// 以便它可以让程序员知道引用是否有在被使用之前
// 就超出作用域的风险。记住，引用是借用，
// 并不拥有它们自己的数据。如果它们的所有者超出了作用域会怎样？

// TODO: 通过更新函数签名来修复编译器错误。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
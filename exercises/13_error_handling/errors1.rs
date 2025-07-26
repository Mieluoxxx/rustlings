// TODO: 这个函数在传递空字符串时拒绝生成要打印在名牌上的文本。
// 如果它能解释问题所在而不是仅仅返回 `None`，那就更好了。
// 幸运的是，Rust 有一个类似于 `Option` 的结构，可以用来表示错误条件。
// 将函数签名和函数体更改为返回 `Result<String, String>` 而不是 `Option<String>`。
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 不允许使用空名称
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            Ok("Hi! My name is Beyoncé".to_string()),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new()),
            Err("`name` was empty; it must be nonempty.".into()),
        );
    }
}
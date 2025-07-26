fn trim_me(input: &str) -> &str {
    // TODO: 从字符串的两端删除空格。
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: 在字符串后添加 " world!"！有多种方法可以做到这一点。
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: 将字符串中的 "cars" 替换为 "balloons"。
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}

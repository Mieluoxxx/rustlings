fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 将其变为一个值为 `Some` 的 if-let 语句。
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 将其变为一个 while-let 语句。记住 `Vec::pop()`
        // 会添加另一层 `Option`。你可以在 if-let 和 while-let 语句中
        // 进行嵌套模式匹配。
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
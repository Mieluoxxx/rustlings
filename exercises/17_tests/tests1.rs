// 测试对于确保你的代码按预期工作非常重要。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO: 导入 `is_even`。你可以使用通配符导入
    // 外部模块中的所有内容。

    #[test]
    fn you_can_assert() {
        // TODO: 使用一些值测试 `is_even` 函数。
        assert!();
        assert!();
    }
}
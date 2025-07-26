// 使用位移计算 2 的幂。
// `1 << n` 等同于 "2 的 n 次方"。
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: 使用一些值测试 `power_of_2` 函数。
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
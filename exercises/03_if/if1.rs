fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完成这个函数以返回更大的数字！
    // 如果两个数字相等，可以返回其中任意一个。
    // 不要使用：
    // - 其他函数调用
    // - 额外的变量
    if a >= b { a } else { b }
}

fn main() {
    // 你可以选择在这里进行实验。
}

// 现在不用担心这个 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

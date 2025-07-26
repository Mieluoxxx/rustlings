// 这个函数返回冰箱里还剩下多少冰淇淋。
// 如果时间在 22:00（24 小时制）之前，那么还剩下 5 勺。到了 22:00，
// 有人把它全吃光了，所以没有冰淇淋了（值为 0）。如果
// `hour_of_day` 大于 23，则返回 `None`。
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: 完成函数体。
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day <= 23 {
        Some(0)
    } else {
        None
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。你如何获取 Option 中包含的值？
        let ice_creams = maybe_ice_cream(12).unwrap();
        assert_eq!(ice_creams, 5); // 不要改变这一行。
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
// 这是针对以下部分的测验：
// - 变量
// - 函数
// - If语句
//
// 玛丽正在买苹果。一个苹果的价格计算如下：
// - 一个苹果花费 2 rustbucks。
// - 但是，如果玛丽购买超过 40 个苹果，整个订单中每个苹果的价格将降至仅 1 rustbuck！

// TODO: 编写一个函数，根据购买的数量计算苹果订单的价格。
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // 你可以在这里进行可选的实验。
}

// 不要更改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}

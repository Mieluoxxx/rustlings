// 需要定义一个以哈希映射形式存在的水果篮。键
// 代表水果的名称，值代表篮子中该
// 特定水果的数量。你必须在篮子中放入至少 3 种不同
// 类型的水果（例如苹果、香蕉、芒果），并且所有水果的总数
// 至少为 5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明哈希映射。
    let mut basket = HashMap::new();

    // 已经为你准备好了两个香蕉 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在你的篮子里放入更多水果。
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
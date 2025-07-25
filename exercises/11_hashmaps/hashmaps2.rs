// 我们正在收集不同的水果来制作一个美味的水果蛋糕。为此，
// 我们有一个篮子，我们将以哈希映射的形式表示它。键
// 代表我们收集的每种水果的名称，值代表我们
// 收集的该特定水果的数量。三种类型的水果 -
// 苹果 (4)、芒果 (2) 和荔枝 (5) 已经存在于篮子哈希映射中。你
// 必须向篮子中添加水果，以使每种水果至少有一个，并且
// 总数超过 11 个 - 我们有很多张嘴要喂。不允许你
// 插入任何已经存在于篮子中的水果（苹果、
// 芒果和荔枝）。

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Banana,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: 如果篮子中尚不存在新水果，则插入它们。
        // 请注意，不允许放入任何已经存在的水果类型！
        basket.entry(fruit).or_insert(1);
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    // 不要修改这个函数！
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}
// 在对集合中的元素执行操作时，迭代器是必不可少的。
// 本模块帮助你熟悉使用迭代器的结构，以及如何遍历
// 可迭代集合中的元素。

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: 创建一个数组的迭代器。
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: 替换 `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: 替换 `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: 替换 `todo!()`
    }
}
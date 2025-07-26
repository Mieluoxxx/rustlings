fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: 使用元组索引来访问 `numbers` 的第二个元素
        // 并将其赋值给一个名为 `second` 的变量。
        // let second = ???;
        let second = numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 从数组 `a` 中获取一个名为 `nice_slice` 的切片，使测试通过。
        // let nice_slice = ???
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);
    }
}

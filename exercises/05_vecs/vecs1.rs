fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 数组

    // TODO: 创建一个名为 `v` 的向量，其中包含与数组 `a` 中完全相同的元素。
    // 使用向量宏。
    // let v = ???;
    let v = vec![10, 20, 30, 40];
    (a, v)
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}

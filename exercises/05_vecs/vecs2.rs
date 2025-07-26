fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: 将输入切片中的每个元素乘以 2，并将结果推入
        // `output` 向量中。
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 一个在映射后收集向量的例子。
    // 我们将输入切片的每个元素映射为其值加 1。
    // 如果输入是 `[1, 2, 3]`，输出就是 `[2, 3, 4]`。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: 在这里，我们也想将输入切片中的每个元素乘以 2，
    // 但是使用迭代器映射而不是手动推入到空向量中。
    // 参考上面函数 `vec_map_example` 中的例子。
    input
        .iter()
        .map(|element| {
            element * 2
        })
        .collect()
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}

// TODO: 修复函数中的编译器错误，不要添加任何新行。
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

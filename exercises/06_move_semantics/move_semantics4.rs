fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO: 仅通过重新排列测试中的行来修复编译器错误。
    // 不要添加、更改或删除任何行。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

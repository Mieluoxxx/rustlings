// 这个强大的包装器提供了存储一个正整数值的能力。
// TODO: 使用泛型重写它，使其支持包装任何类型。
struct Wrapper {
    value: u32,
}

// TODO: 修改结构体的实现，使其在包装值上是泛型的。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
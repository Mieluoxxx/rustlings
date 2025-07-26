// `Vec<T>` 是泛型，其类型参数为 `T`。在大多数情况下，编译器能够
// 推断出 `T` 的类型，例如在向 vector 推入一个具体类型的值之后。
// 但在这个练习中，编译器需要通过类型注解来获得一些帮助。

fn main() {
    // TODO: 通过为 vector `Vec<T>` 添加类型注解来修复编译器错误。
    // 选择一个可以从 `u8` 和 `i8` 创建的整数类型作为 `T`。
    let mut numbers = Vec::new();

    // 不要改变下面的代码行。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
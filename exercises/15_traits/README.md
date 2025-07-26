# Trait

Trait 是方法的集合。

数据类型可以实现 trait。为此，构成 trait 的方法是为数据类型定义的。例如，`String` 数据类型实现了 `From<&str>` trait。这允许用户编写 `String::from("hello")`。

通过这种方式，trait 有点类似于 Java 接口和 C++ 抽象类。

一些其他常见的 Rust trait 包括：

- `Clone` (`clone` 方法)
- `Display` (允许通过 `{}` 进行格式化显示)
- `Debug` (允许通过 `{:?}` 进行格式化显示)

因为 trait 表示数据类型之间的共享行为，所以它们在编写泛型时很有用。

## 更多信息

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
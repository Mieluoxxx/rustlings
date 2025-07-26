# 可选项

`Option` 类型表示一个可选值：每个 `Option` 要么是 `Some` 并包含一个值，要么是 `None`，不包含任何值。
`Option` 类型在 Rust 代码中非常常见，因为它们有多种用途：

- 初始值
- 对于没有在其整个输入范围上定义的函数（部分函数）的返回值
- 用于报告简单错误的返回值，其中在出错时返回 `None`
- 可选的结构体字段
- 可以借出或“拿走”的结构体字段
- 可选的函数参数
- 可空指针
- 在困难情况下交换事物

## 更多信息

- [Option 枚举格式](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
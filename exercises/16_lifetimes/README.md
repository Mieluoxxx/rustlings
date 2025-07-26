# 生命周期

生命周期告诉编译器如何检查引用是否存活得足够长，以便在任何给定的情况下都有效。例如，生命周期会说“确保参数 'a' 的存活时间与参数 'b' 一样长，以便返回值有效”。

它们仅在借用（即引用）时是必需的，因为复制的参数或移动在其作用域内拥有所有权，并且不能在外部被引用。生命周期意味着可以检查例如函数的调用代码，以确保其参数是有效的。生命周期对其调用者是限制性的。

如果你想了解更多关于生命周期注解的信息，[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目的练习风格与 Rustlings 类似，但全部是关于学习编写生命周期注解的。

## 更多信息

- [生命周期 (在 Rust By Example 中)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
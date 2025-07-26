// 在编译时，Rust 需要知道一个类型占用了多少空间。
// 这对于递归类型来说是个问题，因为一个值可以包含
// 另一个相同类型的值作为其一部分。为了解决这个问题，我们可以使用
// `Box`——一个用于在堆上存储数据的智能指针，它也允许我们
// 包装一个递归类型。
//
// 我们在这个练习中实现的递归类型是“cons list”，
// 一种在函数式编程语言中常见的数据结构。cons list 中的每个
// 项包含两个元素：当前项的值和下一个项。
// 最后一项是一个名为 `Nil` 的值。

// TODO: 在枚举定义中使用 `Box` 来使代码编译通过。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 创建一个空的 cons list。
fn create_empty_list() -> List {
    todo!()
}

// TODO: 创建一个非空的 cons list。
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
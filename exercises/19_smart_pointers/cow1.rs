// 本练习探讨 `Cow` (Clone-On-Write) 智能指针。它可以
// 包含并提供对借用数据的不可变访问，并在需要
// 可变性或所有权时延迟克隆数据。该类型旨在通过
// `Borrow` trait 与通用的借用数据一起工作。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果尚未拥有，则克隆到一个向量中。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 发生克隆是因为 `input` 需要被改变。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 没有发生克隆，因为 `input` 不需要被改变。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // 我们也可以不带 `&` 传递 `vec`，这样 `Cow` 就直接拥有它。在这种情况下，
        // 不会发生改变（所有数字都已经是绝对值），因此也不会发生克隆。
        // 但结果仍然是拥有的，因为它从未被借用或改变。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // 当然，如果确实发生了改变（并非所有数字都是绝对值），情况也是如此。
        // 在这种情况下，`abs_all` 函数中对 `to_mut()` 的调用返回
        // 一个与之前相同数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }
}
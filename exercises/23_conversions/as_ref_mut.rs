// AsRef 和 AsMut 允许廉价的引用到引用转换。
// 分别在 https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html 阅读更多关于它们的信息。

// 获取给定参数中的字节数（而不是字符数）
// (`.len()` 返回字符串中的字节数）。
// TODO: 适当地添加 `AsRef` trait 作为 trait bound。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数中的字符数（而不是字节数）。
// TODO: 适当地添加 `AsRef` trait 作为 trait bound。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 `as_mut()` 对数字进行平方。
// TODO: 添加适当的 trait bound。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() *= *arg.as_mut();
}

fn main() {
    // 您可以选择在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

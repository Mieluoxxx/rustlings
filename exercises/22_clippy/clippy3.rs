// 这里有一些更简单的 Clippy 修复，这样你就可以看到它的用途了 📎
// TODO: 修复所有 Clippy lints。

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // 假设您不知道 `my_option` 的值。
    // 在 `Some` 的情况下，我们想要打印它的值。
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("我的数组！在这里：{my_arr:?}");

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("这个 Vec 是空的，看到了吗？{my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 让我们交换这两个！
    value_a = value_b;
    value_b = value_a;
    println!("值 a: {value_a}; 值 b: {value_b}");
}

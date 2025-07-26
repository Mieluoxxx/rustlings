fn main() {
    my_macro!();
}

// TODO: 通过移动此宏的整个定义来修复编译器错误。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

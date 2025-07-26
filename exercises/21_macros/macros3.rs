// TODO: 修复编译器错误，不要将宏定义移出此模块。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}

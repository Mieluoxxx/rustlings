// 字符类型 (`char`)

fn main() {
    // 注意这里使用的是单引号，这与你之前看到的双引号是不同的。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: 类似于上面的例子，在下面声明一个名为 `your_character` 的变量，
    // 使用你最喜欢的字符。
    // 试试字母，试试数字（用单引号），试试特殊字符，试试与你不同语言的字符，试试表情符号 😉
    // let your_character = '';
    let your_character = "🐱";
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

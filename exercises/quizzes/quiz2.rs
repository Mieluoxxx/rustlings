// 这是针对以下部分的测验：
// - 字符串
// - 向量
// - Move 语义
// - 模块
// - 枚举
//
// 让我们以函数的形式构建一个小机器。作为输入，我们将提供一个字符串和命令的列表。这些命令决定了将对字符串应用什么操作。它可以是：
// - 将字符串转换为大写
// - 修剪字符串
// - 将“bar”附加到字符串指定的次数
//
// 这将是具体的形式：
// - 输入将是一个包含2元组的向量，第一个元素是字符串，第二个是命令。
// - 输出元素将是一个字符串向量。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO：如上所述完成函数。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO：我们需要导入什么才能使`transformer`在作用域内？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

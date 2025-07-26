// 你可以使用 `use` 关键字将模块路径从任何地方的模块，
// 特别地从标准库中带入你的作用域。

// TODO: 从 `std::time` 模块中将 `SystemTime` 和 `UNIX_EPOCH` 带入
// 你的作用域。如果你能用一行完成，还能得到额外的风格加分！
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

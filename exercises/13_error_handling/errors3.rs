// 这是一个试图使用前一个练习中 `total_cost` 函数的完整版本的程序。
// 但它不工作！为什么呢？我们应该怎么做来修复它？

use std::num::ParseIntError;

// 不要改变这个函数。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: 通过更改 `main` 函数的签名和主体来修复编译器错误。
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 不要改变这一行。
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }

    Ok(())
}
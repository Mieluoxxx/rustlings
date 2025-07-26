// 假设我们正在编写一个游戏，你可以用代币购买物品。所有物品都花费
// 5 个代币，并且每当你购买物品时，都会有 1 个代币的手续费。
// 游戏的玩家会输入他们想购买多少物品，然后 `total_cost` 函数
// 会计算物品的总成本。由于玩家输入的是数量，我们得到的是一个字符串。
// 他们可能输入了任何东西，而不仅仅是数字！
//
// 目前，这个函数根本没有处理错误情况。我们想做的是：
// 如果我们用一个不是数字的字符串调用 `total_cost` 函数，该函数将
// 返回一个 `ParseIntError`。在这种情况下，我们希望立即从我们的函数中
// 返回该错误，而不是尝试进行乘法和加法运算。
//
// 至少有两种方法可以正确地实现这一点。但其中一种要短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: 如上所述处理错误情况。
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
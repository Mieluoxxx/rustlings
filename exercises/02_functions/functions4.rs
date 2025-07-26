// 这家商店正在促销，如果价格是偶数，您可以得到 10 个
// Rustbucks 的折扣，但如果是奇数，则是 3 个 Rustbucks 的折扣。
// 不要担心函数体本身，我们目前只对
// 函数签名感兴趣。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 修复函数签名。
fn sale_price(price: i64) -> i64{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

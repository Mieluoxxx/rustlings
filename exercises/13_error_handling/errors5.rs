// 这个练习是 `errors4` 练习的修改版。它使用了一些
// 我们要到课程后面才会接触到的概念，比如 `Box` 和 `From` trait。
// 现在不必详细了解它们，但如果你愿意，可以提前阅读。
// 目前，可以将 `Box<dyn ???>` 类型看作是“我想要任何实现了 ??? 的东西”的类型。
//
// 简而言之，这种 box 的特定用例是当你想要拥有一个值，
// 并且你只关心它是一个实现了特定 trait 的类型时。
// 为此，`Box` 被声明为 `Box<dyn Trait>` 类型，其中 `Trait` 是编译器
// 在该上下文中使用的任何值上寻找的 trait。对于本练习，
// 该上下文是可以在 `Result` 中返回的潜在错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 这是必需的，以便 `CreationError` 可以实现 `Error`。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: 添加正确的返回类型 `Result<(), Box<dyn ???>>`。我们可以用什么
// 来描述这两种错误？是否存在两种错误都实现的 trait？
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
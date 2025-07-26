// 不建议在库代码中使用像 `Box<dyn Error>` 这样的包罗万象的错误类型，
// 因为调用者可能希望根据错误内容做出决策，而不是打印出来或进一步传播。
// 在这里，我们定义一个自定义错误类型，以便调用者在我们的函数返回错误时
// 能够决定下一步该做什么。

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 我们将在 `PositiveNonzeroInteger::parse` 中使用的自定义错误类型。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: 在这里添加另一个错误转换函数。
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: 将此更改为在 `parse()` 返回错误时返回适当的错误，而不是 panic。
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
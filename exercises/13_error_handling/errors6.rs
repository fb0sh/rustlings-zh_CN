// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.
// 在库代码中不推荐使用像 `Box<dyn Error>` 这样的“包罗万象”的错误类型，因为
// 调用者可能希望根据错误内容做出决策，而不是将其打印出来或进一步传播。
// 在这里，我们定义了一个自定义错误类型，以便当我们的函数返回错误时，调用者
// 可以决定下一步该做什么。

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
// 一个我们将在 `PositiveNonzeroInteger::parse` 中使用的自定义错误类型。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    // TODO: 在这里添加另一个错误转换函数。
    // fn from_parse_int(???) -> Self { ??? }
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
        // TODO: change this to return an appropriate error instead of panicking
        // when `parse()` returns an error.
        // TODO: 将此更改为在 `parse()` 返回错误时返回一个适当的错误，而不是 panic。
        let x: i64 = s.parse().unwrap();
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
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

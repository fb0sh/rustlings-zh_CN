// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, the `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context. For this exercise, that context is the potential errors which
// can be returned in a `Result`.
// 这个练习是 errors4 练习的修改版本。它使用了一些我们将在课程后期才会
// 学到的概念，比如 Box 和 From trait。现在不重要去详细理解它们，但如果你
// 愿意可以提前阅读。目前，可以把 Box<dyn ???> 类型看作是“我想要任何做
// ??? 的类型”的类型。
//
// 简而言之，Box 的这种特定用法是当你想要拥有一个值，并且你只关心它是一
// 个实现了特定 trait 的类型时。为此，Box 被声明为 Box<dyn Trait> 类型，
// 其中 Trait 是编译器在该上下文中寻找的任何值上的 trait。对于这个练习，
// 该上下文是可以返回在 Result 中的潜在错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
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

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement?
// TODO: 添加正确的返回类型 `Result<(), Box<dyn ???>>`。
// 我们可以用什么来描述这两种错误？
// 是否存在一个这两种错误都实现的 trait？
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

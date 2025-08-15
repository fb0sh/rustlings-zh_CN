// This is similar to the previous `from_into` exercise. But this time, we'll
// implement `FromStr` and return errors instead of falling back to a default
// value. Additionally, upon implementing `FromStr`, you can use the `parse`
// method on strings to generate an object of the implementor type. You can read
// more about it in the documentation:
// https://doc.rust-lang.org/std/str/trait.FromStr.html
// 这与之前的 `from_into` 练习类似。但这一次，我们将实现 `FromStr` 并返回错误，
// 而不是回退到默认值。此外，在实现 `FromStr` 后，你可以使用字符串的 `parse` 方法
// 来生成实现该 trait 的类型的对象。你可以在文档中了解更多信息：
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `FromStr` implementation.
// 我们将在 `FromStr` 实现中使用这个错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    // 字段数量不正确
    BadLen,
    // Empty name field
    // 名字字段为空
    NoName,
    // Wrapped error from parse::<u8>()
    // parse::<u8>() 的封装错误
    ParseInt(ParseIntError),
}

// TODO: Complete this `FromStr` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    error `ParsePersonError::BadLen`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the error `ParsePersonError::NoName`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the error `ParsePersonError::ParseInt`.
// TODO: 完成这个 `FromStr` 实现，以便能够从形如 "Mark,20" 的字符串中解析出 `Person`。
// 注意，你需要将年龄部分解析为 `u8`，例如使用 `"4".parse::<u8>()`。
//
// 步骤：
// 1. 使用逗号分割给定的字符串。
// 2. 如果分割后的元素少于或多于 2 个，返回错误 `ParsePersonError::BadLen`。
// 3. 使用分割后的第一个元素作为名字。
// 4. 如果名字为空，返回错误 `ParsePersonError::NoName`。
// 5. 将分割后的第二个元素解析为 `u8` 作为年龄。
// 6. 如果解析年龄失败，返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}

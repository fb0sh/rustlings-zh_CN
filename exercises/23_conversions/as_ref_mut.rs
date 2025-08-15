// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).

// AsRef 和 AsMut 允许进行低开销的引用到引用的转换。分别可阅读更多信息：
// https://doc.rust-lang.org/std/convert/trait.AsRef.html
// https://doc.rust-lang.org/std/convert/trait.AsMut.html
// 获取给定参数的字节数（不是字符数）
// （`.len()` 返回字符串的字节数）

// TODO: Add the AsRef trait appropriately as a trait bound.
// TODO: 适当地在 trait bound 中添加 `AsRef` trait。
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
// 获取给定参数的字符数（不是字节数）。
// TODO: 在 trait bound 中适当地添加 `AsRef` trait。
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
// 使用 `as_mut()` 对数字进行平方。
// TODO: 添加适当的 trait bound。
fn num_sq<T>(arg: &mut T) {
    // TODO: Implement the function body.
    // TODO: 实现函数体。
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
// Rust 编译器需要知道如何检查提供的引用是否有效，以便在引用有在被使用前
// 超出作用域的风险时，它可以告知程序员。请记住，引用是借用，并且不拥有
// 它们自己的数据。如果它们的所有者超出了作用域怎么办？

// TODO: Fix the compiler error by updating the function signature.
// TODO: 通过更新函数签名来修复编译器错误。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}

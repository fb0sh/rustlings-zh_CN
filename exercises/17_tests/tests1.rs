// Tests are important to ensure that your code does what you think it should
// do.
// 测试很重要，以确保你的代码做了你认为它应该做的事情。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    // TODO: 导入 `is_even`。你可以使用通配符来导入外部模块中的所有内容。

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        // TODO: 用一些值测试函数 `is_even`。
        assert!();
        assert!();
    }
}

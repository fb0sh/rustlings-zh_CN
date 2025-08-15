// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
// 使用位移计算2的幂。
// `1 << n` 等价于 “2的n次方”。

fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        // TODO: 用一些值测试函数 `power_of_2`。
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}

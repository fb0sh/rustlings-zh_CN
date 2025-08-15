// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
// 这个函数返回冰箱里还剩下多少冰淇淋。
// 如果是在22:00之前（24小时制），还剩下5勺。
// 在22:00，有人把它们都吃完了，所以没有冰淇淋了（值为0）。
// 如果 `hour_of_day` 大于23，则返回 `None`。

fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    // TODO: 完成函数体。
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        // TODO: 修复这个测试。如何获取 Option 中包含的值？
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Don't change this line  // 不要修改这一行.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}

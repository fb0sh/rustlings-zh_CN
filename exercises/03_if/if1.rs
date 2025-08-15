fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    // TODO: 完成这个函数以返回较大的数字！
    // 如果两个数字相等，可以返回任意一个。
    // 不要使用：
    // - 其他函数调用
    // - 额外的变量
}

fn main() {
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
}

// Don't mind this for now :)
// 暂时不用理会这个 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

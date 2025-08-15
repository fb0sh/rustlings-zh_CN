fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // TODO: 使用元组索引访问 `numbers` 的第二个元素
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

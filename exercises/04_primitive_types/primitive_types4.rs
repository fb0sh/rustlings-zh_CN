fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // TODO: 从数组 `a` 中获取一个切片 `nice_slice`，确保测试可以通过
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}

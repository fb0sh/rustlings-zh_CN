fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        // TODO: 将 `input` 切片中的每个元素乘以 2 并推入 `output` 向量
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    // 示例：在映射后收集为向量
    // 将 `input` 切片中的每个元素映射为它的值加 1
    // 如果输入是 `[1, 2, 3]`，输出就是 `[2, 3, 4]`
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Here, we also want to multiply each element in the `input` slice
    // by 2, but with iterator mapping instead of manually pushing into an empty
    // vector.
    // See the example in the function `vec_map_example` above.
    // TODO: 使用迭代器映射，而不是手动推入空向量，将 `input` 切片中的每个元素乘以 2
    // 查看上面函数 `vec_map_example` 中的示例。
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}

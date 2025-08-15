fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    // TODO: 仅通过重新排列测试中的行修复编译错误
    // 不要添加、修改或删除任何行
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

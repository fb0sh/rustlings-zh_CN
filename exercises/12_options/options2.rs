fn main() {
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // TODO: 将此项改为其值为 `Some` 的 if-let 语句。
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // TODO: 将此项改为一个 while-let 语句。
        // 记住，`Vec::pop()` 会额外增加一层 `Option`。
        // 你可以在 if-let 和 while-let 语句中进行嵌套模式匹配。
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

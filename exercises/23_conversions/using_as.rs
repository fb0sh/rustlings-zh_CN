// Type casting in Rust is done via the usage of the `as` operator.
// Note that the `as` operator is not only used when type casting. It also helps
// with renaming imports.
// Rust 中的类型转换是通过使用 `as` 运算符完成的。
// 注意，`as` 运算符不仅用于类型转换，它也可用于重命名导入。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Make a conversion before dividing.
    // TODO: 在除法运算前进行类型转换。
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

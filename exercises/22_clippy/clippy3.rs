// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.
// 这里还有一些简单的 Clippy 修复，让你了解它的作用 📎
// TODO: 修复所有 Clippy 警告。

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    // 假设你不知道 `my_option` 的值。
    // 如果是 `Some`，我们希望打印它的值。
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // 我们来交换这两个值吧！
    value_a = value_b;
    value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}

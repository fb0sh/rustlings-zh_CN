// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.
// Clippy 工具是一个 lint 集合，用于分析你的代码，帮助你发现常见错误并改进 Rust 代码。
//
// 对于这些练习，当出现 Clippy 警告时，代码将无法编译。
// 根据输出中的 Clippy 建议来解决练习。

fn main() {
    // TODO: Fix the Clippy lint in this line.
    // TODO: 修复这一行的 Clippy 警告
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}

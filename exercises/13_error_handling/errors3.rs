// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// 这是一个程序，它正在尝试使用之前练习中已完成版本的 `total_cost` 函数。
// 然而它并没有工作！
// 为什么？我们应该怎么做来修复它？

use std::num::ParseIntError;

// Don't change this function.
// 不要修改这个函数
// 不要修改这个函数
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
// TODO: 通过更改 `main` 函数的签名和函数体来修复编译器错误。
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line  // 不要修改这一行.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}

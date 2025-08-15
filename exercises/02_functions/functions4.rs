// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.
// 这家商店正在做促销，如果价格是偶数，你可以减免 10 Rustbucks；
// 如果价格是奇数，你可以减免 3 Rustbucks。
// 这里我们只关注函数签名，不需要实现函数体。


fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
// TODO: 修复函数签名。
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

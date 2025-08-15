// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, we get it as a string. They might have
// typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!
// 假设我们正在编写一个可以用代币购买物品的游戏。所有物品都花费5个代币，
// 并且当你购买物品时，会产生1个代币的手续费。游戏玩家将输入他们想购买的
// 物品数量，而 total_cost 函数将计算物品的总成本。因为玩家输入的是
// 数量，我们将其作为字符串获取。他们可能输入了任何东西，而不仅仅是数字！
//
// 现在，这个函数根本没有处理错误情况。我们想做的是：如果我们在一个不是
// 数字的字符串上调用 total_cost 函数，该函数将返回一个 ParseIntError。
// 在这种情况下，我们希望立即从我们的函数中返回该错误，而不是尝试进行
// 乘法和加法运算。
//
// 至少有两种方法可以实现这一点，它们都是正确的。但其中一种要短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    // TODO: 处理上述错误情况。
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}

// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.

// 这是针对以下章节的测验：
// - 变量（Variables）
// - 函数（Functions）
// - 条件语句（If）
//
// Mary 正在购买苹果。苹果的价格计算如下：
// - 每个苹果 2 Rustbucks。
// - 但是，如果 Mary 购买超过 40 个苹果，则整个订单中每个苹果的价格降至 1 Rustbuck！
//
// TODO: 编写一个函数，根据购买数量计算苹果订单的价格。

// fn calculate_price_of_apples(???) -> ??? { ??? }
fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

// Don't change the tests!
// 不要更改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}

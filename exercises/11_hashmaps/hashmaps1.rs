// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.
// TODO: 定义一个水果篮子，使用哈希映射形式。
// 键表示水果名称，值表示该水果的数量。
// 至少放入 3 种不同的水果（如 apple、banana、mango），且总数至少为 5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // TODO: 声明一个哈希映射（hash map）

    // let mut basket =

    // Two bananas are already given for you :)
    // 已经给你两个香蕉了 :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.
    // TODO: 在你的篮子里放更多的水果。

    basket
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}

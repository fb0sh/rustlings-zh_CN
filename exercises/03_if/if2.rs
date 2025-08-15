// TODO: Fix the compiler error on this function.
// TODO: 修复这个函数的编译错误。
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" { "Yummy!" } else { 1 }
}

fn main() {
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
// TODO: 阅读测试以理解期望的行为。
// 在不修改测试的情况下，让所有测试通过。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        // 这意味着调用 `picky_eater("strawberry")` 应该返回 "Yummy!"。
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}

// TODO: Fix the compiler error in the `main` function without changing this function.
// TODO: 修复 main 函数中的编译错误，但不要修改 is_a_color_word 函数
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line  // 不要修改这一行.

    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

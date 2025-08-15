// Characters (`char`)
// 字符类型 (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    // 注意这里使用的是 _单引号_，它们与之前见到的双引号不同。

    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉
    // TODO: 类似前面的例子，在下面声明一个变量 `your_character`，
    // 并赋值为你喜欢的字符。
    // 可以是字母，可以是数字（用单引号），可以是特殊字符，
    // 可以是不同语言的字符，甚至可以是表情符号 😉

    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

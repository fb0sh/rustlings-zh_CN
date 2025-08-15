// Calls of this function should be replaced with calls of `string_slice` or `string`.
// 对这个函数的调用应该替换为对 `string_slice` 或 `string` 的调用。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
// TODO: 这里有一堆值——有些是 `String`，有些是 `&str`。
// 你的任务是根据每个值的类型，用 `string_slice(…)` 或 `string(…)` 替换 `placeholder(…)`。
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // 警告：这是按字节索引，而不是按字符索引。
    // 如果想按字符索引，可以使用 `s.chars().nth(INDEX)`。
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

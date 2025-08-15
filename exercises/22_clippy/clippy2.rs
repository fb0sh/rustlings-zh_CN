fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // TODO: 修复 Clippy 的 lint 警告。
    for x in option {
        res += x;
    }

    println!("{res}");
}

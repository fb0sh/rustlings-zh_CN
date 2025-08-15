// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.
// 你可以使用 `use` 关键字将模块路径（尤其是标准库中的模块）引入到作用域中。

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// TODO: 将 `SystemTime` 和 `UNIX_EPOCH` 从 `std::time` 模块引入到当前作用域。
// 一行搞定可以获得额外风格分！

// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

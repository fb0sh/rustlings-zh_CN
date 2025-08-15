// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
// trait `AppendBar` 只有一个函数，它将 "Bar" 附加到任何实现此 trait 的对象上。
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    // TODO: 为类型 `String` 实现 `AppendBar`。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}

// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
// 这个强大的包装器提供了存储正整数值的能力。
// TODO: 使用泛型重写它，以便它支持包装任何类型。
struct Wrapper {
    value: u32,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
// TODO: 调整此结构体的实现，使其对被包装的值是泛型的。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

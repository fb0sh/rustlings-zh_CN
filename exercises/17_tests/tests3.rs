struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    // 不要修改这个函数
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            // 在这里返回一个 `Result` 会更好。但我们想学习如何测试会 panic 的函数。
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
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
    fn correct_width_and_height() {
        // TODO: This test should check if the rectangle has the size that we
        // pass to its constructor.
        // TODO: 这个测试应该检查矩形是否具有我们传递给其构造函数的大小。
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Check width // 检查宽度
        assert_eq!(todo!(), 20); // Check height // 检查高度
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative width.
    // TODO: 这个测试应该检查当我们尝试创建一个宽度为负数的矩形时，程序是否会 panic。
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative height.
    // TODO: 这个测试应该检查当我们尝试创建一个高度为负数的矩形时，程序是否会 panic。
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

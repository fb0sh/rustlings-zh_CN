fn main() {
    my_macro!();
}

// TODO: Fix the compiler error by moving the whole definition of this macro.
// TODO: 通过移动宏的整个定义来修复编译错误
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

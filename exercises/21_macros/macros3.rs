// TODO: Fix the compiler error without taking the macro definition out of this
// module.
// TODO: 修复编译错误，但不要将宏定义移出此模块。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}

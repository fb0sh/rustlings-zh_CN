// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.
// 在编译时，Rust 需要知道一个类型占用多少空间。这对于递归类型来说是
// 有问题的，因为一个值可以包含自身类型的另一个值作为其一部分。为了解决
// 这个问题，我们可以使用 `Box`——一种用于在堆上存储数据的智能指针，
// 它也允许我们包装一个递归类型。
//
// 我们在此练习中实现的递归类型是“cons list”，一种在函数式编程语言中
// 常见的数据结构。cons list 中的每个项都包含两个元素：当前项的值和
// 下一个项。最后一项是一个名为 `Nil` 的值。

// TODO: Use a `Box` in the enum definition to make the code compile.
// TODO: 在枚举定义中使用 `Box` 来使代码能够编译。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: Create an empty cons list.
// TODO: 创建一个空的 cons list。
fn create_empty_list() -> List {
    todo!()
}

// TODO: Create a non-empty cons list.
// TODO: 创建一个非空的 cons list。
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

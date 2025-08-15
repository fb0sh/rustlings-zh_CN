// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.
// `Vec<T>` 对类型 `T` 是泛型的。在大多数情况下，编译器能够推断出 `T`，
// 例如在向向量中推入一个具有具体类型的值之后。但在本练习中，编译器需要
// 通过类型注解获得一些帮助。

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    // TODO: 通过注解向量 `Vec<T>` 的类型来修复编译器错误。
    // 选择 `T` 为某种可以由 `u8` 和 `i8` 创建的整数类型。
    let mut numbers = Vec::new();

    // Don't change the lines below.
    // 不要修改下面的行
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}

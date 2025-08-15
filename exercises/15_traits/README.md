# Traits

**Trait** 是一个方法集合。

数据类型可以实现 trait。为此，需要为数据类型定义构成 trait 的方法。例如，`String` 数据类型实现了 `From<&str>` trait。这允许用户编写 `String::from("hello")`。

因此，trait 在某种程度上类似于 Java 接口和 C++ 抽象类。

一些额外的常见 Rust trait 包括：

-   `Clone` (`clone` 方法)
-   `Display` (允许通过 `{}` 进行格式化显示)
-   `Debug` (允许通过 `{:?}` 进行格式化显示)

由于 trait 表示数据类型之间的共享行为，因此它们在编写泛型时非常有用。

---
## 进一步信息

-   [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

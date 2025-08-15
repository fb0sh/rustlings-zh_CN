# 生命周期

生命周期告诉编译器如何检查引用在任何给定情况下是否存活得足够长以保持有效。例如，生命周期会说“确保参数 'a' 的存活时间与参数 'b' 一样长，以便返回值有效”。

它们只在借用（即引用）时是必需的，因为复制的参数或移动的参数在其作用域内是拥有的，不能在作用域外被引用。生命周期意味着可以检查调用代码（例如函数）以确保其参数有效。生命周期对调用者是有限制的。

如果你想了解更多关于生命周期注解的信息，`lifetimekata` 项目拥有与 Rustlings 类似的练习风格，但它全部是关于学习编写生命周期注解的。

---
## 进一步信息

- [生命周期 (Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

# Lifetimes

Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

They are only necessary on borrows, i.e. references,
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are
restrictive of their callers.

If you'd like to learn more about lifetime annotations, the
[lifetimekata](https://tfpk.github.io/lifetimekata/) project
has a similar style of exercises to Rustlings, but is all about
learning to write lifetime annotations.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

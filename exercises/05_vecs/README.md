# 向量（Vectors）

向量是 Rust 中最常用的数据结构之一。在其他编程语言中，它们通常被称为数组，但由于 Rust 的底层操作方式不同，Rust 中的数组存储在栈上（意味着它的大小在编译时就必须确定，且不能动态增长或缩小），而向量存储在堆上（这些限制不适用，可以动态改变大小）。

向量在书中是稍后才讲的内容，但我们认为它们足够重要，所以提前介绍。至于另一种常用数据结构——哈希表（HashMap），我们会在后面讲到。

## 进一步信息

- [使用向量存储值列表](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)


# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

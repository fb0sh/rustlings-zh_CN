# 智能指针

在 Rust 中，智能指针是包含内存地址并引用其他数据，但同时拥有额外元数据和功能的变量。Rust 中的智能指针通常拥有它们指向的数据，而引用只是借用数据。

---
## 进一步信息

- [智能指针](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [使用 Box 指向堆上的数据](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>，引用计数智能指针](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [共享状态并发](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow 文档](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

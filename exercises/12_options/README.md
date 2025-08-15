# Option

`Option` 类型代表一个可选值：每个 `Option` 要么是 `Some` 并包含一个值，要么是 `None`，不包含任何值。

`Option` 类型在 Rust 代码中非常常见，因为它有多种用途：

-   初始值
-   用于未在其整个输入范围内定义的函数（部分函数）的返回值
-   作为报告简单错误的返回值，错误时返回 `None`
-   可选的结构体字段
-   可以被借用或“取出”的结构体字段
-   可选的函数参数
-   可空指针
-   从困难情况中交换出东西

---
## 进一步信息

-   [Option 枚举格式](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
-   [Option 模块文档](https://doc.rust-lang.org/std/option/)
-   [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
-   [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
-   [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

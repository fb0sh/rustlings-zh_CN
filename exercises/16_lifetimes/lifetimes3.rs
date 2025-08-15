// Lifetimes are also needed when structs hold references.
// 当结构体持有引用时，也需要生命周期。

// TODO: Fix the compiler errors about the struct.
// TODO: 修复关于此结构体的编译器错误。
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}

// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// 在此练习中，我们得到一个名为 numbers 的 u32 向量（Vec），其值范围
// 为 0 到 99。我们希望在 8 个不同的线程中同时使用这组数字。每个线程将对
// 每隔八个值进行求和，并使用一个偏移量。
//
// 第一个线程（偏移量 0）将求和 0, 8, 16, …
// 第二个线程（偏移量 1）将求和 1, 9, 17, …
// 第三个线程（偏移量 2）将求和 2, 10, 18, …
// …
// 第八个线程（偏移量 7）将求和 7, 15, 23, …
//
// 每个线程都应该拥有一个指向该数字向量的引用计数指针。但是 Rc 不是
// 线程安全的。因此，我们需要使用 Arc。
//
// 不要被线程的生成和加入方式分散注意力。我们将在稍后的线程练习中进行实践。

// Don't change the lines below.
// 不要修改下面的行
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    // TODO: 使用 `Arc` 定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // TODO: 使用 `shared_numbers` 定义 `child_numbers`。
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

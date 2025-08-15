// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
// 这个程序生成多个线程，每个线程都至少运行 250ms，并且每个线程都返回
// 其完成所花费的时间。该程序应该等待所有生成的线程完成，并将其返回值
// 收集到一个向量中。

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        // TODO: 将所有线程的结果收集到 `results` 向量中。
        // 使用 `thread::spawn` 返回的 `JoinHandle` 结构体。
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}

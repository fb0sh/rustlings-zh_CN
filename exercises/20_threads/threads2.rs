// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`
// 在此练习的基础上，我们希望所有线程都完成它们的工作。但这次，生成的
// 线程需要负责更新一个共享值：`JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    // TODO: 如果你想要一个**可变**的共享状态，`Arc` 是不够的。
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // TODO: 在更新共享值之前，你必须先采取一个行动。
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    // 等待所有任务完成。
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    // TODO: 打印 JobStatus.jobs_done 的值。
    println!("Jobs done: {}", todo!());
}

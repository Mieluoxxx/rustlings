// 在上一个练习的基础上，我们希望所有的线程都能完成它们的工作。
// 但这一次，生成的线程需要负责更新一个共享值：`JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: 如果你想要一个**可变的**共享状态，`Arc` 是不够的。
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: 在更新共享值之前，你必须采取一个行动。
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有任务完成。
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 打印 `JobStatus.jobs_done` 的值。
    println!("已完成的任务：{}", todo!());
}
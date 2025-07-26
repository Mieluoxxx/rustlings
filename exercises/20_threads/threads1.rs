// 这个程序会生成多个线程，每个线程至少运行 250 毫秒，
// 并且每个线程返回它完成所需的时间。程序应该
// 等待所有生成的线程都完成后，并将其返回值收集到一个向量中。

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
            println!("线程 {i} 完成");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: 将所有线程的结果收集到 `results` 向量中。
        // 使用 `thread::spawn` 返回的 `JoinHandle` 结构体。
    }

    if results.len() != 10 {
        panic!("哦不！有些线程还没有完成！");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("线程 {i} 花费了 {result} 毫秒");
    }
}
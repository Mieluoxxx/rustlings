// 在这个练习中，我们给定一个名为 `numbers` 的 `Vec<u32>`，其值范围从 0 到 99。
// 我们希望在 8 个不同的线程中同时使用这组数字。每个线程将获取
// 带有偏移量的每八个值的总和。
//
// 第一个线程（偏移量 0），将对 0, 8, 16, … 求和
// 第二个线程（偏移量 1），将对 1, 9, 17, … 求和
// 第三个线程（偏移量 2），将对 2, 10, 18, … 求和
// …
// 第八个线程（偏移量 7），将对 7, 15, 23, … 求和
//
// 每个线程都应该拥有一个指向数字向量的引用计数指针。
// 但是 `Rc` 不是线程安全的。因此，我们需要使用 `Arc`。
//
// 不要被线程如何生成和加入所分心。我们将在稍后关于线程的练习中练习这一点。

// 不要改变下面的代码行。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: 使用 `Arc` 定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
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
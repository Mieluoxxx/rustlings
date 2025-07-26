// TODO: 在冒号 `:` 后添加缺失的参数 `num` 的类型。
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

// Clippy 工具是一组用于分析代码的 lint 的集合，因此您可以
// 发现常见错误并改进您的 Rust 代码。
//
// 对于这些练习，当存在 Clippy
// 警告时，代码将无法编译。检查 Clippy 从输出中提出的建议来解决这个练习。

fn main() {
    // TODO: 修复此行中的 Clippy lint。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("半径为 {radius:.2} 的圆的面积是 {area:.5}");
}

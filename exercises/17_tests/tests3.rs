struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 不要改变这个函数。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 在这里返回 `Result` 会更好。但我们想学习
            // 如何测试可能 panic 的函数。
            panic!("矩形的宽度和高度必须是正数");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: 这个测试应该检查矩形是否具有我们
        // 传递给其构造函数的大小。
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // 检查宽度
        assert_eq!(todo!(), 20); // 检查高度
    }

    // TODO: 这个测试应该检查当我们尝试创建一个
    // 具有负宽度的矩形时程序是否会 panic。
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: 这个测试应该检查当我们尝试创建一个
    // 具有负高度的矩形时程序是否会 panic。
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
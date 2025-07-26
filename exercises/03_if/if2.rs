// TODO: 修复这个函数上的编译器错误。
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

// TODO: 阅读测试以了解期望的行为。
// 使所有测试通过，不要修改它们。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // 这意味着使用参数 "strawberry" 调用 `picky_eater` 应该返回 "Yummy!"。
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}

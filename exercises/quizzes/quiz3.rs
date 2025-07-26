// 本测验测试：
// - 泛型
// - Trait
//
// 一所虚构的魔法学校有一个用 Rust 编写的新成绩单生成系统！
// 目前，该系统仅支持创建学生成绩以数字表示的成绩单（例如 1.0 -> 5.5）。
// 然而，学校也发布字母等级（A+ -> F-），并且需要能够打印两种类型的成绩单！
//
// 在 `ReportCard` 结构体和 impl 块中进行必要的代码更改，
// 以支持字母成绩单和数字成绩单。

// TODO：如上所述调整结构体。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO：如上所述调整 impl 块。
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}

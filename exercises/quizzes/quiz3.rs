// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// 本测验测试内容：
// - 泛型（Generics）
// - Trait
//
// 一个虚构的魔法学校有一个用 Rust 编写的新成绩单生成系统！目前，
// 系统仅支持生成学生成绩以数字表示的成绩单（例如 1.0 -> 5.5）。
// 但是，学校也会发布字母成绩（A+ -> F-），并且需要能够打印这两种类型的成绩单！
//
// 请在 `ReportCard` 结构体和 impl 块中进行必要的修改，以支持字母成绩单，
// 同时保留数字成绩单的支持。

// TODO: Adjust the struct as described above.
// TODO: 按上述说明调整结构体。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
// TODO: 根据上述描述调整 impl 块。
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
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

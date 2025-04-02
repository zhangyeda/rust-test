// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。

#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    age: u8,
    score: f32,
}

impl Student {
    fn new(name: &str, age: u8, score: f32) -> Self {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }

    fn show(&self) {
        println!(
            "Name: {}, Age: {}, Score: {:.1}",
            self.name, self.age, self.score
        );
    }

    fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let student = Student::new("Bob", 20, 75.5);
        assert_eq!(student.name, "Bob");
        assert_eq!(student.age, 20);
        assert_eq!(student.score, 75.5);
    }

    #[test]
    fn test_is_passed() {
        assert!(Student::new("", 0, 60.0).is_passed());
        assert!(Student::new("", 0, 75.5).is_passed());
        assert!(!Student::new("", 0, 59.9).is_passed());
    }
}

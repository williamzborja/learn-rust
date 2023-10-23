struct Student {
    name: String,
    level: u8,
    remote: bool,
}

struct Grades(char, char, char, char, f32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct() {
        let student_1 = Student {
            name: String::from("Doe"),
            level: 2,
            remote: false,
        };

        let student_2 = Student {
            name: String::from("John"),
            level: 5,
            remote: true,
        };

        let mark_1 = Grades('A', 'B', 'A', 'A', 3.75);
        let mark_2 = Grades('B', 'A', 'A', 'B', 3.25);

        println!("{} {} {} {}", mark_1.0, mark_1.1, mark_1.2, mark_1.3);
        println!("{} {} {} {}", mark_2.0, mark_2.1, mark_2.2, mark_2.3);

        println!(
            "{} {} {}",
            student_1.name, student_1.level, student_1.remote
        );
        println!(
            "{} {} {}",
            student_2.name, student_2.level, student_2.remote
        );
    }
}

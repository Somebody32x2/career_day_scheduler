use crate::misc::{Class, ClassOutput};
use crate::student::Student;

pub fn read_students(path: String) -> Vec<Student> {
    let mut students = Vec::new();
    // Make a new csv reader with flexible enabled and set it to read students.csv
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .unwrap();
    for result in rdr.records() {
        let record: Vec<String> = result.unwrap().iter().map(|x| x.trim_start().to_string()).collect();

        if record.len() >= 8 && record[0].parse::<u64>().is_ok() { // ignore config lines and header
            let student = Student {
                timestamp: record[0].parse().unwrap(),
                first_name: record[1].to_string(),
                last_name: record[2].to_string(),
                homeroom: record[3].to_string(),
                first_period: record[4].to_string(),
                student_id: record[5].parse().unwrap(),
                grade: record[6].parse().unwrap(),
                preferences: record.iter().skip(6).map(|x| x.parse().unwrap()).collect(),
                classes: Vec::new(),
            };
            students.push(student);
        }
    }
    students
}

pub fn read_classes(path: String) -> ClassOutput {
    let mut class_output = ClassOutput {
        num_sessions: u16::MAX,
        min_students: -1,
        max_students: -1,
        classes: Vec::new(),
    };
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(path)
        .unwrap();
    for result in rdr.records() {
        let record: Vec<String> = result.unwrap().iter().map(|x| x.trim_start().to_string()).collect();
        if record[0].to_lowercase().contains("num") && record[0].to_lowercase().contains("sessions") && record[1].parse::<u32>().is_ok() {
            class_output.num_sessions = record[1].parse().unwrap();
        } else if record[0].to_lowercase().contains("min") && record[0].to_lowercase().contains("students") && record[1].parse::<u32>().is_ok() {
            class_output.min_students = record[1].parse().unwrap();
        } else if record[0].to_lowercase().contains("max") && record[0].to_lowercase().contains("students") && record[1].parse::<u32>().is_ok() {
            class_output.max_students = record[1].parse().unwrap();
        } else if record.len() >= 4 && record[0].parse::<i32>().is_ok() { // ignore config lines and header
            let class = Class {
                id: record[0].parse().unwrap(),
                subject: record[1].to_string(),
                teacher: record[2].to_string(),
                presenter: record[3].to_string(),
            };
            class_output.classes.push(class);
        }
    }

    if class_output.num_sessions == u16::MAX {
        class_output.num_sessions = class_output.classes.len() as u16;
    }
    // assert!(class_output.num_sessions > 0, "Number of sessions invalid or not found");
    assert!(class_output.min_students >= 0, "Minimum number of students invalid or not found");
    assert!(class_output.max_students == -1 || class_output.max_students > 0, "Maximum number of students must be -1 (unlimited) or a positive integer");

    class_output

}
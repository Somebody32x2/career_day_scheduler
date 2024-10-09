mod misc;
mod read_data;
mod student;
const STUDENT_FILE: &str = "students.csv";
const CLASSES_FILE: &str = "sessions.csv";

fn main() {
    let mut students = read_data::read_students(STUDENT_FILE.to_string());
    println!("Read {} students", students.len());
    let min_timestamp = students.iter().map(|x| x.timestamp).min().unwrap();
    students.sort_by_key(|a| a.sort_order(min_timestamp));

    let class_output = read_data::read_classes(CLASSES_FILE.to_string());
    println!("Read {} classes", class_output.classes.len());
}

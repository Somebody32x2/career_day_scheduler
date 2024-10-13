use crate::student::Student;

pub fn test_satisfaction(students: &Vec<Student>) {
    let mut grade_avg_satisfaction = vec![0f32; 6];
    let mut grade_count = vec![0; 6];
    for student in students {
        grade_avg_satisfaction[student.grade as usize - 7] += student.satisfaction();
        grade_count[student.grade as usize - 7] += 1;
    }
    for i in 0..6 {
        if grade_count[i] > 0 {
            grade_avg_satisfaction[i] /= grade_count[i] as f32;
            println!("Grade {} average satisfaction: {}", i + 7, grade_avg_satisfaction[i]);
        }
    }
}
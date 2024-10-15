use crate::student::Student;

pub fn test_satisfaction(students: &Vec<Student>) {
    let mut grade_total_satisfaction = vec![0f32; 6];
    let mut grade_count = vec![0; 6];
    for student in students {
        grade_total_satisfaction[student.grade as usize - 7] += student.satisfaction();
        grade_count[student.grade as usize - 7] += 1;
    }
    let mut perfect_satisfaction = 0f32;
    for i in 0..students[0].classes.len() {
        perfect_satisfaction += students[0].preferences.len() as f32 - i as f32;
    }
    println!("   Total average satisfaction: {:.2}/{} ({:.3}%)", grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32, perfect_satisfaction, (grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32 / perfect_satisfaction)*100.0);

    for i in 0..6 {
        if grade_count[i] > 0 {
            println!("Grade {: >2} average satisfaction: {}/{} ({:.3}%)", i + 7, grade_total_satisfaction[i] / grade_count[i] as f32, perfect_satisfaction, (grade_total_satisfaction[i] / grade_count[i] as f32 / perfect_satisfaction)*100.0);
        }
    }
}
use std::collections::HashMap;

use crate::student::Student;

// Represents a session to be taught by one presenter in a teacher's room of a subject
#[derive(Debug)]
pub struct Class {
    pub(crate) id: u16,
    pub(crate) subject: String,
    pub(crate) teacher: String,
    pub(crate) presenter: String,
}

pub struct ClassOutput {
    pub(crate) num_sessions: u16,
    pub(crate) min_students: i32,
    pub(crate) max_students: i32,
    pub(crate) classes: Vec<Class>,
}
struct PeriodClass {
    class_id: u16,
    period: u16,
}
pub fn schedule_valid(schedule: &HashMap<u16, Vec<Vec<u32>>>, i_students: &Vec<Student>, min_students: i32, max_students: i32) -> bool {
    let mut students: HashMap<u32, Vec<PeriodClass>> = HashMap::new();
    for (class_id, periods) in schedule {
        for (i, period) in periods.iter().enumerate() {
            // Check if the number of students in the period is within the bounds
            if period.len() < min_students as usize || period.len() > max_students as usize {
                println!("Class {} has {} students in period {}, which is outside the bounds of min: {} to max: {}", class_id, period.len(), i + 1, min_students, max_students);
                return false;
            }
            // Check if any student is assigned to multiple classes in the same period
            for student_id in period {
                if students.contains_key(student_id) {
                    for period_class in students.get(student_id).unwrap() {
                        if period_class.period == i as u16 {
                            println!("Student {} is assigned to multiple classes in period {}", student_id, i + 1);
                            return false;
                        }
                    }
                }
                students.insert(*student_id, vec![PeriodClass { class_id: *class_id, period: i as u16 }]);
            }
        }
    }
    // Check that every student is assigned to a valid class in every period
    for student in i_students {
        for (i, class_id) in student.classes.iter().enumerate() {
            if !schedule.contains_key(class_id) {
                println!("Student {} is assigned to class {} in period {}, which does not exist", student.student_id, class_id, i + 1);
                return false;
            }
        }
    }
    true
}


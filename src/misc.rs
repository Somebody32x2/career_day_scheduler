use std::collections::HashMap;
use crate::NUM_PERIODS;
use crate::read_write_data::NUM_PREFERENCES;
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
pub fn schedule_valid(schedule: &HashMap<u16, Vec<Vec<u32>>>, i_students: &Vec<Student>, min_students: i32, max_students: i32, middle_school_prohibited_classes: &[&u16]) -> bool {
    let mut is_valid = true;
    let mut students: HashMap<u32, Vec<PeriodClass>> = HashMap::new();
    for (class_id, periods) in schedule {
        for (i, period) in periods.iter().enumerate() {
            // Check if the number of students in the period is within the bounds
            if period.len() < min_students as usize || period.len() > max_students as usize {
                println!("Class {} has {} students in period {}, which is outside the bounds of min: {} to max: {}", class_id, period.len(), i + 1, min_students, max_students);
                is_valid = false;
            }
            // Check if any student is assigned to multiple classes in the same period
            for student_id in period {
                if students.contains_key(student_id) {
                    for period_class in students.get(student_id).unwrap() {
                        if period_class.period == i as u16 {
                            println!("Student {} is assigned to multiple classes in period {}", student_id, i + 1);
                            is_valid = false;
                        }
                    }
                }
                students.insert(*student_id, vec![PeriodClass { class_id: *class_id, period: i as u16 }]);
            }
            // Check student parity - every student has this as their nth period class
            for student_id in period {
                if !i_students.iter().find(|x| x.student_id == *student_id).unwrap().classes[i] == *class_id {
                    println!("Student {} is not assigned to class {} in period {}", student_id, class_id, i + 1);
                    is_valid = false;
                }
            }
        }
    }
    // Check that every student is assigned to a valid class in every period
    for student in i_students {
        if student.classes.len() != NUM_PERIODS as usize { 
            println!("Student {} is assigned to {} classes, but there are {} periods", student.student_id, student.classes.len(), NUM_PERIODS);
            is_valid = false;
        }
        if student.preferences.len() != NUM_PREFERENCES {
            println!("Student {} has {} preferences, but there are {} preferences", student.student_id, student.preferences.len(), NUM_PREFERENCES);
            is_valid = false;
        }
        for (i, class_id) in student.classes.iter().enumerate() {
            if !schedule.contains_key(class_id) {
                println!("Student {} is assigned to class {} in period {}, which does not exist", student.student_id, class_id, i + 1);
                is_valid = false;
            }
            if (student.grade <= 8) && middle_school_prohibited_classes.contains(&class_id) {
                println!("Middle school student {} is assigned to prohibited class {}", student.student_id, class_id);
                is_valid = false;
            }
            // Check for repeats
            for (j, other_class_id) in student.classes.iter().enumerate() {
                if i != j && class_id == other_class_id {
                    println!("Student {} is assigned to class {} multiple times", student.student_id, class_id);
                    is_valid = false;
                }
            }
            // Ensure class parity (each of this persons classes has them as a student)
            if !schedule[class_id][i].contains(&student.student_id) {
                println!("Student {} is not assigned to class {} in schedule in period {}", student.student_id, class_id, i + 1);
                is_valid = false;
            }
        }

    }
    // Check that no student repeats a class
    for (student_id, period_classes) in students {
        let mut classes: Vec<u16> = Vec::new();
        for period_class in period_classes {
            if classes.contains(&period_class.class_id) {
                println!("Student {} is assigned to class {} multiple times", student_id, period_class.class_id);
                is_valid = false;
            }
            classes.push(period_class.class_id);
        }
    }
    is_valid
}


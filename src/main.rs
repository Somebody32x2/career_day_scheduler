use std::collections::HashMap;
use std::ops::Index;

mod misc;
mod read_data;
mod student;

const STUDENT_FILE: &str = "students.csv";
const CLASSES_FILE: &str = "sessions.csv";
const NUM_PERIODS: u16 = 4; // number of slots per presenter done

fn main() {
    let mut students = read_data::read_students(STUDENT_FILE.to_string());
    println!("Read {} students", students.len());
    let min_timestamp = students.iter().map(|x| x.timestamp).min().unwrap();
    students.sort_by_key(|a| a.sort_order(min_timestamp));

    let class_output = read_data::read_classes(CLASSES_FILE.to_string());
    println!("Read {} classes", class_output.classes.len());

    let num_sessions = class_output.num_sessions;
    let min_students_per_session = class_output.min_students;
    let max_students_per_session = class_output.max_students;

    let mut schedule: HashMap<u16, Vec<Vec<u32>>> = HashMap::new();
    // class_id: [periods: [students]]


    // initialize schedule
    for class in &class_output.classes {
        schedule.insert(class.id, vec![vec![]; NUM_PERIODS as usize]);
    }

    // initialize student schedules
    for student in &mut students {
        student.classes = vec![0xffff; num_sessions as usize];
    }


    // 1st, Assign all students to their choices, ensuring valid state
    // let mut ndx = 1;
    // let students_len = students.len();
    for class_assignment_iteration in 0..NUM_PERIODS {
        'student: for student in students.iter_mut() {
            // ndx += 1;
            // Assign the student to the first available period of their first choice (which hasn't been already used) with available space
            'choice: for choice in &student.preferences {
                if student.classes.contains(choice) { continue 'choice; } // Ensure the student isn't already assigned to this class
                // Check if each class has space
                for period_num in 0..schedule[choice].len() {
                    // If the class has space, and the student is free, assign them to the class
                    if schedule[choice][period_num].len() < max_students_per_session as usize && student.classes[period_num] == 0xffff {
                        schedule.get_mut(choice).unwrap()[period_num].push(student.student_id);
                        student.classes[period_num] = *choice;
                        continue 'student; // We found a period for this student, may proceed with giving the next student a period
                    }
                }
            }
        }
    }

    // let class_ids = class_output.classes.iter().map(|x| x.id).collect::<Vec<u16>>();

    // If we reach this point, the student has not been assigned to any of their choices,
    // so put them in a class below its min_students, if any, then a random non-full one as a last resort
    'student: for mut student in students.iter_mut() {
        for period_num in 0..NUM_PERIODS {
            let period_num = period_num as usize;
            // If this period needs to be filled for the student, assign them to the class with the fewest students that period
            while student.classes[period_num] == 0xffff {
                // let random_class_id = &(class_ids[random::<u32>() as usize % class_ids.len()]);
                let class_id = *schedule.iter().min_by_key(|x| x.1[period_num].len()).unwrap().0;
                // if the class has space, and the student hasn't had this class yet, assign them to the class
                if schedule[&class_id][period_num].len() < min_students_per_session as usize && !student.classes.contains(&class_id)  {
                    schedule.get_mut(&class_id).unwrap()[period_num].push(student.student_id);
                    student.classes[period_num] = class_id;
                    continue 'student;
                }
            }
        }
    }
    println!("{:?}", schedule);
    // Print the number of students in each class
    for (class_id, periods) in &schedule {
        for (i, period) in periods.iter().enumerate() {
            println!("Class {} has {} students in period {}", class_id, period.len(), i);
        }
    }
    println!("Schedule is valid: {}", misc::schedule_valid(&schedule, min_students_per_session, max_students_per_session));
}

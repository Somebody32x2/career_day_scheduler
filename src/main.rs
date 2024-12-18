use crate::read_write_data::{check_valid_input, write_student_output, MISSING_PREFERENCE};
use crate::test::test_satisfaction;
use std::collections::HashMap;

mod misc;
mod read_write_data;
mod student;
mod test;

const STUDENT_FILE: &str = "students.csv";
const CLASSES_FILE: &str = "sessions.csv";
const NUM_PERIODS: u16 = 4; // number of slots per presenter done

const MIDDLE_SCHOOL_PROHIBITED_CLASSES: &'static [&'static u16] = &[&45, &46, &47]; // classes that middle school students cannot be in

fn main() {
    let mut students = read_write_data::read_students(STUDENT_FILE.to_string());
    println!("Read {} students", students.len());
    let min_timestamp = students.iter().map(|x| x.timestamp).min().unwrap();
    students.sort_by_key(|a| a.sort_order(min_timestamp));
    students.reverse();
    let class_output = read_write_data::read_classes(CLASSES_FILE.to_string());
    println!("Read {} classes", class_output.classes.len());

    let min_students_per_session = class_output.min_students;
    let max_students_per_session = class_output.max_students;

    let mut schedule: HashMap<u16, Vec<Vec<u32>>> = HashMap::new();
    // class_id: [periods: [students]]

    check_valid_input(&class_output.classes, &students);

    // initialize schedule
    for class in &class_output.classes {
        schedule.insert(class.id, vec![vec![]; NUM_PERIODS as usize]);
    }


    let aggressive_seniority_priority: bool = true; // TODO: make a prompt/cli option for prod


    // 1st, Assign all students to their choices, ensuring valid state

    'period: for _period in 0..NUM_PERIODS {
        'student: for student in students.iter_mut() {
            if aggressive_seniority_priority && student.grade < 11 {
                continue 'period;
            }

            // Check if each period has space for the best class for the student
            // Assign the student to the first available period of their first choice (which hasn't been already used) with available space
            'choice: for choice in &student.preferences {
                if *choice == MISSING_PREFERENCE { continue 'choice; } // Skip if the student has no preference
                if student.classes.contains(choice) { continue 'choice; } // Ensure the student isn't already assigned to this class
                // Check if each class has space
                for period_num in 0..schedule[choice].len() {
                    // If the class has space, and the student is free, assign them to the class
                    if schedule[choice][period_num].len() < max_students_per_session as usize && student.classes[period_num] == 0xffff && !(student.grade <= 8 && MIDDLE_SCHOOL_PROHIBITED_CLASSES.contains(&choice)) {
                        schedule.get_mut(choice).unwrap()[period_num].push(student.student_id);
                        student.classes[period_num] = *choice;
                        if !aggressive_seniority_priority {
                            continue 'student; // We found a period for this student, may proceed with giving the next student a period
                        } else {
                            continue 'choice;
                        }
                    }
                }
            }
        }
    }
    // analyze_capacity(&students, schedule.clone());
    // Go back and do the rest of the students
    if aggressive_seniority_priority {
        'period: for _period in 0..NUM_PERIODS {
            'student: for student in students.iter_mut() {
                if student.grade >= 11 {
                    continue 'student;
                }

                // Check if each period has space for the best class for the student
                // Assign the student to the first available period of their first choice (which hasn't been already used) with available space
                'choice: for choice in &student.preferences {
                    if *choice == MISSING_PREFERENCE { continue 'choice; } // Skip if the student has no preference
                    if student.classes.contains(choice) { continue 'choice; } // Ensure the student isn't already assigned to this class
                    // Check if each class has space
                    for period_num in 0..schedule[choice].len() {
                        // If the class has space, and the student is free, assign them to the class
                        if schedule[choice][period_num].len() < max_students_per_session as usize && student.classes[period_num] == 0xffff && !(student.grade <= 8 && MIDDLE_SCHOOL_PROHIBITED_CLASSES.contains(&choice)) {
                            schedule.get_mut(choice).unwrap()[period_num].push(student.student_id);
                            student.classes[period_num] = *choice;
                            continue 'student; // We found a period for this student, may proceed with giving the next student a period
                        }
                    }
                }
            }
        }
    }


    // let class_ids = class_output.classes.iter().map(|x| x.id).collect::<Vec<u16>>();

    println!("Assigned all students to their choices");

    // If we reach this point, the student has not been assigned to any of their choices,
    // put them in a class that needs students, so put them in the min students valid class
    for student in students.iter_mut() {
        for period_num in 0..NUM_PERIODS {
            let period_num = period_num as usize;
            // If this period needs to be filled for the student, assign them to the class with the fewest students that period
            while student.classes[period_num] == 0xffff {
                // let class_id = *schedule.iter().filter(|x| !student.classes.contains(x.0) && !(student.grade <= 8 && MIDDLE_SCHOOL_PROHIBITED_CLASSES.contains(&x.0))).min_by_key(|x| x.1[period_num].len()).unwrap().0;
                // Below is used to ensure consistent results
                let mut class_id = 0xffff;
                // Sort the class_ids by id
                let mut class_ids = schedule.keys().copied().collect::<Vec<u16>>();
                class_ids.sort();
                // Sort by the number of students in the class
                class_ids.sort_by_key(|x| schedule[x][period_num].len());
                for id in class_ids.iter() {
                    if !student.classes.contains(id) && !(student.grade <= 8 && MIDDLE_SCHOOL_PROHIBITED_CLASSES.contains(&id)) {
                        class_id = *id;
                        break;
                    }
                }
                // if the class has space, and the student hasn't had this class yet, assign them to the class (redundant check but just in case ig)
                if schedule[&class_id][period_num].len() + 1 < max_students_per_session as usize && !student.classes.contains(&class_id) {
                    schedule.get_mut(&class_id).unwrap()[period_num].push(student.student_id);
                    student.classes[period_num] = class_id;

                    // continue 'student;
                }
            }
        }
    }
    

    // println!("Schedule is valid: {}", misc::schedule_valid(&schedule, &students, min_students_per_session, max_students_per_session, MIDDLE_SCHOOL_PROHIBITED_CLASSES));

    // Check for classes with too few students, and assign students, taking from the most filled classes first, and the lowest preference students to be in that class, then just the fullest classes
    // least happy in their current class + most happy in the new class + taking from the fullest classes first
    let mut class_ids = schedule.keys().filter(|x1| { schedule[*x1].len() < min_students_per_session as usize }).copied().collect::<Vec<u16>>();
    class_ids.sort();
    class_ids.reverse();

    // class_ids.sort_by_key(|_| rand::random::<i32>());
    // Check if each class is less than the minimum number of students
    for class_id in class_ids.iter() {
        for period_num in 0..(NUM_PERIODS as usize) {
            if schedule[class_id][period_num].len() < min_students_per_session as usize {
                // Run student.move_score for each student and sort by move_score, then assign the required number of students to the class
                students.sort_by_key(|student| student.student_id);
                students.sort_by_key(|student| student.move_score(*class_id, period_num, schedule[&student.classes[period_num]][period_num].len() as i32, min_students_per_session, max_students_per_session, MIDDLE_SCHOOL_PROHIBITED_CLASSES, aggressive_seniority_priority));
                students.reverse();
                let mut student_taking_ndx = 0;
                while schedule[class_id][period_num].len() < min_students_per_session as usize {
                    let student = &mut students[student_taking_ndx];
                    if student.classes[period_num] != *class_id && schedule[&student.classes[period_num]][period_num].len() > min_students_per_session as usize && !student.classes.contains(class_id) {
                        schedule.get_mut(&student.classes[period_num]).unwrap()[period_num].retain(|x| x != &student.student_id);
                        // println!("Moved student {} from class {} to class {} p.{} ({})", student.student_id, student.classes[period_num], class_id, period_num, schedule[class_id][period_num].len());
                        student.classes[period_num] = *class_id;
                        schedule.get_mut(&class_id).unwrap()[period_num].push(student.student_id);
                    }
                    student_taking_ndx += 1;
                }
            }
        }
    }
    

    // Check if the schedule is valid
    println!("\x1B[1;4;36mSchedule is valid: {}\x1B[0m", misc::schedule_valid(&schedule, &students, min_students_per_session, max_students_per_session, MIDDLE_SCHOOL_PROHIBITED_CLASSES));
    assert!(misc::schedule_valid(&schedule, &students, min_students_per_session, max_students_per_session, MIDDLE_SCHOOL_PROHIBITED_CLASSES));

    write_student_output(&class_output.classes, &mut students, NUM_PERIODS, "output.csv".to_string());

    // write_student_satisfaction_details(&students, NUM_PERIODS, "satisfaction2.csv".to_string());
    // analyze_capacity(&students, schedule.clone());
    test_satisfaction(&students, true);

}

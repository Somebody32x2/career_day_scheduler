use crate::misc::{Class, ClassOutput};
use crate::NUM_PERIODS;
use crate::student::Student;
pub const NUM_PREFERENCES: usize = 6;
pub const MISSING_PREFERENCE: u16 = 0xfffe;
pub const UNASSIGNED_CLASSS: u16 = 0xffff;
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
                grade: record[6].parse().unwrap(), // REMOVE 7th CHOICE ------ REVIEW THIS
                preferences: record.iter().skip(7).rev().skip(if record.len() > 7 + NUM_PREFERENCES {1} else {0}).rev().map(|x| x.parse().unwrap()).collect(),
                classes: Vec::from([UNASSIGNED_CLASSS; NUM_PERIODS as usize]),
            };
            students.push(student);
        }
    }
    // Go over every student and set any unset preferences to 0xfffe
    for student in &mut students {
        while student.preferences.len() < NUM_PREFERENCES {
            student.preferences.push(MISSING_PREFERENCE);
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
    println!("min {} max {}", class_output.min_students, class_output.max_students);
    class_output
}
pub fn check_valid_input(classes: &Vec<Class>, students: &Vec<Student>) {
    // Check that all students have valid preferences
    for student in students {
        assert_eq!(student.preferences.len(), NUM_PREFERENCES, "Student {} {} has an invalid number of preferences: {}", student.first_name, student.last_name, student.preferences.len());
        for preference in &student.preferences {
            assert!(classes.iter().any(|x| x.id == *preference ) || *preference == MISSING_PREFERENCE, "Student {} {} has an invalid preference: {}, it was not listed as any class ID.", student.first_name, student.last_name, preference);
        }
    }
    // Check that all classes have valid, unique IDs
    for class in classes {
        assert!(class.id > 0, "Class {} has an invalid ID: {}", class.id, class.id);
        assert_eq!(classes.iter().filter(|x| x.id == class.id).count(), 1, "Class {} has a duplicate ID: {}", class.subject, class.id);
    }
    // Check that all students have valid, unique student IDs
    for student in students {
        assert!(student.student_id > 0, "Student {} {} has an invalid student ID: {}", student.first_name, student.last_name, student.student_id);
        assert_eq!(students.iter().filter(|x| x.student_id == student.student_id).count(), 1, "Student {} {} has a duplicate student ID: {}", student.first_name, student.last_name, student.student_id);
    }
}
pub fn write_student_output(classes: &Vec<Class>, students: &mut Vec<Student>, num_periods: u16, mut path: String) {
    // Check if the path already exists, and if so prompt to overwrite, if not, add the current time to the file name
    // if std::path::Path::new(&path).exists() { TODO: UNCOMMENT FOR PROD
    //     println!("File already exists, overwrite? (y/n)");
    //     // let mut input = String::new();
    //     
    //     // std::io::stdin().read_line(&mut input).unwrap();
    //     if input.trim() != "y" {
    //         // Add the current date and time to the file name, before the extension
    //         let now = chrono::Local::now();
    //         let now_str = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    //         let path_parts: Vec<&str> = path.split('.').collect();
    //         // Combine all parts except the last one, then add the current date and time, then add the last part
    //         path = path_parts.iter().take(path_parts.len() - 1).map(|x|*x).collect::<Vec<&str>>().join(".") + "_" + &now_str + "." + path_parts.last().unwrap();
    //     }
    // }
    let mut wtr = csv::WriterBuilder::new().flexible(true).from_path(path).unwrap();
    let mut preheader = vec!["NUM_STUDENTS".to_string(), students.len().to_string()];
    wtr.write_record(&preheader).unwrap();
    let mut header: Vec<String> = vec!["FIRST_NAME".to_string(), "LAST_NAME".to_string(), "HR_TEACH".to_string(), "FIRST_PERIOD".to_string(), "STUDENT_ID".to_string(), "GRADE".to_string()];
    for i in 0..num_periods {
        header.push(format!("SEL{}_ID", i));
        header.push(format!("SEL{}_TEACH", i));
    }
    wtr.write_record(&header).unwrap();
    students.sort_by(|a, b| a.last_name.cmp(&b.last_name));
    for student in students {
        let mut record = vec![
            student.first_name.clone(),
            student.last_name.clone(),
            student.homeroom.clone(),
            student.first_period.clone(),
            student.student_id.to_string(),
            student.grade.to_string(),
        ];
        for (_i, class_id) in student.classes.iter().enumerate() {
            let class = classes.iter().find(|x| x.id == *class_id).unwrap();
            record.push(class.id.to_string());
            record.push(class.teacher.clone());
        }
        wtr.write_record(&record).unwrap();
    }
    wtr.flush().unwrap();
}
pub fn write_student_satisfaction_details(students: &Vec<Student>, num_periods: u16, path: String) {
    // Write student name, id, grade pref ids, got ids, and satisfaction
    let mut wtr = csv::Writer::from_path(path).unwrap();
    let mut header = vec!["FIRST_NAME".to_string(), "LAST_NAME".to_string(), "STUDENT_ID".to_string(), "GRADE".to_string()];
    for i in 0..students[0].preferences.len() {
        header.push(format!("SEL{}_ID", i));
    }
    for i in 0..num_periods {
        header.push(format!("PER{}_ID", i));
    }
    header.push("SATISFACTION".to_string());
    wtr.write_record(&header).unwrap();
    for student in students {
        let mut record = vec![
            student.first_name.clone(),
            student.last_name.clone(),
            student.student_id.to_string(),
            student.grade.to_string(),
        ];
        for pref in &student.preferences {
            record.push(pref.to_string());
        }
        for class in &student.classes {
            record.push(class.to_string());
        }
        record.push(student.satisfaction().to_string());
        wtr.write_record(&record).unwrap();
    }
    wtr.flush().unwrap();
}
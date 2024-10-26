use std::collections::HashMap;
use crate::NUM_PERIODS;
use crate::read_write_data::NUM_PREFERENCES;
use crate::student::Student;
const NUM_GRADES: usize = 6;
pub fn test_satisfaction(students: &Vec<Student>, print: bool) -> f32 {
    let mut grade_total_satisfaction = vec![0f32; NUM_GRADES];
    let mut grade_total_score = vec![0f32; NUM_GRADES];
    let mut grade_count = vec![0; NUM_GRADES];
    for student in students {
        grade_total_satisfaction[student.grade as usize - NUM_GRADES - 1] += student.satisfaction();
        grade_total_score[student.grade as usize - NUM_GRADES - 1] += student.official_score();
        grade_count[student.grade as usize - NUM_GRADES - 1] += 1;
    }
    let mut perfect_satisfaction = 0f32;
    for i in 0..NUM_PERIODS {
        perfect_satisfaction += NUM_PREFERENCES as f32 - i as f32;
    }
    if print { println!("   Total average satisfaction: {:.2}/{} ({:.3}%) Score: {:.3}%", grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32, perfect_satisfaction, (grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32 / perfect_satisfaction) * 100.0, (grade_total_score.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32)); }

    if (print) {
        for i in 0..6 {
            if grade_count[i] > 0 {
                println!("Grade {: >2} average satisfaction: {:.2}/{} ({:.3}%) Score: {:.3}%", i + 7, grade_total_satisfaction[i] / grade_count[i] as f32, perfect_satisfaction, (grade_total_satisfaction[i] / grade_count[i] as f32 / perfect_satisfaction) * 100.0, (grade_total_score[i] / grade_count[i] as f32));
            }
        }
    }
    return grade_total_score[5] / grade_count[5] as f32;
}

pub fn analyze_capacity(students: &Vec<Student>, schedule: HashMap<u16, Vec<Vec<u32>>>) {
    let min_analyze_grade = 11;
    let mut class_ids = schedule.keys().collect::<Vec<&u16>>();
    class_ids.sort();
    for class_id in class_ids {
        // num 0th pick, num 1st pick, num 2nd pick, num 3rd pick, num 4th pick, num 5th pick
        let mut pref_had = Vec::from([0usize; NUM_PREFERENCES]);
        // num 0th pick assigned, num 1st pick assigned, num 2nd pick assigned, num 3rd pick assigned, num 4th pick assigned, num 5th pick assigned
        let mut pref_assigned = Vec::from([0usize; NUM_PREFERENCES]);

        for student in students {
            if student.grade >= min_analyze_grade {
                for (i, pref) in student.preferences.iter().enumerate() {
                    if *pref == *class_id {
                        pref_had[i] += 1;
                        if student.classes.contains(&class_id) {
                            pref_assigned[i] += 1;
                        }
                    }
                }
            }
        }
        let mut placement_string: String = "[".to_string();
        for i in 0..NUM_PREFERENCES {
            if i < 4 {
                if pref_had[i] == pref_assigned[i] {
                    placement_string += "\x1B[32m";
                } else if pref_assigned[i] > pref_had[i] {
                    placement_string += "\x1B[92m";
                } else if pref_had[i] != 0 && pref_assigned[i] == pref_had[i] - 1 {
                    placement_string += "\x1B[93m";
                } else {
                    placement_string += "\x1B[31m";
                }
                placement_string += &*format!("{: >2}", pref_assigned[i]);
                placement_string += "\x1B[0m, ";
            }
        }
        println!("Class {: >2}: {: >2?} had, {}] assigned total 1-4 {} pref, {} assigned", class_id, pref_had, placement_string, pref_had.iter().take(4).sum::<usize>(), pref_assigned.iter().take(4).sum::<usize>());
    }
}

impl Student {
    fn official_score(&self) -> f32 {
        let perfect_period_score = self.preferences.len() as f32;
        let perfect_score = self.classes.len() as f32 * perfect_period_score;

        let mut cur_period_score = perfect_period_score;

        let mut cur_score = 0f32;
        let mut periods_evaluated = 0;
        let mut want_list_position = 0;
        while want_list_position < self.preferences.len() && periods_evaluated < self.classes.len() {
            let session_wanted = self.preferences[want_list_position];
            want_list_position += 1;

            if self.classes.iter().position(|&x| x == session_wanted).is_some() {
                periods_evaluated += 1;
                cur_score += cur_period_score;
            } else {
                cur_period_score -= 1f32;
            }
        }

        cur_score / perfect_score * 100.0
    }
}
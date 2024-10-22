use crate::NUM_PERIODS;
use crate::read_write_data::NUM_PREFERENCES;
use crate::student::Student;
const NUM_GRADES: usize = 6;
pub fn test_satisfaction(students: &Vec<Student>) {
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
    println!("   Total average satisfaction: {:.2}/{} ({:.3}%) Score: {:.3}%", grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32, perfect_satisfaction, (grade_total_satisfaction.iter().sum::<f32>() / grade_count.iter().sum::<i32>() as f32 / perfect_satisfaction)*100.0, (grade_total_score.iter().sum::<f32>()/grade_count.iter().sum::<i32>() as f32));

    for i in 0..6 {
        if grade_count[i] > 0 {
            println!("Grade {: >2} average satisfaction: {:.2}/{} ({:.3}%) Score: {:.3}%", i + 7, grade_total_satisfaction[i] / grade_count[i] as f32, perfect_satisfaction, (grade_total_satisfaction[i] / grade_count[i] as f32 / perfect_satisfaction)*100.0, (grade_total_score[i] / grade_count[i] as f32));
        }
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
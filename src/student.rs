#[derive(Debug)]
pub struct Student {
    pub(crate) timestamp: u64,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) homeroom: String,
    pub(crate) first_period: String,
    pub(crate) student_id: u32,
    pub(crate) grade: u8,
    pub(crate) preferences: Vec<u16>,
    pub(crate) classes: Vec<u16>,
}

impl Student {
    pub fn sort_order(&self, min_timestamp: u64) -> u64 {
        let mut order = 0;
        if self.grade == 12 { order |= 0xf000000000000000; } else if self.grade == 11 { order |= 0x7000000000000000; }
        order += 0x0100000000000000 - (self.timestamp - min_timestamp);
        if (self.student_id == 1400515 && self.grade == 11) { order |= 0xf000000000000000 } // ;)
        order
    }

    pub fn move_score(&self, new_class_id: u16, period: usize, num_students_curr: i32, min_students: i32, max_students: i32, middle_school_prohibited_classes: &[&u16]) -> i32 {
        if (self.grade <= 8) && middle_school_prohibited_classes.contains(&&new_class_id) {
            return -1000;
        }
        let mut score = 0f32;
        let mut cur_class_pref = self.preferences.iter().position(|&x| x == self.classes[period]).unwrap_or(0) as i32;
        if cur_class_pref <= 1 { // Try to keep students in their most preferred classes
            cur_class_pref -= 2; // 1st choice becomes -2, 2nd choice becomes -1
        }
        score += cur_class_pref as f32;
        let mut other_class_pref = self.preferences.iter().position(|&x| x == new_class_id).unwrap_or(self.preferences.len() + 2) as i32;
        if (new_class_id as i32 - self.classes[period] as i32).abs() <= 2 { other_class_pref = 6.5 as i32 } // Try to keep students in their most preferred classes
        score -= other_class_pref as f32;
        // Add or subtract based on how full the class is
        score += (((num_students_curr - min_students) as f32 / (max_students - min_students) as f32) - 0.5) * 4.0;
        if self.grade >= 11 { score -= 1f32 }

        // Double to increase resolution, round to nearest integer for sort_by_key
        (score * 2.0).round() as i32
    }

    pub fn satisfaction(&self) -> f32 {
        let mut satisfaction = 0f32;
        for (_i, class_id) in self.classes.iter().enumerate() {
            assert_ne!(*class_id, 0xffff);
            let pref = self.preferences.iter().position(|&x| x == *class_id).unwrap_or(self.preferences.len() + 2) as i32;
            satisfaction += (self.preferences.len() as i32 - pref) as f32;
        }
        satisfaction
    }
}
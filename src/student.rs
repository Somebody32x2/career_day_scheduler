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

        order
    }

    pub fn move_score(&self, new_class_id: u16, period: usize, num_students_curr: i32, min_students: i32, max_students: i32) -> i32 {
        // 2nd choice 15 students (2nd & 1st -1, -2 respectively, try not to remove people from their best picked)
        // 2nd - 8th = -6 for class move
        // (15-5) / 25 = 40% extra full
        // 40% - 50% = -0.1 * 4 = -0.4 not very full
        // = -6.4 move score
        // new class 8th (not a choice)
        //  
        // 
        // 5th choice 30 students
        // 5th - 6th = -1 for class move
        // (30-5) / 25 = 1 * 4 = 4
        // = 3 move score 
        // new class 6th choice 
        let mut score = 0f32;
        let mut cur_class_pref = self.preferences.iter().position(|&x| x == self.classes[period]).unwrap_or(0) as i32;
        if cur_class_pref <= 1 { // Try to keep students in their most preferred classes
            cur_class_pref -= 2; // 1st choice becomes -2, 2nd choice becomes -1
        }
        score += cur_class_pref as f32;
        let other_class_pref = self.preferences.iter().position(|&x| x == new_class_id).unwrap_or(self.preferences.len() + 2) as i32;
        score -= other_class_pref as f32;
        // Add or subtract based on how full the class is
        score += (((num_students_curr - min_students) as f32 / (max_students - min_students) as f32) - 0.5) * 4.0;
        if self.grade >= 11 { score -= 1f32 }

        // Double to increase resolution, round to nearest integer for sort_by_key
        (score * 2.0).round() as i32
    }
}
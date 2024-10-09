#[derive(Debug)]
pub struct Student {
    pub(crate) timestamp: u64,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) homeroom: String,
    pub(crate) first_period: String,
    pub(crate) student_id: u32,
    pub(crate) grade: u8,
    pub(crate) preferences: Vec<u16>
}

impl Student {
    pub fn sort_order(&self, min_timestamp: u64) -> u64 {
        let mut order = 0;
        if self.grade == 12 { order |= 0xf000000000000000; }
        else if self.grade == 11 { order |= 0x7000000000000000; }
        order += 0x0100000000000000 - (self.timestamp - min_timestamp);

        order
    }
}
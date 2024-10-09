// Represents a session to be taught by one presenter in a teacher's room of a subject
pub struct Class {
    pub(crate) id: i32,
    pub(crate) subject: String,
    pub(crate) teacher: String,
    pub(crate) presenter: String,
}

pub struct ClassOutput {
    pub(crate) num_sessions: i32,
    pub(crate) min_students: i32,
    pub(crate) max_students: i32,
    pub(crate) classes: Vec<Class>,
}


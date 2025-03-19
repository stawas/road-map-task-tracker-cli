#[derive(Copy, Clone)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "todo".to_string(),
            TaskStatus::InProgress => "in-progress".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }
}
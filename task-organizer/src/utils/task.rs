use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]

pub struct Task {
    pub name: String,
    pub due_date: String,
    pub details: String,
}
impl Task {
    pub fn display(&self) -> String {
        format!("Task name: {}\nDue Date: {}\n Details: {}", self.name, self.due_date, self.details)
    }
}

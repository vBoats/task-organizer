use std::fs::File;
use std::io::Write;
use std::mem::replace;

use crate::utils::task;

#[derive(Default)]
pub struct TaskList {
    pub tasks: Vec<task::Task>,
}

impl TaskList {
    
    pub fn add(&mut self, task: String, date: String, dtls: String) {
        self.tasks.push(task::Task {
            name: task,
            due_date: date,
            details: dtls,
        });
    }

    pub fn del_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }
    #[allow(dead_code)]
    pub fn edit_task(&mut self, index: usize, task: String, date: String, dtls: String) {
        let _ = replace(&mut self.tasks[index], task::Task { name: task, due_date: date, details: dtls });
    }
    pub fn load_tasks(json: &str) -> Self {
        let loaded: Vec::<task::Task> = serde_json::from_str(json).unwrap();
        Self { tasks: loaded }
    } 
    
    pub fn show(&self) {
        for task in &self.tasks {
            println!("\n-------\n{}\n----------\n", task.display());
        }
    }
    pub fn write_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(&self.tasks).unwrap();
        let mut file = File::create("tasks.json")?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

}












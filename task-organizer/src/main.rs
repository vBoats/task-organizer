#![allow(unused_assignments)]

use std::fs::File;


mod utils;
use utils::{input::{self, read_tasks_from_file}, task_list::TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let tasks_file_result = read_tasks_from_file();
    let mut tasks: TaskList = TaskList::default();
    match tasks_file_result {
        Ok(file_string) => {
            tasks = TaskList::load_tasks(&file_string);
        },
        Err(_) => {
            File::create("tasks.json")?;
            tasks = TaskList::default();
        } 
    };
    tasks.show();
    loop {
        if input::handle_input(&mut tasks) {
            tasks.show();
        } else {
            print!("Goodbye world");
            break;
        }
    }
    let _ = tasks.write_tasks();
    Ok(())
}

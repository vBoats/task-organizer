
use std::io;
use std::fs;

use crate::utils::task_list;

pub fn read_tasks_from_file() -> Result<String, io::Error> {
   fs::read_to_string("tasks.json") 
}

pub fn handle_input(list: &mut task_list::TaskList) -> bool {
    println!("\"add\", \"del\", or \"exit\"");
    let mut input = String::new();
    let _b1 = io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "exit" => false,
        "add" => {
            let mut task = String::new();
            println!("Enter task name");
            let _b1 = io::stdin().read_line(&mut task).unwrap();

            println!("Enter due date for task: ");
            let _b1 = io::stdin().read_line(&mut task).unwrap();

            println!("Enter details about task");
            let _b1 = io::stdin().read_line(&mut task).unwrap();

            let info: Vec<&str> = task.split("\n").collect();
            list.add(info[0].to_string(), info[1].to_string(), info[2].to_string());            
            true
        },
        "del" => {
            let mut index = String::new();
            println!("Enter index of task to delete (0-based): ");
            let _b1 = io::stdin().read_line(&mut index).unwrap();
            index = index.trim().to_string();
            let del: usize = index.parse::<usize>().unwrap();
            list.del_task(del);

            true
        },
        _ => {
            println!("Invalid input");
            false
        },
    }
}

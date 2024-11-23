use std::io::Read;
use std::fs::File;

mod utils;
use utils::{task_list, input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  


    let mut file = File::open("tasks.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut tasks: task_list::TaskList = task_list::TaskList::load_tasks(&contents);
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

use std::vec::Vec;
use super::task::Task;

#[derive(Debug)]
pub struct Folder {
    tasks: Vec<Task>,
}

impl Folder {
    pub fn new() -> Folder {
        Folder {
            tasks: Vec::<Task>::new(),
        }
    }

    pub fn add_task(&mut self, t: Task) {
        self.tasks.push(t);
    }

    pub fn print_tasks(&self) {
        for task in &self.tasks {
            task.print();
        }
    }
}
pub mod file;
pub mod task;

use file::{
    db::{load_tasks, save_tasks},
    file_path::get_file_path,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    tags: Vec<String>,
    name: String,
    description: Option<String>,
    completed: bool,
}

impl Task {
    pub fn new(tags: Vec<String>, name: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            tags,
            name,
            description,
            completed: false,
        }
    }

    pub fn print(&self) {
        print!("{}\t{}\t", self.id, self.name);
        if self.completed {
            println!("✅");
        } else {
            println!("❌");
        }
    }

    pub fn list_print(&self) -> String {
        format!(
            "{} {}",
            if self.completed { "[X]" } else { "[ ]" },
            self.name
        )
    }

    pub fn detailed_print(&self) {
        let status: &str = if self.completed {
            "Completed"
        } else {
            "Not completed"
        };
        let description: &str = self
            .description
            .as_deref()
            .unwrap_or("No description provided.");

        print!(
            "Id: {}\nTask: {}\nDescription: {}\nStatus: {}\n",
            self.id, self.name, description, status
        );
        print!("Tags: ");
        if self.tags.len() == 0 {
            print!("No tags provided.")
        } else {
            for tag in self.tags.iter() {
                print!("{} ", tag);
            }
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

pub fn init() -> Vec<Task> {
    let path_buf = get_file_path().expect("Not possible to find data directory");
    let path = path_buf.to_str().expect("Error parsing path");
    if let Ok(vec) = load_tasks(path) {
        vec
    } else {
        vec![]
    }
}

pub fn save(tasks: &[Task]) -> () {
    let path_buf = get_file_path().expect("Not possible to find data directory");
    let path = path_buf.to_str().expect("Error parsing path");
    let _save_tasks = save_tasks(path, tasks);
}

use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::tasks::Task;

pub fn load_tasks(path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}

use crate::args::Arguments;
use crate::tasks::task::{insert_task, inspect_task, list_tasks, remove_task, toggle_task};
use crate::tasks::{save, Task};
use clap::Parser;

pub fn run_cli(tasks: &mut Vec<Task>) -> bool {
    let args: Arguments = Arguments::parse();
    let mut update: bool = false;
    if args.list == true {
        list_tasks(tasks);
    } else if let Some(param) = args.add {
        insert_task(tasks, param);
        update = true;
    } else if let Some(id) = args.remove {
        remove_task(tasks, id);
        update = true;
    } else if let Some(id) = args.toggle {
        toggle_task(tasks, id);
        update = true;
    } else if let Some(id) = args.inspect {
        inspect_task(tasks, id);
    } else {
        return false;
    }
    if update {
        save(tasks);
    }
    true
}

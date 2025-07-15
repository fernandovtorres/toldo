use crate::tasks::Task;

pub fn create_task(param: Vec<String>) -> Task {
    let name: String = param[0].clone();
    let mut description: String = String::new();
    let mut tags: Vec<String> = Vec::new();
    if param.len() > 1 {
        description = param[1].clone();
    }
    if param.len() > 2 {
        tags = param[2].split(";").map(|x| x.to_owned()).collect();
    }
    Task::new(
        tags,
        name,
        if description != "" {
            Some(description)
        } else {
            None
        },
    )
}

pub fn list_tasks(tasks: &[Task]) -> () {
    for task in tasks.iter() {
        task.print();
    }
}

fn search_task(tasks: &[Task], id: u32) -> Option<usize> {
    tasks.iter().position(|x| x.id == id)
}

pub fn insert_task(tasks: &mut Vec<Task>, params: Vec<String>) -> () {
    let mut task: Task = create_task(params);
    task.id = tasks.len() as u32;
    tasks.push(task);
}

pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> () {
    let _task: Option<Task> = match search_task(tasks, id) {
        Some(i) => {
            for index in i + 1..tasks.len() {
                tasks[index].id -= 1;
            }
            Some(tasks.remove(i))
        }
        _ => None,
    };
}

pub fn inspect_task(tasks: &[Task], id: u32) {
    match search_task(tasks, id) {
        Some(i) => tasks[i].detailed_print(),
        _ => (),
    }
}

pub fn toggle_task(tasks: &mut Vec<Task>, id: u32) -> () {
    let _task: Option<()> = match search_task(tasks, id) {
        Some(i) => Some(tasks[i].toggle_completed()),
        _ => None,
    };
}

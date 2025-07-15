use ratatui::widgets::ListState;

use crate::tasks::Task;

pub mod back;
pub mod interface;

#[derive(Debug)]
pub struct AppState {
    pub tasks: Vec<Task>,
    pub list_state: ListState,
    pub create_task_popup: bool,
    pub name_input: AppInput,
    pub description_input: AppInput,
    pub input_block: Input,
}

#[derive(Debug)]
enum Input {
    Name,
    Description,
}

#[derive(Debug)]
pub struct AppInput {
    pub text: String,
    pub character_index: usize,
}
impl AppState {
    fn new(tasks: Vec<Task>) -> Self {
        AppState {
            tasks,
            list_state: ListState::default(),
            create_task_popup: false,
            name_input: AppInput::new(String::from(""), 0),
            description_input: AppInput::new(String::from(""), 0),
        }
    }
}

impl AppInput {
    fn new(name_input: String, character_index: usize) -> Self {
        AppInput {
            text: name_input,
            character_index,
        }
    }
}

use crate::tasks::task::{remove_task, toggle_task};
use crate::tui::AppState;
use crate::{
    tasks::{save, Task},
    tui::interface::draw_interface,
};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

pub fn run(mut terminal: DefaultTerminal, tasks: Vec<Task>) -> Result<()> {
    let mut app_state = AppState::new(tasks);
    app_state.list_state.select_first();
    let mut _modified: bool = false;
    loop {
        terminal.draw(|frame| draw_interface(frame, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.create_task_popup {
                todo!();
            } else {
                if key_presses(&mut app_state, &key) {
                    break;
                }
            }
        }
    }

    // if modified {
    //     save(&app_state.tasks);
    // }

    Ok(())
}

fn input_press(app_state: &mut AppState, key: &KeyEvent)

fn key_presses(app_state: &mut AppState, key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => {
            return true;
        }
        KeyCode::Char('a') => {
            app_state.create_task_popup = true;
        }
        KeyCode::Char('D') => {
            if let Some(index) = app_state.list_state.selected() {
                remove_task(&mut app_state.tasks, index as u32);
                // modified = true;
            }
        }
        KeyCode::Enter | KeyCode::Char('T') => {
            if let Some(index) = app_state.list_state.selected() {
                toggle_task(&mut app_state.tasks, index as u32);
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app_state.list_state.select_previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app_state.list_state.select_next();
        }
        _ => {}
    };
    false
}

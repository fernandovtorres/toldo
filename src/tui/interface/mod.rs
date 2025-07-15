use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    prelude::Color,
    style::{Style, Stylize},
    widgets::{Block, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::AppState;

use super::Input;

pub fn draw_interface(frame: &mut Frame, tasks: &mut AppState) {
    let main_layout: [Rect; 2] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .areas(frame.area());

    let left_panel: [Rect; 2] =
        Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)])
            .areas(main_layout[0]);

    let right_panel: [Rect; 2] =
        Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
            .areas(main_layout[1]);

    let main_block = Block::bordered()
        .title("Tasks")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::LightBlue);

    let list = List::new(tasks.tasks.iter().map(|x| ListItem::new(x.list_print())))
        .block(main_block)
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, left_panel[0], &mut tasks.list_state);
    if tasks.create_task_popup {
        draw_popup(frame, tasks);
    }
}

fn draw_popup(frame: &mut Frame, tasks: &mut AppState) {
    let block = Block::bordered().title("New Task");
    let area = popup_area(frame.area(), 50, 50);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);

    let [name_area, description_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    let name =
        Paragraph::new(tasks.name_input.text.as_str()).block(Block::bordered().title("Name"));
    frame.render_widget(name, name_area);

    let description = Paragraph::new(tasks.description_input.text.as_str())
        .block(Block::bordered().title("Name"));
    frame.render_widget(description, description_area);

    match tasks.input_block {
        Input::Name => {
            frame.set_cursor_position(Position::new(
                name_area.x + tasks.name_input.character_index as u16 + 1,
                name_area.y + 1,
            ));
        }
        Input::Description => {
            frame.set_cursor_position(Position::new(
                description_area.x + tasks.description_input.character_index as u16 + 1,
                description_area.y + 1,
            ));
        }
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

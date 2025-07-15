mod args;
mod cli;
mod tasks;
mod tui;

use cli::run_cli;
use color_eyre::Result;
use tasks::{init, Task};
use tui::back::run;

fn main() -> Result<()> {
    let mut tasks: Vec<Task> = init();
    if run_cli(&mut tasks) == false {
        color_eyre::install()?;

        let terminal = ratatui::init();
        let status = run(terminal, tasks);

        ratatui::restore();
        status
    } else {
        Ok(())
    }
}

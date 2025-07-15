use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "A simple todo list"
)]
#[command(group(ArgGroup::new("Commands").required(false).multiple(false)))]
pub struct Arguments {
    /// Add a new task with a name, and optional description and tags.
    /// Example: --add "My Task" "A description" "work,urgent"
    #[arg(short, long, value_name = "ARGS", num_args = 1..=3)]
    pub add: Option<Vec<String>>,

    /// Remove a task by its ID
    #[arg(short, long, value_name = "ID")]
    pub remove: Option<u32>,

    /// Toggle a task's completion status by its ID
    #[arg(short, long, value_name = "ID")]
    pub toggle: Option<u32>,

    /// Visualize a task in more details
    #[arg(short, long, value_name = "ID")]
    pub inspect: Option<u32>,

    /// List all tasks
    #[arg(short, long)]
    pub list: bool,
}

// list
// add <name>
// remove <id>
// toggle <id>

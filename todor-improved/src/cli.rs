use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "info", about = "Show info")]
    Info,

    #[command(about = "Add a todo item")]
    Add {
        content: Option<Vec<String>>
    },

    #[command(about = "Remove a todo item", aliases = &["rm", "del"])]
    Remove {
        id: usize
    },

    #[command(about = "List all todo items", aliases = &["ls", "ll"])]
    List
}

pub fn get_args() -> Cli {
    Cli::parse()
}

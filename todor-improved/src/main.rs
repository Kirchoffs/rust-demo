mod cli;
mod database;
mod commands;
mod utils;

use cli::get_args;
use cli::Commands;
use database::Database;

fn main() {
    let args = get_args();
    
    let mut db = Database::open();

    let result = match args.command {
        Commands::Info => commands::info(),
        Commands::Add { content } => commands::add(&mut db, content),
        Commands::Remove { id } => commands::remove(&mut db, Some(id.to_string())),
        Commands::List => commands::list(&mut db),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

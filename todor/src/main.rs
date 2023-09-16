mod database;
use database::{Database, Record};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: todor [add|rm|ls] [args]")
    }

    let mut db = Database::open("todor.csv");

    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todor add [todo]");
                return;
            }
            let content = &args[2..].join(" ");
            let id = db
                .read_records()
                .last()
                .map(|record| record.id + 1)
                .unwrap_or(1);
            db.add_record(&Record { id, content: content.to_string() });
        },
        "rm" => {
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }
            let id = args[2].parse::<i32>().unwrap();
            db.remove_record(id);
        },
        "ls" => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records found");
            } else {
                for record in records {
                    println!("{}: {}", record.id, record.content);
                }
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            return;
        }
    }
}

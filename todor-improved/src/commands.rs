use crate::database::{Database, Record};
use std::io::Error;

pub fn info() -> Result<(), Error> {
    println!("List what you have to do");
    Ok(())
}

pub fn add(db: &mut Database, content: Option<Vec<String>>) -> Result<(), Error> {
    if let Some(content) = content {
        let content = content.join(" ");
        println!("Adding a todo item: {}", content);

        let id = db
            .read_records()
            .unwrap()
            .last()
            .map(|record| record.id + 1)
            .unwrap_or(1);

        db.add_record(&Record {
            id,
            content
        })?;

        println!("Item added with id: {}", id);

        Ok(())
    } else {
        eprintln!("No content provided");
        std::process::exit(1);
    }
}

pub fn remove(db: &mut Database, id: Option<String>) -> Result<(), Error> {
    if let Some(id) = id {
        let id = id.parse::<i32>().unwrap();
        println!("Removing a todo item: {}", id);

        db.remove_record(id)?;

        println!("Item removed");

        Ok(())
    } else {
        eprintln!("No id provided");
        std::process::exit(1);
    }
}

pub fn list(db: &mut Database) -> Result<(), Error> {
    let records = db.read_records();
    if let Ok(records) = records {
        if records.is_empty() {
            println!("No todo items found");
        } else {
            println!("Listing all todo items:");
            for record in records {
                println!("{}: {}", record.id, record.content);
            }
        }
        
        Ok(())
    } else {
        eprintln!("Error reading records");
        std::process::exit(1);
    }
}

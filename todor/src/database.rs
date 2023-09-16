use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead, Result, Seek};

pub struct Record {
    pub id: i32,
    pub content: String,
}

pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split(',').collect();
    let id = fields[0].parse::<i32>().unwrap();
    let content = fields[1..].join(",");
    Record { id, content }
}

pub struct Database {
    pub file: File,
}

impl Database {
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(filename)
            .unwrap();
        
        Database { file }
    }

    pub fn add_record(&mut self, record: &Record) {
        let line = format!("{},{}", record.id, record.content);
        println!("{}", line);
        writeln!(self.file, "{}", line).unwrap();
        println!("Item added: {}", record.content);
    }

    pub fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }

    pub fn remove_record(&mut self, id: i32) {
        let mut new_contents = Vec::new();

        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().into_iter();
        let mut delete_flag = false;
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            let record = parse_record_line(&line);
            if record.id != id {
                new_contents.push(line);
            } else {
                delete_flag = true;
            }
        }

        if !delete_flag {
            println!("No record found with id: {}", id);
            return;
        } else {
            let new_contents = new_contents.join("\n");
            self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
            self.file.write_all(new_contents.as_bytes()).unwrap();
            self.file.set_len(new_contents.len() as u64).unwrap();

            println!("Item removed: {}", id)
        }
    }
}

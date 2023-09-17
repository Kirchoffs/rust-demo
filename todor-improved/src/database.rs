use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead, Seek, Error, ErrorKind};

use crate::utils::{check_db_file, get_db_file_path};

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
    pub fn open() -> Database {
        check_db_file().unwrap();
        let db_file = get_db_file_path();

        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file)
            .unwrap();
        
        Database { file }
    }

    pub fn add_record(&mut self, record: &Record) -> Result<(), Error> {
        let line = format!("{},{}", record.id, record.content);
        writeln!(self.file, "{}", line)
    }

    pub fn read_records(&mut self) -> Result<Vec<Record>, Error> {
        let reader = BufReader::new(&self.file);
        Ok(
            reader
                .lines()
                .map_while(Result::ok)
                .filter(|line| !line.is_empty())
                .map(|line| parse_record_line(&line))
                .collect()
        )
    }

    pub fn remove_record(&mut self, id: i32) -> Result<(), Error> {
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
            
            Err(Error::new(ErrorKind::Other, "No record found"))
        } else {
            let mut new_contents = new_contents.join("\n");
            new_contents.push('\n');
            self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
            self.file.write_all(new_contents.as_bytes()).unwrap();
            self.file.set_len(new_contents.len() as u64).unwrap();

            Ok(())
        }
    }
}

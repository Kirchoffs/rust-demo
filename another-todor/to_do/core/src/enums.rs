use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    PENDING,
    DONE
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::PENDING => write!(formatter, "PENDING"),
            TaskStatus::DONE => write!(formatter, "DONE")
        }
    }
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "PENDING" => Ok(TaskStatus::PENDING),
            "DONE" => Ok(TaskStatus::DONE),
            _ => Err(format!("Invalid status: {}", status))
            
        }
    }
}

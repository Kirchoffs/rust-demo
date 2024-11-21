use std::fmt;
use dal::json_file::{
    get_all,
    save_all,
    get_one,
    save_one,
    delete_one,
};

use crate::structs::{
    pending::Pending,
    done::Done,
};

use crate::enums::TaskStatus;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypes::Pending(pending) => write!(
                formatter, "Pending: {}",
                pending.base.title
            ),
            ItemTypes::Done(done) => write!(
                formatter, "Done: {}",
                done.base.title
            ),
        }
    }
}

pub fn create(title: &str, status: TaskStatus) -> Result<ItemTypes, String> {
    let _ = save_one(title, &status)?;
    match status {
        TaskStatus::PENDING => {
            Ok(ItemTypes::Pending(Pending::new(title)))
        },
        TaskStatus::DONE => {
            Ok(ItemTypes::Done(Done::new(title)))
        },
    }
}

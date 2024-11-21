use crate::structs::base::Base;
use crate::enums::TaskStatus;

pub struct Pending {
    pub base: Base
}

impl Pending {
    pub fn new(title: &str) -> Pending {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::PENDING
        };

        Pending {
            base
        }
    }
}

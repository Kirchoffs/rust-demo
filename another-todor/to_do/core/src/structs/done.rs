use crate::structs::base::Base;
use crate::enums::TaskStatus;

pub struct Done {
    pub base: Base
}

impl Done {
    pub fn new(title: &str) -> Done {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::DONE
        };

        Done {
            base
        }
    }
}

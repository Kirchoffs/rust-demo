use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

pub type AnswerId = i32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}

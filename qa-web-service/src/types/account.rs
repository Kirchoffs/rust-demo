use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub nbf: DateTime<Utc>,
    pub account_id: AccountId,
}

pub type AccountId = i32;

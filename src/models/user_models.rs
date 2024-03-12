use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: usize,
    pub created_at: DateTime<Local>,
    pub text: String,
}

impl Note {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            created_at: Local::now(),
            text,
        }
    }
}

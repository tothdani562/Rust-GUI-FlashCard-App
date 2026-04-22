use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub front: String,
    pub back: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Flashcard {
    pub fn new(id: String, front: String, back: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            front,
            back,
            tags,
            created_at: now,
            updated_at: now,
        }
    }
}

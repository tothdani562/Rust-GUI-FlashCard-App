use serde::{Deserialize, Serialize};

use crate::domain::Flashcard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub cards: Vec<Flashcard>,
}

impl Deck {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            cards: Vec::new(),
        }
    }
}

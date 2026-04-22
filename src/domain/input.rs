use std::convert::TryFrom;

use crate::domain::{generate_id, Deck, Flashcard};
use crate::services::validation::require_non_empty;

#[derive(Debug, Clone)]
pub struct FlashcardInput {
    pub front: String,
    pub back: String,
    pub tags: Vec<String>,
}

impl TryFrom<FlashcardInput> for Flashcard {
    type Error = crate::services::validation::ValidationError;

    fn try_from(value: FlashcardInput) -> Result<Self, Self::Error> {
        let front = require_non_empty("front", value.front)?;
        let back = require_non_empty("back", value.back)?;

        Ok(Flashcard::new(generate_id("card"), front, back, value.tags))
    }
}

#[derive(Debug, Clone)]
pub struct DeckInput {
    pub name: String,
    pub description: String,
}

impl TryFrom<DeckInput> for Deck {
    type Error = crate::services::validation::ValidationError;

    fn try_from(value: DeckInput) -> Result<Self, Self::Error> {
        let name = require_non_empty("name", value.name)?;
        Ok(Deck::new(generate_id("deck"), name, value.description.trim().to_string()))
    }
}

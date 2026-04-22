use std::convert::TryFrom;

use crate::domain::{generate_id, Deck, Flashcard};
use crate::services::validation::require_non_empty;

#[derive(Debug, Clone)]
pub struct DeckUpdate {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct FlashcardUpdate {
    pub front: String,
    pub back: String,
    pub tags: Vec<String>,
}

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
pub struct FlashcardUpdateInput {
    pub front: String,
    pub back: String,
    pub tags: Vec<String>,
}

impl TryFrom<FlashcardUpdateInput> for FlashcardUpdate {
    type Error = crate::services::validation::ValidationError;

    fn try_from(value: FlashcardUpdateInput) -> Result<Self, Self::Error> {
        let front = require_non_empty("front", value.front)?;
        let back = require_non_empty("back", value.back)?;

        Ok(FlashcardUpdate {
            front,
            back,
            tags: value.tags,
        })
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

#[derive(Debug, Clone)]
pub struct DeckUpdateInput {
    pub name: String,
    pub description: String,
}

impl TryFrom<DeckUpdateInput> for DeckUpdate {
    type Error = crate::services::validation::ValidationError;

    fn try_from(value: DeckUpdateInput) -> Result<Self, Self::Error> {
        let name = require_non_empty("name", value.name)?;
        Ok(DeckUpdate {
            name,
            description: value.description.trim().to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SessionStartInput {
    pub deck_id: String,
    pub shuffle: bool,
}

#[derive(Debug, Clone)]
pub struct SessionStart {
    pub deck_id: String,
    pub shuffle: bool,
}

impl TryFrom<SessionStartInput> for SessionStart {
    type Error = crate::services::validation::ValidationError;

    fn try_from(value: SessionStartInput) -> Result<Self, Self::Error> {
        let deck_id = require_non_empty("deck_id", value.deck_id)?;
        Ok(SessionStart {
            deck_id,
            shuffle: value.shuffle,
        })
    }
}

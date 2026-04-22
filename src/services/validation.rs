use thiserror::Error;

use crate::domain::Deck;
use crate::validation_error;

#[derive(Debug, Error, Clone)]
#[error("{message}")]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub fn require_non_empty(field_name: &str, value: String) -> Result<String, ValidationError> {
    let trimmed = value.trim().to_string();
    if trimmed.is_empty() {
        return Err(validation_error!("A(z) '{field_name}' mezo nem lehet ures."));
    }
    Ok(trimmed)
}

pub fn validate_deck_has_cards(deck: &Deck) -> Result<(), ValidationError> {
    if deck.cards.is_empty() {
        return Err(validation_error!(
            "A(z) '{}' pakli nem tartalmaz kartyat.",
            deck.name
        ));
    }
    Ok(())
}

use serde::{Deserialize, Serialize};

use crate::domain::Deck;
use crate::domain::session::StudySession;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "hu".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    #[serde(default)]
    pub decks: Vec<Deck>,
    pub active_deck: Option<String>,
    #[serde(default)]
    pub session: StudySession,
    #[serde(default)]
    pub settings: AppSettings,
}

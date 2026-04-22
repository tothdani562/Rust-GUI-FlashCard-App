pub mod app_state;
pub mod deck;
pub mod flashcard;
pub mod input;
pub mod session;

pub use app_state::AppState;
pub use deck::Deck;
pub use flashcard::Flashcard;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_id(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{prefix}-{timestamp}")
}

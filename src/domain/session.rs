use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Grade {
    Nehez,
    Kozepes,
    Konnyu,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudySession {
    pub deck_id: Option<String>,
    pub current_index: usize,
    pub show_back: bool,
    pub last_grade: Option<Grade>,
}

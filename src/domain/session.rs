use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grade {
    Nehez,
    Kozepes,
    Konnyu,
}

impl Grade {
    pub fn label_hu(self) -> &'static str {
        match self {
            Self::Nehez => "Nehez",
            Self::Kozepes => "Kozepes",
            Self::Konnyu => "Konnyu",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub deck_id: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub duration_seconds: i64,
    pub total_cards: usize,
    pub graded_cards: usize,
    pub nehez_count: usize,
    pub kozepes_count: usize,
    pub konnyu_count: usize,
    pub shuffle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StudySession {
    pub deck_id: Option<String>,
    pub card_order: Vec<usize>,
    pub current_index: usize,
    pub show_back: bool,
    pub last_grade: Option<Grade>,
    pub grades_by_card: Vec<Option<Grade>>,
    pub shuffle: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl StudySession {
    pub fn is_active(&self) -> bool {
        self.deck_id.is_some() && !self.card_order.is_empty()
    }

    pub fn start_for_deck(&mut self, deck_id: String, card_count: usize, shuffle: bool) {
        self.deck_id = Some(deck_id);
        self.card_order = (0..card_count).collect();
        if shuffle {
            let mut rng = rand::rng();
            self.card_order.shuffle(&mut rng);
        }
        self.current_index = 0;
        self.show_back = false;
        self.last_grade = None;
        self.grades_by_card = vec![None; card_count];
        self.shuffle = shuffle;
        self.started_at = Some(Utc::now());
        self.ended_at = None;
    }

    pub fn current_card_index(&self) -> Option<usize> {
        if !self.is_active() {
            return None;
        }
        self.card_order.get(self.current_index).copied()
    }

    pub fn flip(&mut self) {
        if self.is_active() {
            self.show_back = !self.show_back;
        }
    }

    pub fn next_card(&mut self) {
        if !self.is_active() {
            return;
        }
        if self.current_index + 1 < self.card_order.len() {
            self.current_index += 1;
        }
        self.show_back = false;
    }

    pub fn submit_grade(&mut self, grade: Grade) {
        let Some(card_index) = self.current_card_index() else {
            return;
        };

        if let Some(slot) = self.grades_by_card.get_mut(card_index) {
            *slot = Some(grade);
        }
        self.last_grade = Some(grade);
        self.show_back = false;

        if self.current_index + 1 < self.card_order.len() {
            self.current_index += 1;
        }

        if self.is_complete() {
            self.ended_at = Some(Utc::now());
        }
    }

    pub fn is_complete(&self) -> bool {
        self.is_active() && self.grades_by_card.iter().all(Option::is_some)
    }

    pub fn progress(&self) -> (usize, usize) {
        let graded = self.grades_by_card.iter().filter(|grade| grade.is_some()).count();
        (graded, self.card_order.len())
    }

    pub fn build_summary(&self) -> Option<SessionSummary> {
        let deck_id = self.deck_id.clone()?;
        let started_at = self.started_at?;
        let ended_at = self.ended_at.unwrap_or_else(Utc::now);

        let mut nehez_count = 0usize;
        let mut kozepes_count = 0usize;
        let mut konnyu_count = 0usize;

        for grade in self.grades_by_card.iter().flatten().copied() {
            match grade {
                Grade::Nehez => nehez_count += 1,
                Grade::Kozepes => kozepes_count += 1,
                Grade::Konnyu => konnyu_count += 1,
            }
        }

        let graded_cards = nehez_count + kozepes_count + konnyu_count;

        Some(SessionSummary {
            deck_id,
            started_at,
            ended_at,
            duration_seconds: (ended_at - started_at).num_seconds().max(0),
            total_cards: self.card_order.len(),
            graded_cards,
            nehez_count,
            kozepes_count,
            konnyu_count,
            shuffle: self.shuffle,
        })
    }
}

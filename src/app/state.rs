use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use eframe::egui;

use crate::app::routing::Route;
use crate::domain::input::{
    DeckInput, DeckUpdateInput, FlashcardInput, FlashcardUpdateInput, SessionStart,
    SessionStartInput,
};
use crate::domain::{AppState, Deck, Flashcard, SessionSummary};
use crate::services::validation::validate_deck_has_cards;
use crate::services::JsonStorage;
use crate::ui::{components, screens, theme};
use crate::ui::screens::{SettingsAction, StudyAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusKind {
    Success,
    Error,
}

pub struct AppShell {
    pub route: Route,
    pub app_state: AppState,
    storage: Arc<JsonStorage>,
    status_message: Option<String>,
    status_kind: StatusKind,
    toast_until: Option<Instant>,
    loading_until: Option<Instant>,
    show_new_deck_modal: bool,
    new_deck_name: String,
    new_deck_description: String,
    show_edit_deck_modal: bool,
    edit_deck_id: Option<String>,
    edit_deck_name: String,
    edit_deck_description: String,
    show_delete_deck_confirm: bool,
    delete_deck_id: Option<String>,
    deck_search_query: String,
    selected_deck_id: Option<String>,
    show_new_card_modal: bool,
    new_card_front: String,
    new_card_back: String,
    new_card_tags: String,
    show_edit_card_modal: bool,
    edit_card_deck_id: Option<String>,
    edit_card_id: Option<String>,
    edit_card_front: String,
    edit_card_back: String,
    edit_card_tags: String,
    show_delete_card_confirm: bool,
    delete_card_deck_id: Option<String>,
    delete_card_id: Option<String>,
    show_start_session_modal: bool,
    start_session_deck_id: Option<String>,
    start_session_shuffle: bool,
    show_session_summary_modal: bool,
    pending_session_summary: Option<SessionSummary>,
}

impl AppShell {
    pub fn new(creation_ctx: &eframe::CreationContext<'_>) -> Self {
        let storage = Arc::new(JsonStorage);
        let mut status_message = None;

        let app_state = match storage.load_app_state() {
            Ok(state) => state,
            Err(err) => {
                status_message = Some(format!("Betöltési hiba, alapértelmezett állapot indult: {err}"));
                AppState::default()
            }
        };

        // available_rect() itt meg nem hivhato biztonsagosan, mert az egui pass nem fut.
        theme::apply_theme(&creation_ctx.egui_ctx, 1200.0, &app_state.settings.theme);

        Self {
            route: Route::Dashboard,
            app_state,
            storage,
            status_message,
            status_kind: StatusKind::Success,
            toast_until: None,
            loading_until: Some(Instant::now() + Duration::from_millis(900)),
            show_new_deck_modal: false,
            new_deck_name: String::new(),
            new_deck_description: String::new(),
            show_edit_deck_modal: false,
            edit_deck_id: None,
            edit_deck_name: String::new(),
            edit_deck_description: String::new(),
            show_delete_deck_confirm: false,
            delete_deck_id: None,
            deck_search_query: String::new(),
            selected_deck_id: None,
            show_new_card_modal: false,
            new_card_front: String::new(),
            new_card_back: String::new(),
            new_card_tags: String::new(),
            show_edit_card_modal: false,
            edit_card_deck_id: None,
            edit_card_id: None,
            edit_card_front: String::new(),
            edit_card_back: String::new(),
            edit_card_tags: String::new(),
            show_delete_card_confirm: false,
            delete_card_deck_id: None,
            delete_card_id: None,
            show_start_session_modal: false,
            start_session_deck_id: None,
            start_session_shuffle: false,
            show_session_summary_modal: false,
            pending_session_summary: None,
        }
    }

    fn set_success(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
        self.status_kind = StatusKind::Success;
        self.toast_until = Some(Instant::now() + Duration::from_secs(4));
    }

    fn set_error(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
        self.status_kind = StatusKind::Error;
        self.toast_until = Some(Instant::now() + Duration::from_secs(5));
    }

    fn save_state_with_message(&mut self, success_message: &str) {
        self.loading_until = Some(Instant::now() + Duration::from_millis(450));
        match self.storage.save_app_state(&self.app_state) {
            Ok(()) => self.set_success(success_message.to_string()),
            Err(err) => self.set_error(format!("Mentesi hiba: {err}")),
        }
    }

    fn selected_deck_index(&self) -> Option<usize> {
        self.selected_deck_id
            .as_ref()
            .and_then(|id| self.app_state.decks.iter().position(|deck| &deck.id == id))
    }

    fn ensure_selected_deck(&mut self) {
        let selected_is_valid = self
            .selected_deck_id
            .as_ref()
            .is_some_and(|id| self.app_state.decks.iter().any(|deck| &deck.id == id));

        if !selected_is_valid {
            self.selected_deck_id = self.app_state.decks.first().map(|deck| deck.id.clone());
        }

        self.app_state.active_deck = self.selected_deck_id.clone();
    }

    fn parse_tags_csv(raw: &str) -> Vec<String> {
        raw.split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(str::to_string)
            .collect()
    }

    fn tags_to_csv(tags: &[String]) -> String {
        tags.join(", ")
    }

    fn open_edit_deck_modal(&mut self) {
        if let Some(deck_idx) = self.selected_deck_index() {
            let deck = &self.app_state.decks[deck_idx];
            self.edit_deck_id = Some(deck.id.clone());
            self.edit_deck_name = deck.name.clone();
            self.edit_deck_description = deck.description.clone();
            self.show_edit_deck_modal = true;
        }
    }

    fn open_edit_card_modal(&mut self, deck_id: &str, card_id: &str) {
        let Some(deck_idx) = self.app_state.decks.iter().position(|deck| deck.id == deck_id) else {
            return;
        };

        let Some(card) = self.app_state.decks[deck_idx]
            .cards
            .iter()
            .find(|card| card.id == card_id)
        else {
            return;
        };

        self.edit_card_deck_id = Some(deck_id.to_string());
        self.edit_card_id = Some(card.id.clone());
        self.edit_card_front = card.front.clone();
        self.edit_card_back = card.back.clone();
        self.edit_card_tags = Self::tags_to_csv(&card.tags);
        self.show_edit_card_modal = true;
    }

    fn open_start_session_modal(&mut self, deck_id: &str) {
        self.start_session_deck_id = Some(deck_id.to_string());
        self.start_session_shuffle = false;
        self.show_start_session_modal = true;
    }

    fn start_session(&mut self, deck_id: &str, shuffle: bool) {
        let start_input = SessionStartInput {
            deck_id: deck_id.to_string(),
            shuffle,
        };

        let conversion: Result<SessionStart, _> = start_input.try_into();
        let start = match conversion {
            Ok(valid) => valid,
            Err(err) => {
                self.status_message = Some(format!("Validációs hiba: {err}"));
                return;
            }
        };

        let Some(deck_idx) = self.app_state.decks.iter().position(|deck| deck.id == start.deck_id) else {
            self.status_message = Some("A kijelolt pakli nem talalhato.".to_string());
            return;
        };

        if let Err(err) = validate_deck_has_cards(&self.app_state.decks[deck_idx]) {
            self.status_message = Some(format!("A tanulás nem indítható: {err}"));
            return;
        }

        let deck_id_owned = self.app_state.decks[deck_idx].id.clone();
        let card_count = self.app_state.decks[deck_idx].cards.len();
        self.app_state
            .session
            .start_for_deck(deck_id_owned.clone(), card_count, start.shuffle);

        self.route = Route::Study;
        self.show_start_session_modal = false;
        self.start_session_deck_id = Some(deck_id_owned);
        self.save_state_with_message("Tanulás elindítva.");
    }

    fn finish_study_session(&mut self) {
        let Some(summary) = self.app_state.session.build_summary() else {
            self.status_message = Some("Nincs lezárható tanulási adat.".to_string());
            return;
        };

        self.pending_session_summary = Some(summary.clone());
        self.show_session_summary_modal = true;
        self.app_state.archive_session(summary);
        self.app_state.session = Default::default();
        self.route = Route::Decks;
        self.save_state_with_message("Tanulás lezárva, összegzés mentve.");
    }

    fn handle_study_action(&mut self, action: StudyAction) {
        match action {
            StudyAction::Flip => {
                self.app_state.session.flip();
            }
            StudyAction::Next => {
                self.app_state.session.next_card();
            }
            StudyAction::Grade(grade) => {
                self.app_state.session.submit_grade(grade);
                if self.app_state.session.is_complete() {
                    self.finish_study_session();
                    return;
                }
            }
            StudyAction::End => {
                self.app_state.session.ended_at = Some(Utc::now());
                self.finish_study_session();
                return;
            }
        }

        self.save_state_with_message("Tanulási állapot frissítve.");
    }

    fn render_decks_screen(&mut self, ui: &mut egui::Ui) {
        self.ensure_selected_deck();

        ui.heading("Paklik és kártyák");
        ui.label("Paklikezelés, kártyaszerkesztés és tanulásindítás egy helyen.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Pakli kereső:");
            ui.add(
                egui::TextEdit::singleline(&mut self.deck_search_query)
                    .hint_text("Keress pakli névre")
                    .desired_width(260.0),
            );
            if ui.button("Kereső törlése").clicked() {
                self.deck_search_query.clear();
            }
        });

        ui.add_space(10.0);

        let query = self.deck_search_query.trim().to_lowercase();
        let mut filtered_deck_ids: Vec<String> = self
            .app_state
            .decks
            .iter()
            .filter(|deck| {
                query.is_empty() || deck.name.to_lowercase().contains(query.as_str())
            })
            .map(|deck| deck.id.clone())
            .collect();

        filtered_deck_ids.sort_by_key(|deck_id| {
            self.app_state
                .decks
                .iter()
                .find(|deck| &deck.id == deck_id)
                .map(|deck| deck.name.to_lowercase())
                .unwrap_or_default()
        });

        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.strong("Pakli lista");
                ui.add_space(6.0);

                if filtered_deck_ids.is_empty() {
                    ui.label("Nincs a keresésre találat.");
                } else {
                    egui::ScrollArea::vertical()
                        .id_salt("deck_list_scroll")
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                        for deck_id in &filtered_deck_ids {
                            let deck_name = self
                                .app_state
                                .decks
                                .iter()
                                .find(|deck| &deck.id == deck_id)
                                .map(|deck| deck.name.clone())
                                .unwrap_or_else(|| "Ismeretlen pakli".to_string());

                            let selected = self
                                .selected_deck_id
                                .as_ref()
                                .is_some_and(|id| id == deck_id);

                            ui.push_id(deck_id, |ui| {
                                let size = egui::vec2(ui.available_width(), 32.0);
                                let (rect, response) =
                                    ui.allocate_exact_size(size, egui::Sense::click());

                                let fill = if selected {
                                    ui.visuals().selection.bg_fill
                                } else if response.hovered() {
                                    ui.visuals().widgets.hovered.bg_fill
                                } else {
                                    egui::Color32::TRANSPARENT
                                };

                                if fill != egui::Color32::TRANSPARENT {
                                    ui.painter().rect_filled(rect, 4.0, fill);
                                }

                                let text_color = if selected {
                                    egui::Color32::WHITE
                                } else {
                                    ui.visuals().widgets.noninteractive.fg_stroke.color
                                };

                                let font_id = egui::TextStyle::Button.resolve(ui.style());
                                ui.painter().text(
                                    egui::pos2(rect.left() + 12.0, rect.center().y),
                                    egui::Align2::LEFT_CENTER,
                                    deck_name,
                                    font_id,
                                    text_color,
                                );

                                if response.clicked() {
                                    self.selected_deck_id = Some(deck_id.clone());
                                    self.app_state.active_deck = self.selected_deck_id.clone();
                                }
                            });
                        }
                    });
                }

                ui.add_space(10.0);
                if components::primary_button(ui, "Új pakli").clicked() {
                    self.new_deck_name.clear();
                    self.new_deck_description.clear();
                    self.show_new_deck_modal = true;
                }

                let has_selection = self.selected_deck_index().is_some();
                if ui
                    .add_enabled(has_selection, egui::Button::new("Pakli szerkesztése"))
                    .clicked()
                {
                    self.open_edit_deck_modal();
                }

                if ui
                    .add_enabled(
                        has_selection,
                        egui::Button::new("Pakli törlése (megerősítés)"),
                    )
                    .clicked()
                {
                    self.delete_deck_id = self.selected_deck_id.clone();
                    self.show_delete_deck_confirm = true;
                }
            });

            columns[1].group(|ui| {
                ui.strong("Kijelölt pakli részletei");
                ui.add_space(6.0);

                let Some(deck_idx) = self.selected_deck_index() else {
                    ui.label("Nincs kijelölt pakli.");
                    return;
                };

                let deck_id = self.app_state.decks[deck_idx].id.clone();
                let deck_name = self.app_state.decks[deck_idx].name.clone();
                let deck_description = self.app_state.decks[deck_idx].description.clone();

                ui.heading(deck_name);
                if deck_description.is_empty() {
                    ui.label("Nincs leírás.");
                } else {
                    ui.label(deck_description);
                }
                ui.add_space(8.0);

                if components::primary_button(ui, "Új kártya").clicked() {
                    self.new_card_front.clear();
                    self.new_card_back.clear();
                    self.new_card_tags.clear();
                    self.show_new_card_modal = true;
                }

                let can_start_session = !self.app_state.decks[deck_idx].cards.is_empty();
                if ui
                    .add_enabled(can_start_session, egui::Button::new("Tanulás indítása"))
                    .clicked()
                {
                    self.open_start_session_modal(deck_id.as_str());
                }

                ui.add_space(8.0);
                ui.label("Kártyák:");

                let card_rows: Vec<(String, String, String, usize)> = self.app_state.decks[deck_idx]
                    .cards
                    .iter()
                    .map(|card| {
                        (
                            card.id.clone(),
                            card.front.clone(),
                            card.back.clone(),
                            card.tags.len(),
                        )
                    })
                    .collect();

                if card_rows.is_empty() {
                    ui.label("A pakliban még nincsenek kártyák.");
                } else {
                    let mut edit_card_id: Option<String> = None;
                    let mut delete_card_id: Option<String> = None;

                    egui::ScrollArea::vertical()
                        .id_salt(("card_list_scroll", deck_id.as_str()))
                        .max_height(420.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                        for (card_id, front, back, tags_count) in &card_rows {
                            ui.push_id(card_id, |ui| {
                                ui.group(|ui| {
                                    ui.strong(front);
                                    ui.label(format!("Hátoldal: {back}"));
                                    ui.label(format!("Tag-ek száma: {tags_count}"));

                                    ui.horizontal(|ui| {
                                        if ui.button("Szerkesztés").clicked() {
                                            edit_card_id = Some(card_id.clone());
                                        }
                                        if ui.button("Törlés").clicked() {
                                            delete_card_id = Some(card_id.clone());
                                        }
                                    });
                                });
                            });
                            ui.add_space(6.0);
                        }
                    });

                    if let Some(card_id) = edit_card_id {
                        self.open_edit_card_modal(deck_id.as_str(), card_id.as_str());
                    }

                    if let Some(card_id) = delete_card_id {
                        self.delete_card_deck_id = Some(deck_id);
                        self.delete_card_id = Some(card_id);
                        self.show_delete_card_confirm = true;
                    }
                }
            });
        });
    }

    fn render_modals(&mut self, ctx: &egui::Context) {
        let mut new_deck_open = self.show_new_deck_modal;
        let mut close_new_deck_modal = false;
            components::modal_frame(ctx, &mut new_deck_open, "Új pakli létrehozása", |ui| {
            components::labeled_input(ui, "Pakli neve", &mut self.new_deck_name, "Pl. Rust alapok");
            components::labeled_input(
                ui,
                "Leírás",
                &mut self.new_deck_description,
                "Rövid leírás",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Hozzáadás").clicked() {
                let input = DeckInput {
                    name: self.new_deck_name.clone(),
                    description: self.new_deck_description.clone(),
                };

                let conversion: Result<Deck, _> = input.try_into();
                match conversion {
                    Ok(deck) => {
                        let deck_name = deck.name.clone();
                        let deck_id = deck.id.clone();
                        self.app_state.decks.push(deck);
                        self.selected_deck_id = Some(deck_id);
                        self.new_deck_name.clear();
                        self.new_deck_description.clear();
                        close_new_deck_modal = true;
                        self.save_state_with_message(&format!("Pakli létrehozva: {deck_name}"));
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validációs hiba: {err}"));
                    }
                }
            }
        });
        if close_new_deck_modal {
            new_deck_open = false;
        }
        self.show_new_deck_modal = new_deck_open;

        let mut edit_deck_open = self.show_edit_deck_modal;
        let mut close_edit_deck_modal = false;
        components::modal_frame(ctx, &mut edit_deck_open, "Pakli szerkesztése", |ui| {
            components::labeled_input(ui, "Pakli neve", &mut self.edit_deck_name, "Pakli név");
            components::labeled_input(
                ui,
                "Leírás",
                &mut self.edit_deck_description,
                "Rövid leírás",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Mentés").clicked() {
                let input = DeckUpdateInput {
                    name: self.edit_deck_name.clone(),
                    description: self.edit_deck_description.clone(),
                };

                let conversion: Result<crate::domain::input::DeckUpdate, _> = input.try_into();
                match conversion {
                    Ok(update) => {
                        if let Some(deck_id) = &self.edit_deck_id {
                            if let Some(deck_idx) =
                                self.app_state.decks.iter().position(|deck| &deck.id == deck_id)
                            {
                                self.app_state.decks[deck_idx].name = update.name;
                                self.app_state.decks[deck_idx].description = update.description;
                                close_edit_deck_modal = true;
                                self.save_state_with_message("Pakli frissítve.");
                            }
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validációs hiba: {err}"));
                    }
                }
            }
        });
        if close_edit_deck_modal {
            edit_deck_open = false;
        }
        self.show_edit_deck_modal = edit_deck_open;

        let mut start_session_open = self.show_start_session_modal;
        let mut start_session_requested = false;
        components::modal_frame(
            ctx,
            &mut start_session_open,
            "Tanulási indítás",
            |ui| {
                if let Some(deck_id) = &self.start_session_deck_id {
                    let deck_name = self
                        .app_state
                        .decks
                        .iter()
                        .find(|deck| &deck.id == deck_id)
                        .map(|deck| deck.name.clone())
                        .unwrap_or_else(|| "Ismeretlen pakli".to_string());

                    ui.label(format!("Pakli: {deck_name}"));
                    ui.checkbox(&mut self.start_session_shuffle, "Kártyasorrend keverése");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if components::primary_button(ui, "Indítás").clicked() {
                            start_session_requested = true;
                        }
                        if ui.button("Mégse").clicked() {
                            self.show_start_session_modal = false;
                        }
                    });
                }
            },
        );
        self.show_start_session_modal = start_session_open;

        if start_session_requested {
            if let Some(deck_id) = self.start_session_deck_id.clone() {
                self.start_session(deck_id.as_str(), self.start_session_shuffle);
            }
        }

        let mut delete_deck_open = self.show_delete_deck_confirm;
        let mut should_delete_deck = false;
        components::modal_frame(ctx, &mut delete_deck_open, "Pakli törlése", |ui| {
            ui.label("Biztosan törölni szeretnéd a kijelölt paklit és az összes kártyáját?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if components::variant_button(ui, "Igen, törlés", components::ButtonVariant::Danger)
                    .clicked()
                {
                    should_delete_deck = true;
                }
                if components::variant_button(ui, "Mégse", components::ButtonVariant::Secondary)
                    .clicked()
                {
                    self.show_delete_deck_confirm = false;
                }
            });
        });
        self.show_delete_deck_confirm = delete_deck_open && !should_delete_deck;

        if should_delete_deck {
            if let Some(deck_id) = self.delete_deck_id.clone() {
                if let Some(deck_idx) = self.app_state.decks.iter().position(|deck| deck.id == deck_id)
                {
                    let removed = self.app_state.decks.remove(deck_idx);
                    self.status_message = Some(format!("Pakli torolve: {}", removed.name));
                    self.ensure_selected_deck();
                    self.save_state_with_message("Pakli torolve es allapot mentve.");
                }
            }
            self.delete_deck_id = None;
            self.show_delete_deck_confirm = false;
        }

        let mut new_card_open = self.show_new_card_modal;
        let mut close_new_card_modal = false;
        components::modal_frame(ctx, &mut new_card_open, "Új kártya létrehozása", |ui| {
            components::labeled_input(ui, "Előoldal", &mut self.new_card_front, "Kérdés");
            components::labeled_input(ui, "Hátoldal", &mut self.new_card_back, "Válasz");
            components::labeled_input(
                ui,
                "Tag-ek (vesszővel)",
                &mut self.new_card_tags,
                "rust, ownership",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Kártya hozzáadása").clicked() {
                let input = FlashcardInput {
                    front: self.new_card_front.clone(),
                    back: self.new_card_back.clone(),
                    tags: Self::parse_tags_csv(&self.new_card_tags),
                };

                let conversion: Result<Flashcard, _> = input.try_into();
                match conversion {
                    Ok(card) => {
                        if let Some(deck_idx) = self.selected_deck_index() {
                            self.app_state.decks[deck_idx].cards.push(card);
                            self.new_card_front.clear();
                            self.new_card_back.clear();
                            self.new_card_tags.clear();
                            close_new_card_modal = true;
                            self.save_state_with_message("Kártya létrehozva.");
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validációs hiba: {err}"));
                    }
                }
            }
        });
        if close_new_card_modal {
            new_card_open = false;
        }
        self.show_new_card_modal = new_card_open;

        let mut edit_card_open = self.show_edit_card_modal;
        let mut close_edit_card_modal = false;
        components::modal_frame(ctx, &mut edit_card_open, "Kártya szerkesztése", |ui| {
            components::labeled_input(ui, "Előoldal", &mut self.edit_card_front, "Kérdés");
            components::labeled_input(ui, "Hátoldal", &mut self.edit_card_back, "Válasz");
            components::labeled_input(
                ui,
                "Tag-ek (vesszővel)",
                &mut self.edit_card_tags,
                "rust, ownership",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Mentés").clicked() {
                let input = FlashcardUpdateInput {
                    front: self.edit_card_front.clone(),
                    back: self.edit_card_back.clone(),
                    tags: Self::parse_tags_csv(&self.edit_card_tags),
                };

                let conversion: Result<crate::domain::input::FlashcardUpdate, _> = input.try_into();
                match conversion {
                    Ok(update) => {
                        if let (Some(deck_id), Some(card_id)) =
                            (&self.edit_card_deck_id, &self.edit_card_id)
                        {
                            if let Some(deck_idx) =
                                self.app_state.decks.iter().position(|deck| &deck.id == deck_id)
                            {
                                if let Some(card_idx) = self.app_state.decks[deck_idx]
                                    .cards
                                    .iter()
                                    .position(|card| &card.id == card_id)
                                {
                                    let card = &mut self.app_state.decks[deck_idx].cards[card_idx];
                                    card.front = update.front;
                                    card.back = update.back;
                                    card.tags = update.tags;
                                    card.updated_at = Utc::now();
                                    close_edit_card_modal = true;
                                    self.save_state_with_message("Kártya frissítve.");
                                }
                            }
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validációs hiba: {err}"));
                    }
                }
            }
        });
        if close_edit_card_modal {
            edit_card_open = false;
        }
        self.show_edit_card_modal = edit_card_open;

        let mut delete_card_open = self.show_delete_card_confirm;
        let mut should_delete_card = false;
        components::modal_frame(ctx, &mut delete_card_open, "Kártya törlése", |ui| {
            ui.label("Biztosan törölni szeretnéd a kijelölt kártyát?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if components::variant_button(ui, "Igen, törlés", components::ButtonVariant::Danger)
                    .clicked()
                {
                    should_delete_card = true;
                }
                if components::variant_button(ui, "Mégse", components::ButtonVariant::Secondary)
                    .clicked()
                {
                    self.show_delete_card_confirm = false;
                }
            });
        });
        self.show_delete_card_confirm = delete_card_open && !should_delete_card;

        if should_delete_card {
            if let (Some(deck_id), Some(card_id)) =
                (self.delete_card_deck_id.clone(), self.delete_card_id.clone())
            {
                if let Some(deck_idx) = self.app_state.decks.iter().position(|deck| deck.id == deck_id)
                {
                    if let Some(card_idx) = self.app_state.decks[deck_idx]
                        .cards
                        .iter()
                        .position(|card| card.id == card_id)
                    {
                        self.app_state.decks[deck_idx].cards.remove(card_idx);
                        self.save_state_with_message("Kártya törölve és állapot mentve.");
                    }
                }
            }

            self.delete_card_deck_id = None;
            self.delete_card_id = None;
            self.show_delete_card_confirm = false;
        }

        let mut summary_open = self.show_session_summary_modal;
        let mut close_summary = false;
        components::modal_frame(ctx, &mut summary_open, "Tanulási összegzés", |ui| {
            if let Some(summary) = &self.pending_session_summary {
                components::session_summary_panel(ui, summary);
                ui.add_space(10.0);
                if components::primary_button(ui, "Bezárás").clicked() {
                    close_summary = true;
                }
            }
        });

        if close_summary {
            summary_open = false;
            self.pending_session_summary = None;
        }
        self.show_session_summary_modal = summary_open;
    }

    fn nav_button(ui: &mut egui::Ui, is_active: bool, label: &str) -> egui::Response {
        let fill = if is_active {
            ui.visuals().selection.bg_fill
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };

        let text_color = if is_active {
            egui::Color32::WHITE
        } else if ui.visuals().dark_mode {
            egui::Color32::from_gray(220)
        } else {
            egui::Color32::from_gray(64)
        };

        ui.add_sized(
            [ui.available_width(), 36.0],
            egui::Button::new(egui::RichText::new(label).color(text_color)).fill(fill),
        )
    }
}

impl eframe::App for AppShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let width = ctx.available_rect().width();
        theme::apply_theme(ctx, width, &self.app_state.settings.theme);

        let global_action = ctx.input(|input| {
            if input.modifiers.ctrl && input.key_pressed(egui::Key::S) {
                Some("save")
            } else if input.modifiers.ctrl && input.key_pressed(egui::Key::Num1) {
                Some("route_dashboard")
            } else if input.modifiers.ctrl && input.key_pressed(egui::Key::Num2) {
                Some("route_decks")
            } else if input.modifiers.ctrl && input.key_pressed(egui::Key::Num3) {
                Some("route_study")
            } else if input.modifiers.ctrl && input.key_pressed(egui::Key::Num4) {
                Some("route_settings")
            } else {
                None
            }
        });

        if let Some(action) = global_action {
            match action {
                "save" => self.save_state_with_message("Allapot mentve."),
                "route_dashboard" => self.route = Route::Dashboard,
                "route_decks" => self.route = Route::Decks,
                "route_study" => {
                    if self.app_state.session.is_active() {
                        self.route = Route::Study;
                    }
                }
                "route_settings" => self.route = Route::Settings,
                _ => {}
            }
        }

        if self.route == Route::Study && self.app_state.session.is_active() {
            let keyboard_action = ctx.input(|input| {
                if input.key_pressed(egui::Key::Space) {
                    Some(StudyAction::Flip)
                } else if input.key_pressed(egui::Key::ArrowLeft) {
                    Some(StudyAction::Flip)
                } else if input.key_pressed(egui::Key::ArrowRight) {
                    Some(StudyAction::Next)
                } else if input.key_pressed(egui::Key::Num1) {
                    Some(StudyAction::Grade(crate::domain::Grade::Nehez))
                } else if input.key_pressed(egui::Key::Num2) {
                    Some(StudyAction::Grade(crate::domain::Grade::Kozepes))
                } else if input.key_pressed(egui::Key::Num3) {
                    Some(StudyAction::Grade(crate::domain::Grade::Konnyu))
                } else {
                    None
                }
            });

            if let Some(action) = keyboard_action {
                self.handle_study_action(action);
            }
        }

        egui::SidePanel::left("left_nav")
            .resizable(false)
            .exact_width(240.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Tanulókártya");
                    ui.label("Modern tanulási alkalmazás");
                    ui.label("Ctrl+1..4: nézetváltás");
                    ui.label("Ctrl+S: mentés");
                });
                ui.add_space(16.0);

                ui.vertical_centered(|ui| {
                    let save_response = components::primary_button(ui, "💾");
                    let save_response = save_response.on_hover_text("Állapot mentése");
                    if save_response.clicked() {
                        self.save_state_with_message("Állapot mentve.");
                    }
                });

                ui.add_space(10.0);

                if Self::nav_button(ui, self.route == Route::Dashboard, "Dashboard").clicked() {
                    self.route = Route::Dashboard;
                }
                if Self::nav_button(ui, self.route == Route::Decks, "Paklik").clicked() {
                    self.route = Route::Decks;
                }
                if Self::nav_button(ui, self.route == Route::Study, "Tanulás").clicked() {
                    if self.app_state.session.is_active() {
                        self.route = Route::Study;
                    } else {
                        self.status_message = Some(
                            "Nincs aktív tanulás. Indíts egyet a Paklik nézetből.".to_string(),
                        );
                    }
                }
                if Self::nav_button(ui, self.route == Route::Settings, "Beállítások").clicked() {
                    self.route = Route::Settings;
                }

                if self.loading_until.is_some_and(|deadline| Instant::now() < deadline) {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        let palette = theme::palette(&self.app_state.settings.theme);
                        ui.spinner();
                        ui.label(
                            egui::RichText::new("Betöltés/frissítés...")
                                .size(11.5)
                                .color(palette.warning),
                        );
                    });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(status) = &self.status_message {
                ui.label(status);
                ui.separator();
            }

            match self.route {
                Route::Dashboard => {
                    screens::dashboard_screen(ui, &self.app_state);
                }
                Route::Decks => {
                    self.render_decks_screen(ui);
                }
                Route::Study => {
                    let deck_and_card = self
                        .app_state
                        .session
                        .deck_id
                        .as_ref()
                        .and_then(|deck_id| {
                            self.app_state
                                .decks
                                .iter()
                                .find(|deck| &deck.id == deck_id)
                                .and_then(|deck| {
                                    self.app_state
                                        .session
                                        .current_card_index()
                                        .and_then(|card_idx| {
                                            deck.cards.get(card_idx).map(|card| {
                                                (
                                                    deck.name.clone(),
                                                    card.front.clone(),
                                                    card.back.clone(),
                                                )
                                            })
                                        })
                                })
                        });

                    if let Some((deck_name, card_front, card_back)) = deck_and_card {
                        if let Some(action) = screens::study_screen(
                            ui,
                            &self.app_state,
                            deck_name.as_str(),
                            card_front.as_str(),
                            card_back.as_str(),
                        ) {
                            self.handle_study_action(action);
                        }
                    } else {
                        ui.heading("Tanulási mód");
                        ui.label("Nincs aktív vagy érvényes tanulás.");
                        if components::primary_button(ui, "Vissza a paklikhoz").clicked() {
                            self.route = Route::Decks;
                        }
                    }
                }
                Route::Settings => {
                    if let Some(action) = screens::settings_screen(ui, &self.app_state) {
                        match action {
                            SettingsAction::SetTheme(theme_name) => {
                                self.app_state.settings.theme = theme_name;
                                self.save_state_with_message("Beállítások frissítve.");
                            }
                        }
                    }
                }
            }
        });

        if let Some(until) = self.toast_until {
            if Instant::now() < until {
                let colors = theme::palette(&self.app_state.settings.theme);
                let bg = if self.status_kind == StatusKind::Error {
                    colors.error
                } else {
                    colors.success
                };

                egui::Area::new("status_toast".into())
                    .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 16.0))
                    .order(egui::Order::Foreground)
                    .show(ctx, |ui| {
                        egui::Frame::new()
                            .fill(bg)
                            .corner_radius(8.0)
                            .inner_margin(egui::Margin::same(10))
                            .show(ui, |ui| {
                                if let Some(status) = &self.status_message {
                                    ui.colored_label(egui::Color32::WHITE, status);
                                }
                            });
                    });
            }
        }

        self.render_modals(ctx);
    }
}

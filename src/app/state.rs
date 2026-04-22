use std::sync::Arc;

use chrono::Utc;
use eframe::egui;

use crate::app::routing::Route;
use crate::domain::input::{
    DeckInput, DeckUpdateInput, FlashcardInput, FlashcardUpdateInput,
};
use crate::domain::{AppState, Deck, Flashcard};
use crate::services::JsonStorage;
use crate::ui::{components, screens, theme};

pub struct AppShell {
    pub route: Route,
    pub app_state: AppState,
    storage: Arc<JsonStorage>,
    status_message: Option<String>,
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
}

impl AppShell {
    pub fn new(creation_ctx: &eframe::CreationContext<'_>) -> Self {
        let storage = Arc::new(JsonStorage);
        let mut status_message = None;

        let app_state = match storage.load_app_state() {
            Ok(state) => state,
            Err(err) => {
                status_message = Some(format!("Betoltesi hiba, default allapot indult: {err}"));
                AppState::default()
            }
        };

        // available_rect() itt meg nem hivhato biztonsagosan, mert az egui pass nem fut.
        theme::apply_theme(&creation_ctx.egui_ctx, 1200.0);

        Self {
            route: Route::Dashboard,
            app_state,
            storage,
            status_message,
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
        }
    }

    fn save_state_with_message(&mut self, success_message: &str) {
        match self.storage.save_app_state(&self.app_state) {
            Ok(()) => self.status_message = Some(success_message.to_string()),
            Err(err) => self.status_message = Some(format!("Mentesi hiba: {err}")),
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

    fn render_decks_screen(&mut self, ui: &mut egui::Ui) {
        self.ensure_selected_deck();

        ui.heading("Deckek es kartyak");
        ui.label("Iteracio 1 CRUD: kereses, rendezes, deck/kartya muveletek");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Deck kereso:");
            ui.add(
                egui::TextEdit::singleline(&mut self.deck_search_query)
                    .hint_text("Keress deck nevre")
                    .desired_width(260.0),
            );
            if ui.button("Kereso torlese").clicked() {
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
                ui.strong("Deck lista");
                ui.add_space(6.0);

                if filtered_deck_ids.is_empty() {
                    ui.label("Nincs a keresesre talalat.");
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
                                .unwrap_or_else(|| "Ismeretlen deck".to_string());

                            let selected = self
                                .selected_deck_id
                                .as_ref()
                                .is_some_and(|id| id == deck_id);

                            ui.push_id(deck_id, |ui| {
                                if ui.selectable_label(selected, deck_name).clicked() {
                                    self.selected_deck_id = Some(deck_id.clone());
                                    self.app_state.active_deck = self.selected_deck_id.clone();
                                }
                            });
                        }
                    });
                }

                ui.add_space(10.0);
                if components::primary_button(ui, "Uj deck").clicked() {
                    self.new_deck_name.clear();
                    self.new_deck_description.clear();
                    self.show_new_deck_modal = true;
                }

                let has_selection = self.selected_deck_index().is_some();
                if ui
                    .add_enabled(has_selection, egui::Button::new("Deck szerkesztese"))
                    .clicked()
                {
                    self.open_edit_deck_modal();
                }

                if ui
                    .add_enabled(
                        has_selection,
                        egui::Button::new("Deck torlese (megerosites)"),
                    )
                    .clicked()
                {
                    self.delete_deck_id = self.selected_deck_id.clone();
                    self.show_delete_deck_confirm = true;
                }
            });

            columns[1].group(|ui| {
                ui.strong("Kijelolt deck reszletei");
                ui.add_space(6.0);

                let Some(deck_idx) = self.selected_deck_index() else {
                    ui.label("Nincs kijelolt deck.");
                    return;
                };

                let deck_id = self.app_state.decks[deck_idx].id.clone();
                let deck_name = self.app_state.decks[deck_idx].name.clone();
                let deck_description = self.app_state.decks[deck_idx].description.clone();

                ui.heading(deck_name);
                if deck_description.is_empty() {
                    ui.label("Nincs leiras.");
                } else {
                    ui.label(deck_description);
                }
                ui.add_space(8.0);

                if components::primary_button(ui, "Uj kartya").clicked() {
                    self.new_card_front.clear();
                    self.new_card_back.clear();
                    self.new_card_tags.clear();
                    self.show_new_card_modal = true;
                }

                ui.add_space(8.0);
                ui.label("Kartyak:");

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
                    ui.label("A deckben meg nincsenek kartyak.");
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
                                    ui.label(format!("Hatoldal: {back}"));
                                    ui.label(format!("Tag-ek szama: {tags_count}"));

                                    ui.horizontal(|ui| {
                                        if ui.button("Szerkesztes").clicked() {
                                            edit_card_id = Some(card_id.clone());
                                        }
                                        if ui.button("Torles").clicked() {
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
        components::modal_frame(ctx, &mut new_deck_open, "Uj deck letrehozasa", |ui| {
            components::labeled_input(ui, "Deck neve", &mut self.new_deck_name, "Pl. Rust alapok");
            components::labeled_input(
                ui,
                "Leiras",
                &mut self.new_deck_description,
                "Rovid leiras",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Hozzaadas").clicked() {
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
                        self.save_state_with_message(&format!("Deck letrehozva: {deck_name}"));
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validacios hiba: {err}"));
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
        components::modal_frame(ctx, &mut edit_deck_open, "Deck szerkesztese", |ui| {
            components::labeled_input(ui, "Deck neve", &mut self.edit_deck_name, "Deck nev");
            components::labeled_input(
                ui,
                "Leiras",
                &mut self.edit_deck_description,
                "Rovid leiras",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Mentes").clicked() {
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
                                self.save_state_with_message("Deck frissitve.");
                            }
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validacios hiba: {err}"));
                    }
                }
            }
        });
        if close_edit_deck_modal {
            edit_deck_open = false;
        }
        self.show_edit_deck_modal = edit_deck_open;

        let mut delete_deck_open = self.show_delete_deck_confirm;
        let mut should_delete_deck = false;
        components::modal_frame(ctx, &mut delete_deck_open, "Deck torlese", |ui| {
            ui.label("Biztosan torolni szeretned a kijelolt decket es az osszes kartyajat?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if components::primary_button(ui, "Igen, torles").clicked() {
                    should_delete_deck = true;
                }
                if ui.button("Megse").clicked() {
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
                    self.status_message = Some(format!("Deck torolve: {}", removed.name));
                    self.ensure_selected_deck();
                    self.save_state_with_message("Deck torolve es allapot mentve.");
                }
            }
            self.delete_deck_id = None;
            self.show_delete_deck_confirm = false;
        }

        let mut new_card_open = self.show_new_card_modal;
        let mut close_new_card_modal = false;
        components::modal_frame(ctx, &mut new_card_open, "Uj kartya letrehozasa", |ui| {
            components::labeled_input(ui, "Elooldal", &mut self.new_card_front, "Kerdes");
            components::labeled_input(ui, "Hatoldal", &mut self.new_card_back, "Valasz");
            components::labeled_input(
                ui,
                "Tag-ek (vesszovel)",
                &mut self.new_card_tags,
                "rust, ownership",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Kartya hozzaadasa").clicked() {
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
                            self.save_state_with_message("Kartya letrehozva.");
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validacios hiba: {err}"));
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
        components::modal_frame(ctx, &mut edit_card_open, "Kartya szerkesztese", |ui| {
            components::labeled_input(ui, "Elooldal", &mut self.edit_card_front, "Kerdes");
            components::labeled_input(ui, "Hatoldal", &mut self.edit_card_back, "Valasz");
            components::labeled_input(
                ui,
                "Tag-ek (vesszovel)",
                &mut self.edit_card_tags,
                "rust, ownership",
            );

            ui.add_space(12.0);
            if components::primary_button(ui, "Mentes").clicked() {
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
                                    self.save_state_with_message("Kartya frissitve.");
                                }
                            }
                        }
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Validacios hiba: {err}"));
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
        components::modal_frame(ctx, &mut delete_card_open, "Kartya torlese", |ui| {
            ui.label("Biztosan torolni szeretned a kijelolt kartyat?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if components::primary_button(ui, "Igen, torles").clicked() {
                    should_delete_card = true;
                }
                if ui.button("Megse").clicked() {
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
                        self.save_state_with_message("Kartya torolve es allapot mentve.");
                    }
                }
            }

            self.delete_card_deck_id = None;
            self.delete_card_id = None;
            self.show_delete_card_confirm = false;
        }
    }

    fn nav_button(ui: &mut egui::Ui, is_active: bool, label: &str) -> egui::Response {
        let fill = if is_active {
            ui.visuals().selection.bg_fill
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };

        ui.add_sized(
            [ui.available_width(), 36.0],
            egui::Button::new(label).fill(fill),
        )
    }
}

impl eframe::App for AppShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let width = ctx.available_rect().width();
        theme::apply_theme(ctx, width);

        egui::SidePanel::left("left_nav")
            .resizable(false)
            .exact_width(240.0)
            .show(ctx, |ui| {
                ui.heading("Tanulokartya");
                ui.label("Iteracio 1 CRUD");
                ui.add_space(16.0);

                if Self::nav_button(ui, self.route == Route::Dashboard, "Dashboard").clicked() {
                    self.route = Route::Dashboard;
                }
                if Self::nav_button(ui, self.route == Route::Decks, "Deckek").clicked() {
                    self.route = Route::Decks;
                }
                if Self::nav_button(ui, self.route == Route::Settings, "Beallitasok").clicked() {
                    self.route = Route::Settings;
                }

                ui.add_space(16.0);
                if components::primary_button(ui, "Allapot mentese").clicked() {
                    self.save_state_with_message("Allapot mentve.");
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
                Route::Settings => {
                    screens::settings_screen(ui, &self.app_state);
                }
            }
        });

        self.render_modals(ctx);
    }
}

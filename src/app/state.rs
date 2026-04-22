use std::sync::Arc;

use eframe::egui;

use crate::app::routing::Route;
use crate::domain::input::DeckInput;
use crate::domain::AppState;
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
        }
    }

    fn save_state(&mut self) {
        match self.storage.save_app_state(&self.app_state) {
            Ok(()) => self.status_message = Some("Allapot mentve.".to_string()),
            Err(err) => self.status_message = Some(format!("Mentesi hiba: {err}")),
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
                ui.label("Iteracio 0 alap shell");
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
                    self.save_state();
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
                    screens::decks_screen(ui, &self.app_state);

                    ui.add_space(12.0);
                    if components::primary_button(ui, "Uj deck").clicked() {
                        self.show_new_deck_modal = true;
                    }
                }
                Route::Settings => {
                    screens::settings_screen(ui, &self.app_state);
                }
            }
        });

        let mut modal_open = self.show_new_deck_modal;

        components::modal_frame(
            ctx,
            &mut modal_open,
            "Uj deck letrehozasa",
            |ui| {
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

                    match input.try_into() {
                        Ok(deck) => {
                            self.app_state.decks.push(deck);
                            self.new_deck_name.clear();
                            self.new_deck_description.clear();
                            self.show_new_deck_modal = false;
                            self.save_state();
                        }
                        Err(err) => {
                            self.status_message = Some(format!("Validacios hiba: {err}"));
                        }
                    }
                }
            },
        );

        self.show_new_deck_modal = modal_open;
    }
}

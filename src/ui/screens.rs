use eframe::egui;

use crate::domain::AppState;
use crate::ui::components;

pub fn dashboard_screen(ui: &mut egui::Ui, app_state: &AppState) {
    ui.heading("Dashboard");
    ui.label("Kezdo attekintes");
    ui.add_space(12.0);

    components::card_panel(ui, "Projekt allapot", |ui| {
        ui.label(format!("Deckek szama: {}", app_state.decks.len()));
        if app_state.decks.is_empty() {
            ui.label("Meg nincsenek deckek. Kezdd egy uj deck letrehozasaval.");
        } else {
            ui.label("Van mar mentett deck az allapotban.");
        }
    });
}

pub fn decks_screen(ui: &mut egui::Ui, app_state: &AppState) {
    ui.heading("Deckek");
    ui.label("Alap lista nezet Iteracio 0-hoz.");
    ui.add_space(12.0);

    if app_state.decks.is_empty() {
        components::card_panel(ui, "Ures allapot", |ui| {
            ui.label("Nincs meg deck. Hozz letre egyet az 'Uj deck' gombbal.");
        });
    } else {
        components::card_panel(ui, "Deck lista", |ui| {
            for deck in &app_state.decks {
                ui.label(format!("- {}", deck.name));
            }
        });
    }
}

pub fn settings_screen(ui: &mut egui::Ui, app_state: &AppState) {
    ui.heading("Beallitasok");
    components::card_panel(ui, "Aktiv beallitasok", |ui| {
        ui.label(format!("Tema: {}", app_state.settings.theme));
        ui.label(format!("Nyelv: {}", app_state.settings.language));
    });
}

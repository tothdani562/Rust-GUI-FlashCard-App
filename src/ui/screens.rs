use eframe::egui;

use crate::domain::{AppState, Grade};
use crate::ui::components;

#[derive(Debug, Clone, Copy)]
pub enum StudyAction {
    Flip,
    Grade(Grade),
    Next,
    End,
}

pub fn dashboard_screen(ui: &mut egui::Ui, app_state: &AppState) {
    ui.heading("Dashboard");
    ui.label("Kezdo attekintes");
    ui.add_space(12.0);

    components::card_panel(ui, "Projekt allapot", |ui| {
        ui.label(format!("Deckek szama: {}", app_state.decks.len()));
        ui.label(format!("Lezart sessionok szama: {}", app_state.session_history.len()));
        if app_state.decks.is_empty() {
            ui.label("Meg nincsenek deckek. Kezdd egy uj deck letrehozasaval.");
        } else {
            ui.label("Van mar mentett deck az allapotban.");
        }

        if app_state.session.is_active() {
            let (graded, total) = app_state.session.progress();
            ui.label(format!("Folyamatban levo session: {graded}/{total} ertekelve"));
        }
    });
}

pub fn settings_screen(ui: &mut egui::Ui, app_state: &AppState) {
    ui.heading("Beallitasok");
    components::card_panel(ui, "Aktiv beallitasok", |ui| {
        ui.label(format!("Tema: {}", app_state.settings.theme));
        ui.label(format!("Nyelv: {}", app_state.settings.language));
    });
}

pub fn study_screen(
    ui: &mut egui::Ui,
    app_state: &AppState,
    deck_name: &str,
    card_front: &str,
    card_back: &str,
) -> Option<StudyAction> {
    let mut action = None;

    ui.heading("Tanulasi mod");
    ui.label(format!("Deck: {deck_name}"));
    ui.add_space(8.0);

    let (graded, total) = app_state.session.progress();
    components::session_progress_bar(ui, graded, total);
    ui.add_space(8.0);

    components::card_panel(ui, "Aktualis kartya", |ui| {
        let side_label = if app_state.session.show_back {
            "Hatoldal"
        } else {
            "Elooldal"
        };
        ui.strong(side_label);
        ui.add_space(4.0);
        if app_state.session.show_back {
            ui.label(card_back);
        } else {
            ui.label(card_front);
        }
    });

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        if components::primary_button(ui, "Flip").clicked() {
            action = Some(StudyAction::Flip);
        }
        if ui.button("Kovetkezo kartya").clicked() {
            action = Some(StudyAction::Next);
        }
    });

    ui.add_space(10.0);
    ui.label("Ertekeles:");
    ui.horizontal(|ui| {
        if ui.button("Nehez").clicked() {
            action = Some(StudyAction::Grade(Grade::Nehez));
        }
        if ui.button("Kozepes").clicked() {
            action = Some(StudyAction::Grade(Grade::Kozepes));
        }
        if ui.button("Konnyu").clicked() {
            action = Some(StudyAction::Grade(Grade::Konnyu));
        }
    });

    ui.add_space(10.0);
    if ui.button("Session lezarasa").clicked() {
        action = Some(StudyAction::End);
    }

    if let Some(last_grade) = app_state.session.last_grade {
        ui.add_space(8.0);
        ui.label(format!("Utolso ertekeles: {}", last_grade.label_hu()));
    }

    action
}

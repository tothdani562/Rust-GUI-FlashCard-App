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

#[derive(Debug, Clone)]
pub enum SettingsAction {
    SetTheme(String),
}

pub fn dashboard_screen(ui: &mut egui::Ui, app_state: &AppState) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(22, 14))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Tanulókártya")
                        .text_style(egui::TextStyle::Name("display".into()))
                        .strong(),
                );
                ui.label("Áttekintés és gyors hozzáférés a legfontosabb funkciókhoz.");
            });
            ui.add_space(16.0);

            ui.columns(2, |columns| {
                components::card_panel(&mut columns[0], "Gyors induláshoz", |ui| {
                    ui.label("Hozz létre legalább egy paklit, majd add hozzá a kártyákat.");
                    ui.label("Tanulás közben a Space gombbal tudsz gyorsan fordítani.");
                });

                components::card_panel(&mut columns[1], "Projektállapot", |ui| {
                    ui.label(format!("Paklik száma: {}", app_state.decks.len()));
                    ui.label(format!("Lezárt tanulások száma: {}", app_state.session_history.len()));
                    if app_state.decks.is_empty() {
                        ui.label("Még nincsenek paklik. Kezdd egy új pakli létrehozásával.");
                    } else {
                        ui.label("Van már mentett pakli az állapotban.");
                    }

                    if app_state.session.is_active() {
                        let (graded, total) = app_state.session.progress();
                        ui.label(format!("Folyamatban lévő tanulás: {graded}/{total} értékelve"));
                    }
                });
            });

            ui.add_space(12.0);
            ui.columns(3, |columns| {
                components::card_panel(&mut columns[0], "CRUD", |ui| {
                    ui.label("Pakli- és kártyakezelés gyors szerkesztéssel.");
                });
                components::card_panel(&mut columns[1], "Tanulás", |ui| {
                    ui.label("Flip, értékelés, következő kártya navigáció.");
                });
                components::card_panel(&mut columns[2], "Perzisztencia", |ui| {
                    ui.label("JSON állapot automatikus frissítéssel és kézi mentéssel.");
                });
            });
        });
}

pub fn settings_screen(ui: &mut egui::Ui, app_state: &AppState) -> Option<SettingsAction> {
    let mut action = None;

    ui.heading("Beállítások");
    components::card_panel(ui, "Aktív beállítások", |ui| {
        ui.label(format!("Téma: {}", app_state.settings.theme));
        ui.label(format!("Nyelv: {}", app_state.settings.language));

        ui.add_space(10.0);
        ui.label("Téma választása:");
        ui.horizontal(|ui| {
            let light_selected = app_state.settings.theme.eq_ignore_ascii_case("light");
            let dark_selected = app_state.settings.theme.eq_ignore_ascii_case("dark");

            if ui.selectable_label(light_selected, "Világos").clicked() {
                action = Some(SettingsAction::SetTheme("light".to_string()));
            }
            if ui.selectable_label(dark_selected, "Sötét").clicked() {
                action = Some(SettingsAction::SetTheme("dark".to_string()));
            }
        });
    });

    action
}

pub fn study_screen(
    ui: &mut egui::Ui,
    app_state: &AppState,
    deck_name: &str,
    card_front: &str,
    card_back: &str,
) -> Option<StudyAction> {
    let mut action = None;

    ui.heading("Tanulási mód");
    ui.label(format!("Pakli: {deck_name}"));
    components::card_panel(ui, "Gyorsbillentyűk", |ui| {
        ui.label("Space: flip | Bal/Jobb nyíl: navigáció");
        ui.label("1 = Nehéz, 2 = Közepes, 3 = Könnyű");
    });
    ui.add_space(8.0);

    let (graded, total) = app_state.session.progress();
    components::session_progress_bar(ui, graded, total);
    ui.add_space(8.0);

    components::card_panel(ui, "Aktuális kártya", |ui| {
        let side_label = if app_state.session.show_back {
            "Hátoldal"
        }
        else {
            "Előoldal"
        };

        let flip_anim = ui
            .ctx()
            .animate_bool(ui.make_persistent_id("study_flip"), app_state.session.show_back);
        let active_color = ui.visuals().selection.bg_fill;
        let inactive_color = ui.visuals().widgets.noninteractive.fg_stroke.color;

        ui.strong(side_label);
        ui.add_space(4.0 + (1.0 - flip_anim) * 4.0);
        if app_state.session.show_back {
            ui.label(card_back);
        } else {
            ui.label(card_front);
        }

        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.label("Flip állapot:");
            ui.colored_label(
                if app_state.session.show_back {
                    inactive_color
                } else {
                    active_color
                },
                if app_state.session.show_back {
                    "○ Előoldal"
                } else {
                    "● Előoldal"
                },
            );
            ui.colored_label(
                if app_state.session.show_back {
                    active_color
                } else {
                    inactive_color
                },
                if app_state.session.show_back {
                    "● Hátoldal"
                } else {
                    "○ Hátoldal"
                },
            );
        });
    });

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        if components::primary_button(ui, "Fordítás").clicked() {
            action = Some(StudyAction::Flip);
        }
        if ui.button("Következő kártya").clicked() {
            action = Some(StudyAction::Next);
        }
    });

    ui.add_space(10.0);
    ui.label("Értékelés:");
    ui.horizontal(|ui| {
        if ui.button("Nehéz").clicked() {
            action = Some(StudyAction::Grade(Grade::Nehez));
        }
        if ui.button("Közepes").clicked() {
            action = Some(StudyAction::Grade(Grade::Kozepes));
        }
        if ui.button("Könnyű").clicked() {
            action = Some(StudyAction::Grade(Grade::Konnyu));
        }
    });

    ui.add_space(10.0);
    if ui.button("Tanulás lezárása").clicked() {
        action = Some(StudyAction::End);
    }

    if let Some(last_grade) = app_state.session.last_grade {
        ui.add_space(8.0);
        ui.label(format!("Utolsó értékelés: {}", last_grade.label_hu()));
    }

    action
}

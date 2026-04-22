use eframe::egui;

use crate::domain::SessionSummary;

pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(label)
            .fill(egui::Color32::from_rgb(15, 92, 214))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(8, 61, 145)))
            .corner_radius(8.0),
    )
}

pub fn labeled_input(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.label(label);
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(hint)
            .desired_width(ui.available_width()),
    );
}

pub fn card_panel<R>(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(14))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.strong(title);
            ui.add_space(6.0);
            content(ui)
        })
        .inner
}

pub fn modal_frame(
    ctx: &egui::Context,
    open: &mut bool,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    if *open {
        egui::Window::new(title)
            .open(open)
            .resizable(false)
            .collapsible(false)
            .default_width(420.0)
            .show(ctx, |ui| {
                add_contents(ui);
            });
    }
}

pub fn session_progress_bar(ui: &mut egui::Ui, graded: usize, total: usize) {
    let progress = if total == 0 {
        0.0
    } else {
        graded as f32 / total as f32
    };

    ui.add(
        egui::ProgressBar::new(progress)
            .show_percentage()
            .text(format!("Haladas: {graded}/{total}")),
    );
}

pub fn session_summary_panel(ui: &mut egui::Ui, summary: &SessionSummary) {
    card_panel(ui, "Session osszegzes", |ui| {
        ui.label(format!("Deck azonosito: {}", summary.deck_id));
        ui.label(format!("Osszes kartya: {}", summary.total_cards));
        ui.label(format!("Ertekelt kartya: {}", summary.graded_cards));
        ui.label(format!("Nehez: {}", summary.nehez_count));
        ui.label(format!("Kozepes: {}", summary.kozepes_count));
        ui.label(format!("Konnyu: {}", summary.konnyu_count));
        ui.label(format!("Idotartam: {} mp", summary.duration_seconds));
        ui.label(format!("Shuffle: {}", if summary.shuffle { "igen" } else { "nem" }));
    });
}

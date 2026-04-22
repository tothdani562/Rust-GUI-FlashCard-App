use eframe::egui;

use crate::domain::SessionSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

pub fn primary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    variant_button(ui, label, ButtonVariant::Primary)
}

pub fn variant_button(ui: &mut egui::Ui, label: &str, variant: ButtonVariant) -> egui::Response {
    let visuals = ui.visuals();

    let (fill, stroke_color) = match variant {
        ButtonVariant::Primary => (visuals.selection.bg_fill, visuals.selection.stroke.color),
        ButtonVariant::Secondary => (visuals.widgets.inactive.bg_fill, visuals.window_stroke.color),
        ButtonVariant::Danger => (
            egui::Color32::from_rgb(205, 57, 73),
            egui::Color32::from_rgb(146, 41, 52),
        ),
    };

    let text = match variant {
        ButtonVariant::Primary => egui::RichText::new(label).color(egui::Color32::WHITE),
        _ => egui::RichText::new(label),
    };

    let response = ui.add(
        egui::Button::new(text)
            .fill(fill)
            .stroke(egui::Stroke::new(1.0, stroke_color))
            .corner_radius(8.0),
    );

    response
}

pub fn labeled_input(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    let label_response = ui.label(label);
    let input_response = ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(hint)
            .desired_width(ui.available_width()),
    );
    input_response.labelled_by(label_response.id);
}

pub fn card_panel<R>(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let anim = ui.ctx().animate_bool(ui.make_persistent_id(title), true);
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(14))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.add_space((1.0 - anim) * 8.0);
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
        let anim = ctx.animate_bool(egui::Id::new(("modal", title)), *open);
        egui::Window::new(title)
            .open(open)
            .resizable(false)
            .collapsible(false)
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.add_space((1.0 - anim) * 10.0);
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
    card_panel(ui, "Tanulási összegzés", |ui| {
        ui.label(format!("Pakli azonosító: {}", summary.deck_id));
        ui.label(format!("Összes kártya: {}", summary.total_cards));
        ui.label(format!("Értékelt kártya: {}", summary.graded_cards));
        ui.label(format!("Nehéz: {}", summary.nehez_count));
        ui.label(format!("Közepes: {}", summary.kozepes_count));
        ui.label(format!("Könnyű: {}", summary.konnyu_count));
        ui.label(format!("Időtartam: {} mp", summary.duration_seconds));
        ui.label(format!("Keverés: {}", if summary.shuffle { "igen" } else { "nem" }));
    });
}

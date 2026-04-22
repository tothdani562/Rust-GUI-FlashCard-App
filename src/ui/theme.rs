use eframe::egui;

#[derive(Debug, Clone, Copy)]
pub enum Breakpoint {
    Small,
    Medium,
    Large,
}

pub fn breakpoint(width: f32) -> Breakpoint {
    if width < 1100.0 {
        Breakpoint::Small
    } else if width < 1440.0 {
        Breakpoint::Medium
    } else {
        Breakpoint::Large
    }
}

pub fn apply_theme(ctx: &egui::Context, width: f32) {
    let bp = breakpoint(width);

    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = match bp {
        Breakpoint::Small => egui::vec2(8.0, 8.0),
        Breakpoint::Medium => egui::vec2(10.0, 10.0),
        Breakpoint::Large => egui::vec2(12.0, 12.0),
    };

    style.spacing.button_padding = match bp {
        Breakpoint::Small => egui::vec2(10.0, 8.0),
        Breakpoint::Medium => egui::vec2(14.0, 9.0),
        Breakpoint::Large => egui::vec2(18.0, 10.0),
    };

    style.visuals = egui::Visuals::light();
    style.visuals.window_fill = egui::Color32::from_rgb(248, 249, 252);
    style.visuals.panel_fill = egui::Color32::from_rgb(240, 244, 251);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(232, 238, 248);
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(15, 92, 214);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(20, 104, 232);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(34, 121, 245);
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(31, 41, 55);
    style.visuals.window_corner_radius = egui::epaint::CornerRadius::same(12);

    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(27.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(17.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );

    ctx.set_style(style);
}

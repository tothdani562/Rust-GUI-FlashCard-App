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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub primary: egui::Color32,
    pub neutral_surface: egui::Color32,
    pub neutral_panel: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub error: egui::Color32,
}

pub fn theme_mode(theme: &str) -> ThemeMode {
    if theme.eq_ignore_ascii_case("dark") {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

pub fn palette(theme: &str) -> Palette {
    match theme_mode(theme) {
        ThemeMode::Light => Palette {
            primary: egui::Color32::from_rgb(15, 92, 214),
            neutral_surface: egui::Color32::from_rgb(248, 249, 252),
            neutral_panel: egui::Color32::from_rgb(240, 244, 251),
            success: egui::Color32::from_rgb(32, 136, 85),
            warning: egui::Color32::from_rgb(219, 136, 32),
            error: egui::Color32::from_rgb(205, 57, 73),
        },
        ThemeMode::Dark => Palette {
            primary: egui::Color32::from_rgb(82, 171, 255),
            neutral_surface: egui::Color32::from_rgb(22, 25, 31),
            neutral_panel: egui::Color32::from_rgb(29, 34, 44),
            success: egui::Color32::from_rgb(72, 196, 127),
            warning: egui::Color32::from_rgb(247, 185, 85),
            error: egui::Color32::from_rgb(247, 112, 132),
        },
    }
}

pub fn apply_theme(ctx: &egui::Context, width: f32, theme: &str) {
    let bp = breakpoint(width);
    let palette = palette(theme);
    let mode = theme_mode(theme);

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

    style.visuals = match mode {
        ThemeMode::Light => egui::Visuals::light(),
        ThemeMode::Dark => egui::Visuals::dark(),
    };
    style.visuals.window_fill = palette.neutral_surface;
    style.visuals.panel_fill = palette.neutral_panel;
    style.visuals.extreme_bg_color = match mode {
        ThemeMode::Light => egui::Color32::from_rgb(232, 238, 248),
        ThemeMode::Dark => egui::Color32::from_rgb(36, 42, 54),
    };
    style.visuals.selection.bg_fill = palette.primary;
    style.visuals.widgets.active.bg_fill = palette.primary;
    style.visuals.widgets.hovered.bg_fill = match mode {
        ThemeMode::Light => egui::Color32::from_rgb(34, 121, 245),
        ThemeMode::Dark => egui::Color32::from_rgb(104, 192, 255),
    };
    style.visuals.widgets.noninteractive.fg_stroke.color = match mode {
        ThemeMode::Light => egui::Color32::from_rgb(31, 41, 55),
        ThemeMode::Dark => egui::Color32::from_rgb(221, 229, 239),
    };
    style.visuals.window_corner_radius = egui::epaint::CornerRadius::same(12);

    style.text_styles.insert(
        egui::TextStyle::Name("display".into()),
        egui::FontId::new(34.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(26.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Name("caption".into()),
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
    );

    ctx.set_style(style);
}

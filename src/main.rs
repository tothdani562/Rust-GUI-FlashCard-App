mod app;
mod domain;
mod macros;
mod services;
mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 760.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Tanulokartya - Iteracio 1"),
        ..Default::default()
    };

    eframe::run_native(
        "Tanulokartya",
        native_options,
        Box::new(|creation_ctx| Ok(Box::new(app::AppShell::new(creation_ctx)))),
    )
}

use std::fmt::format;

use eframe::{epi::App, run_native, egui::{CentralPanel, ScrollArea}, NativeOptions};
mod gwip;

use gwip::{Gwip, WeatherInfoCard};

impl App for Gwip {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, | ui | {
            ScrollArea::auto_sized().show(ui, | ui | {
                for dumy in &self.weather_info {
                    ui.label(&dumy.title);
                    ui.label(&dumy.url);
                    ui.label(&dumy.description);
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Get Weather Information"
    }
}

fn main() {
    let app: Gwip = Gwip::new();
    let window_options: NativeOptions = NativeOptions::default();

    run_native(Box::new(app), window_options);
}

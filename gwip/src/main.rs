use eframe::{epi::App, run_native, egui::CentralPanel, NativeOptions};

struct Gwip;

impl App for Gwip {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, | ui | {
            ui.label("Article");
        });
    }

    fn name(&self) -> &str {
        "Get Weather Information"
    }
}

fn main() {
    let app: Gwip = Gwip;
    let window_options: NativeOptions = NativeOptions::default();

    run_native(Box::new(app), window_options);
}

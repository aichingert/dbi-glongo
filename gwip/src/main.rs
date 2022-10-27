use eframe::{epi::App, run_native, egui::{CentralPanel, ScrollArea, Label}, NativeOptions};
use reqwest;
use tokio;

mod gwip;

use gwip::Gwip;

impl App for Gwip {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, | ui | {
            ScrollArea::auto_sized().show(ui, | ui | {
                for card in &self.weather_info {
                    ui.add(Label::new(&card.title).heading());
                    ui.label(&card.description);
                    ui.label(format!("Date: {}", &card.date));
                    ui.add_space(5.0);
               }
            })
        });
    }

    fn name(&self) -> &str {
        "Get Weather Information"
    }

    fn clear_color(&self) -> eframe::egui::Rgba {
        eframe::egui::Rgba::from_rgb(0.0, 0.0, 255.0).into()
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://localhost:8080/weatherlist/entries").await?;
    let body = res.text().await?;   

    let app: Gwip = Gwip::new(&body);
    let window_options: NativeOptions = NativeOptions::default();

    run_native(Box::new(app), window_options);

    Ok(())
}
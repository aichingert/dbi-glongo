use eframe::{epi::App, run_native, egui::{CentralPanel, ScrollArea}, NativeOptions};
use reqwest;
use tokio;

mod gwip;

use gwip::{Gwip, WeatherInfoCard};

impl App for Gwip {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, | ui | {
            ScrollArea::auto_sized().show(ui, | ui | {
                for dumy in &self.weather_info {
                    ui.label(&dumy.title);
                    ui.label(&dumy.description);
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Get Weather Information"
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://localhost:8080/weatherlist/entries").await?;
    println!("{:?}", &res);
    let body = res.text().await?;   

    println!("{body}");

    let app: Gwip = Gwip::new(&body);
    let window_options: NativeOptions = NativeOptions::default();

    run_native(Box::new(app), window_options);

    Ok(())
}
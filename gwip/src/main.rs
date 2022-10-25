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


// fn main() -> Result<()> {
//     // let app: Gwip = Gwip::new();
//     // let window_options: NativeOptions = NativeOptions::default();

//     // run_native(Box::new(app), window_options);

//     let result = reqwest::blocking::get("127.0.0.0:8080")?;

//     println!("{:?}", result);
    
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let app: Gwip = Gwip::new();
    let window_options: NativeOptions = NativeOptions::default();

    run_native(Box::new(app), window_options);

    let res = reqwest::get("").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
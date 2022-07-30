use std::fmt::format;

use eframe::{epi::App, run_native, egui::{CentralPanel, ScrollArea}, NativeOptions};

pub struct Gwip {
    pub weather_info: Vec<WeatherInfoCard>
}

pub struct WeatherInfoCard {
    pub title: String,
    pub description: String,
    pub url: String,
}

impl Gwip {
    pub fn new() -> Self {
        let iter = (0..20).map( | dumy | WeatherInfoCard {
            title: format!("title{}", dumy),
            description: format!("desc{}", dumy),
            url: format!("https://examle.com/{}", dumy)
        });

        Gwip {
            weather_info: Vec::from_iter(iter)
        } 
    }
}
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: String,
    pub weather_state: String
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub weather_state: String
}
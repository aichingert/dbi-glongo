use serde_json::Value;

pub struct Gwip {
    pub weather_info: Vec<WeatherInfoCard>
}

pub struct WeatherInfoCard {
    pub title: String,
    pub description: String,
    pub date: i64
}

impl Gwip {
    pub fn new(content: &String) -> Self {
        let json: Value = serde_json::from_str(&content)
            .unwrap_or(Value::String("".to_string()));

        
        Gwip {
            weather_info: vec![]
        }
    }
}
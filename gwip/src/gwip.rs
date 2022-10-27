use serde_json::Value;

pub struct Gwip {
    pub weather_info: Vec<WeatherInfoCard>
}

pub struct WeatherInfoCard {
    pub title: String,
    pub description: String,
    pub date: String
}

impl Gwip {
    pub fn new(content: &String) -> Self {
        // Parsing the json values we get from the weather api
        let json: Value = serde_json::from_str(&content)
            .unwrap_or(Value::String("".to_string()));

        // Creating the weather info cards so we can use that when we create Gwip
        let mut weather_cards: Vec<WeatherInfoCard> = Vec::new();

        // Taking the json as array because there are multiple weather infos
        let values = json.as_array();
        
        // Checking if there are some values
        if let Some(objects) = values {
            // Iterating over all objects and adding them to the list of weather cards
            for obj in objects {
                let card = obj.as_object().unwrap();

                // Adding the card to the cards
                weather_cards.push(WeatherInfoCard {
                    title: card["title"].as_str().unwrap().to_string(),
                    description: card["weather_state"].as_str().unwrap().to_string(),
                    date: card["date"].as_str().unwrap().to_string(), 
                });
            }
        }
        
        Gwip {
            weather_info: weather_cards
        }
    }
}
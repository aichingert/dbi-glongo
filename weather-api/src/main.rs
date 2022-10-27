use  actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex; 

mod weatherlist;
use weatherlist::services;


struct AppState {
    weatherlist_entries: Mutex<Vec<WeatherListEntry>>   
}

#[derive(Serialize, Deserialize, Clone)]
struct WeatherListEntry {
    id: i32,
    title: String,
    weather_state: String,
    date: String
}

#[get("/")]
async fn index() -> String {
    "This is health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        weatherlist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
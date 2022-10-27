use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, WeatherListEntry};
use super::models::{CreateEntryData, UpdateEntryData};

#[get("/weatherlist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.weatherlist_entries.lock().unwrap().to_vec())
}

#[post("weatherlist/entries")]
async fn create_entry(data: web::Data<AppState>, extractor: web::Json<CreateEntryData>) -> impl Responder {
    let mut weatherlist_entries = data.weatherlist_entries.lock().unwrap();
    let mut max_id: i32 = 0;

    for i in 0..weatherlist_entries.len() {
        if weatherlist_entries[i].id > max_id {
            max_id = weatherlist_entries[i].id;
        }
    }

    weatherlist_entries.push(WeatherListEntry {
        id: max_id + 1,
        title: extractor.title.clone(),
        date: extractor.date.clone(),
        weather_state: extractor.weather_state.clone()
    });

    HttpResponse::Ok().json(weatherlist_entries.to_vec())
}

#[put("/weatherlist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, exractor: web::Json<UpdateEntryData>) -> impl Responder {
    let id = path.into_inner();
    let mut weatherlist_entries = data.weatherlist_entries.lock().unwrap();

    for i in 0..weatherlist_entries.len() {
        if weatherlist_entries[i].id == id {
            weatherlist_entries[i].weather_state = exractor.weather_state.clone();
            break;
        }
    }

    HttpResponse::Ok().json(weatherlist_entries.to_vec())
}

#[delete("weatherlist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut weatherlist_entries = data.weatherlist_entries.lock().unwrap();
    let id = path.into_inner();

    *weatherlist_entries = weatherlist_entries.to_vec().into_iter().filter(|searching| searching.id != id).collect();

    HttpResponse::Ok().json(weatherlist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
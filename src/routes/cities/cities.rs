use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct City {
    pub city_id: i32,
    pub city: String,
    pub country_id: i16,
    pub last_update: chrono::NaiveDateTime,
}

#[get("/city")]
pub async fn get_cities(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, City>("SELECT * FROM city")
        .fetch_all(&state.db)
        .await
    {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().body("Cities not found")
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities);
}

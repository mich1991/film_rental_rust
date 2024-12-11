use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct City {
    pub city_id: i32,
    pub city: String,
    pub country_id: i16,
    pub last_update: chrono::NaiveDateTime,
}

#[get("")]
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

#[get("/{country_id}")]
pub async fn get_cities_by_country(
    state: web::Data<AppState>,
    path: web::Path<i16>,
) -> impl Responder {
    let country_id = path.into_inner();
    match sqlx::query_as::<_, City>(
        "SELECT * FROM city WHERE country_id = $1",
    )
    .bind(country_id)
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

#[post("")]
pub async fn post_city(
    state: web::Data<AppState>,
    city: web::Json<City>,
) -> impl Responder {
    match sqlx::query(
        "INSERT INTO city (city, country_id, last_update) VALUES ($1, $2, $3)",
    )
    .bind(&city.city)
    .bind(&city.country_id)
    .bind(&city.last_update)
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().body("City added successfully"),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().body("City not added")
        }
    }
}


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities);
    cfg.service(get_cities_by_country);
}

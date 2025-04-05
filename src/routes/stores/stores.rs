use crate::AppState;
use crate::models::GenericResponse;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct StoresPerCountry {
    country: String,
    count: i64
}

#[get("/stores_per_country")]
pub async fn get_all_stores_per_country(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, StoresPerCountry>("
    SELECT count(*) as count, ct.country
    FROM store st
    JOIN address ad on st.address_id = ad.address_id
    JOIN city ci on ad.city_id = ci.city_id
    JOIN country ct on ct.country_id = ci.country_id
    GROUP BY ct.country_id, ct.country
    ")
        .fetch_all(&state.db)
        .await
    {
        Ok(movies) => HttpResponse::Ok().json(
            GenericResponse::success(movies, "Returned store per country")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().json(GenericResponse::error((), "Didn't find any stores"))
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all_stores_per_country);
}
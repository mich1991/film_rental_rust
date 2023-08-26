use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, JsonValue};
use sqlx::{self, Error, FromRow};

#[derive(FromRow, Deserialize, Serialize)]
pub struct TotalMoviesPerCategory {
    category_name: String,
    count: i64,
}

#[get("/movies/total_by_category")]
pub async fn get_total_movies_per_category(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, TotalMoviesPerCategory>("\
    SELECT t1.name as category_name, count(*)
    FROM category t1
    JOIN film_category t2
        ON t1.category_id = t2.category_id
    GROUP BY category_name;
    ")
        .fetch_all(&state.db)
        .await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().body("Movies not found")
        }
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct TopMovies {
    title: String,
    count: i64
}
#[get("/movies/top_3_rented")]
pub async fn top_3_rented(state: web::Data<AppState>) -> impl Responder {
    //
    match sqlx::query_as::<_, TopMovies>("
    SELECT t3.title,  count(*)
    FROM rental t1
    JOIN inventory t2
        ON t1.inventory_id = t2.inventory_id
    JOIN film t3
        ON t2.film_id = t3.film_id
    GROUP BY t3.title
    ORDER BY count(*) DESC
    LIMIT 3
    ")
        .fetch_all(&state.db)
        .await
    {
        Ok(top) => HttpResponse::Ok().json(top),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().body("Movies not found")
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_total_movies_per_category)
        .service(top_3_rented);
}
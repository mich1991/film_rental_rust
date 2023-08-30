use crate::AppState;
use crate::models::GenericResponse;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use rust_decimal;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Movies {
    film_id: i32,
    title: String,
    description: String,
    release_year: i32,
    language_id: i16,
    replacement_cost: rust_decimal::Decimal,
    rating: String,
}

#[get("/movies")]
pub async fn get_all_movies(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Movies>("
    SELECT * FROM film
    ")
        .fetch_all(&state.db)
        .await
    {
        Ok(movies) => HttpResponse::Ok().json(
            GenericResponse::success(movies, "Returned all movies")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().json(GenericResponse::error((), "Didn't find any movies"))
        }
    }
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct TotalMoviesPerCategory {
    category_name: String,
    count: i64,
}

#[get("/movies/total_by_category")]
pub async fn get_total_movies_per_category(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, TotalMoviesPerCategory>("\
    SELECT t1.name as category_name, count(*) as count
    FROM category t1
    JOIN film_category t2
        ON t1.category_id = t2.category_id
    GROUP BY category_name
    ORDER BY count DESC;
    ")
        .fetch_all(&state.db)
        .await {
        Ok(movies) => HttpResponse::Ok()
            .json(GenericResponse::success(movies, "Returned total movies per category")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().json(GenericResponse::error((), "Movies not found"))
        }
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct TopMovies {
    title: String,
    count: i64,
}

#[get("/movies/top_3_rented")]
pub async fn top_3_rented(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, TopMovies>("
    SELECT t3.title, count(*)
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
        Ok(top) => HttpResponse::Ok().json(GenericResponse::success(top, "Returned top 3 rented movies")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().json(GenericResponse::error((), "Didn't find any movies"))
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all_movies)
        .service(get_total_movies_per_category)
        .service(top_3_rented);
}
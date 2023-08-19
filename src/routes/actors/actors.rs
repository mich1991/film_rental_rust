use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, JsonValue};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Actor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: chrono::NaiveDateTime,
}

#[get("/actors")]
pub async fn get_actors(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Actor>("SELECT * FROM actor")
        .fetch_all(&state.db)
        .await
    {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().json("Users not found")
        }
    }
}

#[get("/actor/{id}")]
pub async fn get_actor(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Actor>("SELECT * FROM actor WHERE actor_id = $1 ")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().json("Users not found")
        }
    }
}

#[derive(Deserialize)]
struct ActorQuery {
    pub first_name: String,
    pub last_name: String,
}

#[get("/actor-query")]
pub async fn get_actor_query(
    state: web::Data<AppState>,
    query: web::Query<ActorQuery>,
) -> impl Responder {
    match sqlx::query_as::<_, Actor>(
        "\
    SELECT * FROM actor \
    WHERE first_name = $1\
    AND last_name = $2\
    ",
    )
    .bind(&query.first_name)
    .bind(&query.last_name)
    .fetch_one(&state.db)
    .await
    {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().json("User not found")
        }
    }
}

#[derive(FromRow, Deserialize, Serialize)]
struct ActorFilmsByCategory {
    first_name: String,
    last_name: String,
    titles: JsonValue,
}

#[get("actor-film-in-category/{actor_id}/{category_id}")]
pub async fn get_actor_films_by_category(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (actor_id, category_id) = path.into_inner();
    match sqlx::query_as::<_, ActorFilmsByCategory>(
        "\
    SELECT * FROM get_actor_film_in_category($1, $2)
    ",
    )
    .bind(actor_id)
    .bind(category_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound().body("Not found")
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_actor_query)
        .service(get_actors)
        .service(get_actor)
        .service(get_actor_films_by_category);
}

use crate::AppState;
use crate::models::GenericResponse;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
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

#[get("")]
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

#[derive(FromRow, Serialize, Deserialize)]
pub struct ActorForm {
   pub first_name: String,
   pub last_name: String,
}

#[post("")]
pub async fn post_actor(state: web::Data<AppState>, form: web::Json<ActorForm>) -> impl Responder {
    match sqlx::query_as::<_, Actor>("\
    INSERT INTO actor (first_name, last_name) \
    VALUES ($1,$2)\
    RETURNING *")
        .bind(&form.first_name).bind(&form.last_name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(actors) => HttpResponse::Ok().json(GenericResponse::success(actors, "Successfully added new actor")),
        Err(e) => {
            println!("{}", e);
            HttpResponse::BadRequest().json("Some fields are missing.")
        }
    }
}

#[get("/{id}")]
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

#[put("/{id}")]
pub async fn update_actor(state: web::Data<AppState>, path: web::Path<i64>, form: web::Json<ActorForm>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Actor>("\
    UPDATE actor \
    SET \
    first_name = $1, \
    last_name = $2 \
    WHERE actor_id = $3
    RETURNING *")
        .bind(&form.first_name).bind(&form.last_name)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(actors) => HttpResponse::Ok().json(GenericResponse::success(actors, "updated actor successfully")),
        Err(e) => {
            println!("{}", e);
            HttpResponse::BadRequest().json("Some fields are missing.")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_actor(state: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let query = sqlx::query("DELETE FROM actor WHERE actor_id = $1")
        .bind(&id.clone())
        .execute(&state.db)
        .await;
    match query {
        Ok(_) => HttpResponse::Ok().json(GenericResponse::success((), "success: ".to_owned() + id.to_string().as_str())),
        Err(e) => {
            println!("{e}");
            HttpResponse::BadRequest().json(GenericResponse::error((), "error while deleting user: ".to_owned() + id.to_string().as_str()))
        }
    }
}


#[derive(Deserialize)]
pub struct ActorQuery {
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
        .service(post_actor)
        .service(update_actor)
        .service(delete_actor)
        .service(get_actor_films_by_category);
}

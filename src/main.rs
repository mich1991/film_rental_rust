mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Mutex;
use actix_cors::Cors;

pub struct AppState {
    pub counter: Mutex<i32>,
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to DB");

    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
        db: pool.clone(),
    });

    HttpServer::new(move || {
        let counter = web::scope("/counter").configure(routes::counter_routes);
        let api = web::scope("/api").configure(routes::api_routes);
        let cors = Cors::permissive();

        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:4200/")
        //     .allowed_origin_fn(|origin, _req_head| {
        //         origin.as_bytes().ends_with(b".localhost:4200")
        //     })
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(counter)
            .service(api)
        // .route("/hey", web::get().to(manual_hello()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

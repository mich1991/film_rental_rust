mod routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Mutex;

pub struct AppState {
    app_name: String,
    counter: Mutex<i32>,
    db: Pool<Postgres>,
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
        app_name: String::from("Actix web"),
        counter: Mutex::new(0),
        db: pool.clone(),
    });

    HttpServer::new(move || {
        let counter = web::scope("/counter").configure(routes::counter_routes);
        let api = web::scope("/api").configure(routes::api_routes);

        // let api = web::scope("/api")
        //     .configure(routes::actors::routes)
        //     .configure(routes::cities::routes);

        App::new()
            .app_data(app_state.clone())
            .service(counter)
            .service(api)
        // .route("/hey", web::get().to(manual_hello()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

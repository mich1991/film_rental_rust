use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CounterInfo {
    amount: i32,
    multiplier: i32,
}

#[get("/add")]
async fn add_counter(state: web::Data<AppState>) -> impl Responder {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/add-query")]
async fn add_counter_amount_query(
    state: web::Data<AppState>,
    path: web::Query<CounterInfo>,
) -> impl Responder {
    let path = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    *counter += path.amount * path.multiplier;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/add/{amount}")]
async fn add_counter_amount(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let path = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    let amount = path;
    *counter += amount;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/add/{amount}/{multiplier}")]
async fn add_counter_amount_multi(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    // let path = path.into_inner();
    let (amount, multiplier) = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    *counter += amount * multiplier;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/minus")]
pub async fn minus_counter(state: web::Data<AppState>) -> impl Responder {
    let mut counter = state.counter.lock().unwrap();
    *counter -= 1;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/minus-query")]
async fn minus_counter_amount_query(
    state: web::Data<AppState>,
    path: web::Query<CounterInfo>,
) -> impl Responder {
    let path = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    *counter -= path.amount * path.multiplier;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/minus/{amount}")]
pub async fn minus_counter_amount(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let path = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    let amount = path;
    *counter -= amount;
    HttpResponse::Ok().body(counter.to_string())
}

#[get("/minus/{amount}/{multiplier}")]
async fn minus_counter_amount_multi(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    // let path = path.into_inner();
    let (amount, multiplier) = path.into_inner();
    let mut counter = state.counter.lock().unwrap();
    *counter += amount * multiplier;
    HttpResponse::Ok().body(counter.to_string())
}

pub fn counter_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_counter)
        .service(add_counter_amount)
        .service(add_counter_amount_multi)
        .service(add_counter_amount_query)
        .service(minus_counter)
        .service(minus_counter_amount)
        .service(minus_counter_amount_multi)
        .service(minus_counter_amount_query);
}

use crate::AppState;
use crate::models::GenericResponse;

use actix_web::{get, web, HttpResponse, Responder};
use chrono;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Deserialize,Serialize, FromRow)]
pub struct TotalCustomersPerShop {
    count: Option<i64>,
    address: String,
}

#[get("/total_per_shop")]
pub async fn get_total_customers_per_shop(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(TotalCustomersPerShop, "
    SELECT count(*) as count, t3.address
    FROM customer t1
    JOIN store t2
        ON t2.store_id = t1.store_id
    JOIN address t3
        ON t2.address_id = t3.address_id
    GROUP BY t1.store_id, t3.address
    ORDER BY count DESC;
    ").fetch_all(&state.db).await
    {
        Ok(customers) => HttpResponse::Ok()
            .json(GenericResponse::success(customers, "Returned customers per shop")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound()
                .json(GenericResponse::error((), "Didn't find any customers"))
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CustomersInShop {
    first_name: String,
    last_name: String,
    email: Option<String>,
    activebool: bool,
    create_date: chrono::NaiveDate,
    last_update: Option<chrono::NaiveDateTime>,
}

#[get("/shop/{shop_id}")]
pub async fn get_customers_from_shop(state: web::Data<AppState>, path: web::Path<i16>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(CustomersInShop, "
    SELECT first_name, last_name, email, activebool, create_date, last_update
    FROM customer
    WHERE store_id = $1", id)
        .fetch_all(&state.db)
        .await {
        Ok(customers) => HttpResponse::Ok()
            .json(GenericResponse::success(customers, "Returned customers for a single shop")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound()
                .json(GenericResponse::error((), "Didn't find any customers"))
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CustomerDetails {
    first_name: String,
    last_name: String,
    email: Option<String>,
    activebool: bool,
    create_date: chrono::NaiveDate,
    last_update: Option<chrono::NaiveDateTime>,
    address: String,
    district: String,
    phone: String,
    postal_code: Option<String>,
    city: String,
}

#[get("/{customer_id}")]
pub async fn get_customer_details(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let customer_id = path.into_inner();
    match sqlx::query_as!(CustomerDetails, "\
    SELECT t1.first_name, t1.last_name, t1.email, t1.activebool, t1.create_date, t1.last_update,
       t2.address, t2.district, t2.phone, t2.postal_code,
       t3.city
    FROM customer t1
    JOIN address t2
        ON t1.address_id = t2.address_id
    JOIN city t3
        ON t2.city_id = t3.city_id
    WHERE customer_id = $1", customer_id)
        .fetch_one(&state.db)
        .await {
        Ok(customer) => HttpResponse::Ok().json(GenericResponse::success(customer, "Returned customer details")),
        Err(e) => {
            println!("{e}");
            HttpResponse::NotFound()
                .json(GenericResponse::error((), "Customer not found"))
        }
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_total_customers_per_shop)
        .service(get_customer_details)
        .service(get_customers_from_shop);

}
use crate::AppState;
use crate::models::GenericResponse;

use actix_web::{get, web, HttpResponse, Responder, post};
use chrono;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Deserialize, Serialize, FromRow)]
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

#[derive(Deserialize, Serialize)]
pub struct CreateCustomerForm {
    store_id: i16,
    first_name: String,
    last_name: String,
    email: Option<String>,
    activebool: bool,
    address: CreateAddress,
}

#[derive(Deserialize, Serialize)]
pub struct CreateCustomer {
    customer_id: Option<i32>,
    store_id: i16,
    first_name: String,
    last_name: String,
    email: Option<String>,
    activebool: bool,
    active: Option<i32>,
    address_id: i16,
}

#[derive(Deserialize, Serialize)]
pub struct CreateAddress {
    address: String,
    address2: Option<String>,
    district: String,
    postal_code: Option<String>,
    phone: String,
    city: String,
    country: String,
}

pub struct ValueExists {
    pub exists: Option<bool>,
}

pub struct CountryRespond {
    pub country_id: i32,
}

pub struct CityRespond {
    pub city_id: i32,
}

pub struct AddressRespond {
    pub address_id: i32,
}

#[post("")]
pub async fn create_customer(state: web::Data<AppState>, data: web::Json<CreateCustomerForm>) -> impl Responder {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = state.db.begin().await.expect("failed to start transaction");
    let country_exists = sqlx::query_as!(
        ValueExists,
        "SELECT exists(select * from country where country.country = $1)",
        &data.address.country.clone())
        .fetch_one(&mut *tx).await.expect("Error while checking country");

    let country_respond: CountryRespond;

    if country_exists.exists == Some(true) {
        country_respond = sqlx::query_as!(
            CountryRespond,
            "SELECT t1.country_id FROM country t1 WHERE t1.country = $1",
            data.address.country.clone()
        ).fetch_one(&mut *tx).await.expect("Error while Inserting");
    } else {
        country_respond = sqlx::query_as!(
            CountryRespond,
            "INSERT INTO country (country)\
            VALUES ($1)\
            RETURNING country_id",
            data.address.country.clone()
        ).fetch_one(&mut *tx).await.expect("Error while Inserting");
    }

    let city_exists:ValueExists = sqlx::query_as!(
        ValueExists,
        "SELECT exists(select * from city where city.city = $1 and city.country_id = $2)",
        &data.address.city.clone(),
        country_respond.country_id.clone() as i16
        ).fetch_one(&mut *tx).await.expect("Error while checking existing city");

    let city_respond:CityRespond;

    if city_exists.exists == Some(true) {
        city_respond = sqlx::query_as!(
            CityRespond,
            "SELECT city_id FROM city WHERE city.city = $1 and city.country_id = $2",
            &data.address.city.clone(),
            country_respond.country_id.clone() as i16
        ).fetch_one(&mut *tx).await.expect("Error while Inserting");
    } else {
        city_respond = sqlx::query_as!(
            CityRespond,
            "INSERT INTO city (city, country_id)\
            VALUES ($1, $2)\
            RETURNING city_id",
            data.address.city.clone(),
            country_respond.country_id.clone() as i16
        ).fetch_one(&mut *tx).await.expect("Error while Inserting");
    }

    let address_respond = sqlx::query_as!(
        AddressRespond,
        "INSERT \
        INTO address (address, address2, district, city_id, postal_code, phone)\
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING address_id",
        data.address.address.clone(),
        data.address.address2.clone(),
        data.address.district.clone(),
        city_respond.city_id.clone() as i16,
        data.address.postal_code.clone(),
        data.address.phone.clone()
    )
        .fetch_one(&mut *tx).await.expect("Error while inserting new address");

    let customer = sqlx::query!("INSERT INTO customer \
    (store_id, first_name, last_name, email, address_id, activebool) \
    VALUES ($1, $2, $3, $4, $5, $6)\
    RETURNING *
    ;",
        &data.store_id.to_owned(),
        &data.first_name,
        &data.last_name,
        data.email,
        address_respond.address_id as i16,
        &data.activebool

    )
        .fetch_one(&mut *tx).await.expect("Error while Inserting");

    tx.commit().await.expect("Transaction got rollback due to the internal error");

    let respond = CreateCustomer {
        customer_id: Some(customer.customer_id),
        store_id: customer.store_id,
        first_name: customer.first_name,
        last_name: customer.last_name,
        email: customer.email,
        address_id: customer.address_id,
        activebool: customer.activebool,
        active: customer.active,
    };


    HttpResponse::Ok().json(GenericResponse::success(respond, "Successfully created customer"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_total_customers_per_shop)
        .service(get_customer_details)
        .service(create_customer)
        .service(get_customers_from_shop);
}
use actix_web::web;

pub mod actors;
pub mod cities;
pub mod counter;
pub mod movies;
pub mod customers;

pub use counter::counter_routes;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .configure(actors::routes)
        .configure(cities::routes)
        .configure(customers::routes)
        .configure(movies::routes);
}

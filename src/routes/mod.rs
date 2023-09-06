use actix_web::web;

pub mod actors;
pub mod cities;
pub mod counter;
pub mod movies;
pub mod customers;

pub use counter::counter_routes;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("actors").configure(actors::routes))
        .service(web::scope("cities").configure(cities::routes))
        .service(web::scope("customers").configure(customers::routes))
        .service(web::scope("movies").configure(movies::routes));
}

mod graphql_routes;

use actix_web::web;
use graphql_routes::{graphql, playground};

pub fn register(config: &mut web::ServiceConfig) {
    config.service(playground).service(graphql);
}

mod graphql_routes;
mod status;

use self::{
    graphql_routes::{graphql, playground},
    status::get_status,
};
use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    config
        .service(playground)
        .service(graphql)
        .service(get_status);
}

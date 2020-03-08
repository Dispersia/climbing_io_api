mod v1;

use actix_web::web;

pub fn register(config: &mut web::ServiceConfig) {
    v1::register(config);
}

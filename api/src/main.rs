mod configuration;
mod routes;

use actix_web::{middleware, App, HttpServer};
use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let container = Arc::new(configuration::create_container());
    let schema = Arc::new(graphql::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(container.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::register)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

mod routes;

use actix_web::{middleware, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let schema = std::sync::Arc::new(graphql::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

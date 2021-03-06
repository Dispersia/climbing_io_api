use std::sync::Arc;

use ::graphql::{Context, Schema};
use actix_web::{get, post, web, Error, HttpResponse};
use coi::Container;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

#[post("v1/graphql")]
pub async fn graphql(
    container: web::Data<Arc<Container>>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context::new(container.get_ref().clone());

    let res = data.execute(&schema, &ctx).await;
    let response = Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}

#[get("v1/playground")]
pub async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/v1/graphql", None))
}

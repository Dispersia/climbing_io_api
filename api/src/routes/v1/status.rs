use actix_web::{get, HttpResponse};

#[get("api/v1/status")]
pub fn get_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": true}"#)
}

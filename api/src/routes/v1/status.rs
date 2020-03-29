use actix_web::{get, HttpResponse};

#[get("v1/status")]
pub fn get_status() -> HttpResponse {
    HttpResponse::Ok().finish()
}

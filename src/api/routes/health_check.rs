use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the Voyage Atlas API!"
    }))
}

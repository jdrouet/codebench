use actix_web::{get, HttpResponse};

#[get("/api/")]
pub async fn handler() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

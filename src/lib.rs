pub mod lingua;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input { teacs: String }

#[get("/health")]
pub async fn health() -> impl Responder {
    match lingua::ga::gaelspell::check_word("GaelSpell") {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[post("/api/gaelspell/1.0")]
pub async fn api(payload: web::Json<Input>) -> impl Responder {
    match lingua::ga::gaelspell::spellcheck(&payload.teacs) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health).service(api);
}



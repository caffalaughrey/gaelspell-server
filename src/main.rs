use actix_web::{web, App, HttpServer};
use gaelspell_server::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting GaelSpell Rust server on 0.0.0.0:5000…");
    HttpServer::new(|| App::new().configure(configure))
        .workers(1)
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}

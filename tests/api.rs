use actix_web::{test, App};
use gaelspell_server::configure;

#[actix_web::test]
async fn api_endpoint_basic() {
    std::env::set_var("GAELSPELL_DISABLE_PERL", "1");
    let app = test::init_service(App::new().configure(configure)).await;
    let req = test::TestRequest::post()
        .uri("/api/gaelspell/1.0")
        .set_json(&serde_json::json!({"teacs": "Ba mhath liom abcdefxyz"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

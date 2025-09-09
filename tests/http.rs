use actix_web::{test, App};
use gaelspell_server::configure;

#[actix_web::test]
async fn health_endpoint() {
    std::env::set_var("GAELSPELL_DISABLE_PERL", "1");
    let app = test::init_service(App::new().configure(configure)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

use actix_web::{http::StatusCode, test, App};

#[actix_rt::test]
async fn test_health() {
    let app = test::init_service(App::new().configure(api_service_rs::routes::config_routes)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

use actix_web::{http::StatusCode, test, App};
use api_service_rs::controllers::health::health;

#[actix_rt::test]
async fn test_health_endpoint() {

    let app = test::init_service(App::new().service(health)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(test::read_body(resp).await, "Healthy");
}

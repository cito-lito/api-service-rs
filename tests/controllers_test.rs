use actix_web::{http::StatusCode, test, web, App};
use api_service_rs::{models::trainer::TrainerDto, server::AppState};
use serde_json;

mod utils;
use crate::utils::{get_test_pool_db, truncate_all_tables};

#[actix_rt::test]
async fn test_health_endpoint() {
    let app = test::init_service(App::new().configure(api_service_rs::routes::config_routes)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(test::read_body(resp).await, "Healthy");
}

#[actix_rt::test]
async fn test_create_trainer_endpoint() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(api_service_rs::routes::config_routes),
    )
    .await;

    let resp = test::TestRequest::post()
        .uri("/trainer")
        .set_json(&TrainerDto {
            name: "Ash".to_string(),
            level: 1,
        })
        .send_request(&app)
        .await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;
    let body_json =
        serde_json::from_slice::<TrainerDto>(&body_bytes).expect("Failed to parse body");

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body_json.name, "Ash".to_string());
    assert_eq!(body_json.level, 1);
}

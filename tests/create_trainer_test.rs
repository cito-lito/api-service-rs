use actix_web::{
    http::StatusCode,
    test,
    web::{self},
    App,
};
use api_service_rs::{
    models::trainer::Trainer,
    server::AppState,
};
use serde_json::json;

mod utils;
use crate::utils::{get_test_pool_db, truncate_all_tables};

#[actix_rt::test]
async fn test_create_trainer() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(api_service_rs::routes::config_routes),
    )
    .await;

    let mock_trainer_dto = create_json_trainer("Ash", 1);

    let resp = test::TestRequest::post()
        .uri("/trainers")
        .set_json(&mock_trainer_dto)
        .send_request(&app)
        .await;

    let status = resp.status();
    let body: Trainer = test::read_body_json(resp).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body.name, "Ash");
    assert_eq!(body.level, 1);
}

#[actix_rt::test]
async fn test_create_trainer_with_repeated_name() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(api_service_rs::routes::config_routes),
    )
    .await;
    
    let mock_trainer_dto = create_json_trainer("Ash", 1);

    let _ = test::TestRequest::post()
        .uri("/trainers")
        .set_json(&mock_trainer_dto)
        .send_request(&app)
        .await;

    let resp = test::TestRequest::post()
        .uri("/trainers")
        .set_json(&mock_trainer_dto)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_create_trainer_invalid_dto() {
    let pool = get_test_pool_db().await;
    truncate_all_tables(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: pool }))
            .configure(api_service_rs::routes::config_routes),
    )
    .await;

    let mock_trainer_dto = create_json_trainer("ABC", 51);

    let resp = test::TestRequest::post()
        .uri("/trainers")
        .set_json(&mock_trainer_dto)
        .send_request(&app)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

fn create_json_trainer(name: &str, level: u8) -> serde_json::Value {
    json!({
        "name": name,
        "level": level,
    })
}
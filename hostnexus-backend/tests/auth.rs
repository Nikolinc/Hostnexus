use actix_web::{test, App};
use hostnexus_backend::{api::mod_api, db::init_db};
use serde_json::json;
use sqlx::SqlitePool;

#[actix_rt::test]
async fn test_register_and_login() {
    let pool = init_db().await.unwrap();

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(mod_api),
    )
    .await;

    // Register user
    let register_payload = json!({
        "username": "testuser",
        "password": "testpass"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Login user
    let login_payload = json!({
        "username": "testuser",
        "password": "testpass"
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("token").is_some());
}

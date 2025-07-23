use actix_web::{test, App};
use hostnexus_backend::{api::mod_api};
use sqlx::{SqlitePool};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}

#[actix_rt::test]
async fn test_users_get() {
    let pool = setup_test_db().await;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool))
            .configure(mod_api)
    )
    .await;

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

use sqlx::sqlite::SqlitePool;
use dotenvy::dotenv;
use std::env;

pub mod user;

pub async fn init_db() -> SqlitePool {
    dotenv().ok(); // Загружаем .env

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = SqlitePool::connect(&database_url).await.expect("DB connection failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("DB migration failed");

    pool
}
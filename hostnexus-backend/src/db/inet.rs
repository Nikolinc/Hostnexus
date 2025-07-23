// В модуле db/init.rs
pub async fn init_db() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

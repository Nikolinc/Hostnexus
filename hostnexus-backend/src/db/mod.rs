use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

pub mod user;

pub async fn init_db() -> anyhow::Result<SqlitePool> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:hostnexus.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Applying migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations applied");

    Ok(pool)
}

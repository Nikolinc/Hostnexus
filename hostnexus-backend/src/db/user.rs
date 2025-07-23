use sqlx::{FromRow, SqlitePool};
use serde::{Deserialize, Serialize};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

pub async fn create_user(pool: &SqlitePool, new_user: NewUser) -> Result<User, sqlx::Error> {
    let rec = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES (?, ?) RETURNING id, username, email",
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

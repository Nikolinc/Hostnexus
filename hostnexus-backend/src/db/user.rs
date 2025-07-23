use crate::models::user::{User, NewUser};
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, new_user: NewUser) -> Result<User, sqlx::Error> {
    let rec = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role)
        VALUES (?, ?, ?)
        RETURNING id, username, password_hash, role
        "#,
        new_user.username,
        new_user.password_hash,
        new_user.role
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash, role FROM users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn find_user_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash, role FROM users WHERE id = ?",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user_by_id(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_password_hash_by_username(
    db: &SqlitePool,
    username: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = ?",
        username
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| r.password_hash))
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
    User,
    "SELECT id, username, password_hash, role FROM users WHERE username = ?",
    username
)
.fetch_optional(pool)
.await
}
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    #[allow(dead_code)]
    pub password_hash: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
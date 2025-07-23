use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::User;
use super::jwt::{generate_token};
use super::password::{hash_password, verify_password};

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

pub async fn register(
    pool: web::Data<SqlitePool>,
    payload: web::Json<RegisterInput>,
) -> impl Responder {
    let hash = hash_password(&payload.password).unwrap();

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?) RETURNING id, username, password_hash, role"
    )
    .bind(&payload.username)
    .bind(&hash)
    .bind("user")
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Register error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    payload: web::Json<LoginInput>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(u)) => {
            if verify_password(&payload.password, &u.password_hash) {
                let token = generate_token(&u.username, &u.role).unwrap();
                HttpResponse::Ok().json(TokenResponse { token })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        _ => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::db::user::{User, NewUser, create_user as db_create_user};

#[post("/users")]
async fn create_user(user: web::Json<NewUser>, pool: web::Data<SqlitePool>) -> impl Responder {
    match db_create_user(pool.get_ref(), user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::BadRequest().body("User creation failed")
        }
    }
}

#[get("/users")]
async fn list_users(pool: web::Data<SqlitePool>) -> impl Responder {
    let users = sqlx::query_as::<_, User>("SELECT id, username, email FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(users)
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(list_users);
}

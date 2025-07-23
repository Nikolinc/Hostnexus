use actix_web::{get, post, web, delete,HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::user::{NewUser, LoginRequest};
use crate::db::user::{create_user, list_users, find_user_by_id, delete_user_by_id, get_password_hash_by_username, find_user_by_username};
use argon2::{Argon2, PasswordHash, PasswordVerifier};


#[post("/users")]
async fn create_user_handler(user: web::Json<NewUser>, pool: web::Data<SqlitePool>) -> impl Responder {
    match create_user(pool.get_ref(), user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::BadRequest().body("User creation failed")
        }
    }
}

#[get("/users")]
async fn list_users_handler(pool: web::Data<SqlitePool>) -> impl Responder {
    match list_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to list users: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

#[get("/users/search")]
async fn search_user(
    pool: web::Data<SqlitePool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    if let Some(id_str) = query.get("id") {
        if let Ok(id) = id_str.parse::<i64>() {
            match find_user_by_id(pool.get_ref(), id).await {
                Ok(Some(user)) => return HttpResponse::Ok().json(user),
                Ok(None) => return HttpResponse::NotFound().body("User not found"),
                Err(e) => {
                    eprintln!("Search by id error: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        } else {
            return HttpResponse::BadRequest().body("Invalid id parameter");
        }
    }

    HttpResponse::BadRequest().body("Provide 'id' query parameter")
}

#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
     match delete_user_by_id(pool.get_ref(), *id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().body(format!("User with id {} deleted", id))
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/auth")]
async fn login(
    pool: web::Data<SqlitePool>,
    creds: web::Json<LoginRequest>,
) -> impl Responder {
    println!("Login attempt for user: {}", &creds.username);

    let hash_opt = get_password_hash_by_username(pool.get_ref(), &creds.username).await;
    let hash = match hash_opt {
        Ok(Some(h)) => h,
        _ => {
            println!("User not found or DB error");
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
    };

    let parsed_hash = PasswordHash::new(&hash);
    if parsed_hash.is_err() {
        println!("Corrupted password hash");
        return HttpResponse::InternalServerError().body("Corrupted password hash");
    }

    let argon2 = Argon2::default();
    let is_valid = argon2
        .verify_password(creds.password.as_bytes(), &parsed_hash.unwrap())
        .is_ok();

    if !is_valid {
        println!("Invalid password");
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    // Получаем пользователя для возврата
    match find_user_by_username(pool.get_ref(), &creds.username).await {
        Ok(Some(user)) => {
            println!("Auth successful");
            HttpResponse::Ok().json(user)
        }
        Ok(None) => {
            println!("User not found after password check");
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal error")
        }
    }
}


pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user_handler);
    cfg.service(list_users_handler);
    cfg.service(search_user);
    cfg.service(delete_user);
    cfg.service(login);
}
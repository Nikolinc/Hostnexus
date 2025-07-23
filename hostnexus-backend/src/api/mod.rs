use actix_web::{ web};
use crate::api::user::user_routes;
use crate::auth;

pub mod user;

pub fn mod_api(cfg: &mut web::ServiceConfig) {
    user_routes(cfg);
    cfg.service(
        web::scope("/api")
            .configure(auth::auth_config)
    );
}


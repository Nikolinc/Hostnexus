use actix_web::{ web};
use crate::api::user::user_routes;

pub mod user;

pub fn mod_api(cfg: &mut web::ServiceConfig) {
    user_routes(cfg);
}


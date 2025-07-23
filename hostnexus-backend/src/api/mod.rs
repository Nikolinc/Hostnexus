pub mod user;

use actix_web::web;

pub fn mod_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user::user_routes)
    );
}

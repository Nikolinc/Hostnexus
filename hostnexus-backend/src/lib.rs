use actix_web::{App, HttpServer, web};
use crate::api::mod_api;
use crate::db::init_db;

pub mod models;
pub mod api;
pub mod db;
pub mod auth;


pub async fn run() -> std::io::Result<()> {
    let db = init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(mod_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_something() {
        // твой тест здесь
    }
}

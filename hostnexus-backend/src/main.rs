mod api;
mod db;
mod models;
mod startup;
mod utils;

use actix_web::{App, HttpServer, web};
use crate::startup::init::init_admin;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_db().await.expect("Не удалось инициализировать БД");

    init_admin(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::mod_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

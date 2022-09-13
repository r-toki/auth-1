mod application;
mod domain;
mod infrastructure;
mod lib;
mod module;
mod presentation;

use crate::lib::config::CONFIG;
use crate::module::Modules;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = &CONFIG.port;
    let database_url = &CONFIG.database_url;

    let pool = PgPool::connect(database_url).await.unwrap();

    let modules = Modules::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(modules.clone()))
            .configure(presentation::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

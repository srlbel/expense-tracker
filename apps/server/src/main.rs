use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use config::Config;
use dotenv::dotenv;
use log::info;

mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::from_env();
    env_logger::init();

    let db_pool = infrastructure::database::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4321")
            .max_age(3600);

        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(interfaces::http::configure)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

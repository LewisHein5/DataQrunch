use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{middleware::Logger, App, HttpServer, web};
use actix_web::cookie::Key;
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger;

use api::datasets::handlers::{getDataset, listDatasets, createDataset};
use authentication::handlers::validate_request;
use redis_manager::RedisManager;
use crate::api::datasets::handlers::put_dataset::updateDataset;

mod redis_manager;
mod authentication;
mod api;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug"); //TODO: Don't set this here, do it in the docker container
    std::env::set_var("RUST_BACKTRACE", "1"); //TODO: same
    env_logger::init();
    let host = String::from("127.0.0.1");
    let port = 6379;
    let redis_connection_string = format!("{}:{}", host, port);
    let redis_manager =
        web::Data::new(RedisManager::new(host, port).expect("Could not connect to redis"));

    let secret_key = Key::generate();
    //todo: Require compression
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validate_request);
        App::new()
            .wrap(Logger::default())
            .wrap(auth)
            .wrap(Cors::permissive())//TODO: Set this more securely
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(&redis_connection_string),
                secret_key.clone(),
            ))
            .app_data(redis_manager.clone())
            .service(getDataset)
            .service(createDataset)
            .service(listDatasets)
            .service(updateDataset)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


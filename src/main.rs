use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{App, HttpServer, web};
use actix_web::cookie::Key;
use actix_web_httpauth::middleware::HttpAuthentication;
use simple_logger::SimpleLogger;

use api::datasets::handlers::{get_dataset, list_datasets, new_dataset};
use authentication::handlers::validate_request;
use redis_manager::RedisManager;

mod log_error;
mod redis_manager;
mod authentication;
mod api;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .init()
        .expect("Could not initialize logging");
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
            .wrap(auth)
            .wrap(Cors::permissive())//TODO: Set this more securely
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(&redis_connection_string),
                secret_key.clone(),
            ))
            .app_data(redis_manager.clone())
            .route("/datasets/new", web::post().to(new_dataset))
            .route("/datasets/{id}", web::get().to(get_dataset))
            .route(
                "/datasets",
                web::get().to(list_datasets),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


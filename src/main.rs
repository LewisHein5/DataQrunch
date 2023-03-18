mod authenticate_user;
mod dataset;
mod dataset_dto;
mod get_dataset;
mod log_error;
mod login;
mod new_dataset;
mod redis_manager;
mod session_key;
mod user_session_data;
mod user_session_data_cache;
mod get_datasets_list;

use new_dataset::new_dataset;
use user_session_data_cache::UserSessionDataCache;

use crate::redis_manager::RedisManager;
use actix_web::{web, App, HttpServer};
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .init()
        .expect("Colud not initialize logging");
    let host = String::from("127.0.0.1");
    let port = 6379;
    let user_session_key_store = web::Data::new(UserSessionDataCache::new());
    let redis_manager =
        web::Data::new(RedisManager::new(host, port).expect("Could not connect to redis"));

    //todo: Require compression
    HttpServer::new(move || {
        App::new()
            .app_data(user_session_key_store.clone())
            .app_data(redis_manager.clone())
            .route("/datasets/new", web::post().to(new_dataset))
            .route("/login", web::post().to(login::login))
            .route("/datasets/{id}", web::get().to(get_dataset::get_dataset))
            .route("/datasets", web::get().to(get_datasets_list::get_datasets_list))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

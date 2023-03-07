mod dataset_dto;
mod new_dataset;
mod session_key;
mod login;
mod user_session_data_cache;
mod user_session_data;
mod dataset;
mod get_dataset;

use new_dataset::new_dataset;
use user_session_data_cache::UserSessionDataCache;

use std::collections::HashMap;
use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_session_key_store = web::Data::new(UserSessionDataCache::new());

    //todo: Require compression
    HttpServer::new(move || {
        App::new()
            .app_data(user_session_key_store.clone())
            .route("/datasets/new", web::post().to(new_dataset))
            .route("/login", web::post().to(login::login))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
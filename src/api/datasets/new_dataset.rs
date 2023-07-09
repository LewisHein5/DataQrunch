use super::models::dataset::Dataset;


use crate::log_error;
use crate::redis_manager::RedisManager;
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use std::path::Path;
use uuid::Uuid;
use crate::api::get_authenticated_user_id::get_authenticated_user_id;

pub(crate) async fn new_dataset(
    req: HttpRequest,
    dataset: web::Json<Dataset>,
    redis_manager: web::Data<RedisManager>,
) -> impl Responder {
    let user_id = get_authenticated_user_id(req);

    let dataset_uuid = Uuid::new_v4();
    let out_path = Path::new("/datasets")
        .join(&user_id)
        .join(dataset_uuid.to_string());

    match redis_manager.new_dataset_for_user(&user_id, &dataset_uuid, &out_path) {
        Ok(_) => {}
        Err(e) => {
            let error_json = log_error!("Error adding dataset to redis {}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(error_json);
        }
    };

    let dataset_size = match dataset.try_save_to(&out_path) {
        Ok(val) => val,
        Err(e) => {
            let error_json = log_error!(
                "Could not write file {}: {}",
                out_path.to_string_lossy(),
                e.to_string()
            );
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(error_json);
        }
    };

    match redis_manager.set_dataset_size(&user_id, dataset_uuid, dataset_size) {
        Ok(_) => (),
        Err(e) => {
            let error_json = log_error!(
                "Could not set dataset size for user {}, dataset {}",
                user_id,
                dataset_uuid
            );
            return HttpResponse::InternalServerError().body(error_json);
        }
    }

    return HttpResponse::Ok().body(format!("{{uuid: {}}}", dataset_uuid.to_string()));
}

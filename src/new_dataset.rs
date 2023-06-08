use super::dataset::Dataset;
use super::dataset_dto::DatasetDto;
use super::user_session_data_cache::UserSessionDataCache;
use crate::authenticate_user::get_user_session_data;
use crate::log_error;
use crate::redis_manager::RedisManager;
use actix_web::{web, HttpResponse, Responder};
use std::path::Path;
use uuid::Uuid;

pub(crate) async fn new_dataset(
    data: web::Json<DatasetDto>,
    user_data_cache: web::Data<UserSessionDataCache>,
    redis_manager: web::Data<RedisManager>,
) -> impl Responder {
    let user_data = match get_user_session_data(&data.session_key, user_data_cache) {
        Ok(val) => val,
        Err(e) => return e,
    };

    let dataset_uuid = Uuid::new_v4();
    let out_path = Path::new("/datasets")
        .join(&user_data.user_name)
        .join(dataset_uuid.to_string());

    match redis_manager.new_dataset_for_user(&user_data.user_name, &dataset_uuid, &out_path) {
        Ok(_) => {}
        Err(e) => {
            let error_json = log_error!("Error adding dataset to redis {}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(error_json);
        }
    };

    let dataset = match Dataset::try_from(data) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
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

    match redis_manager.set_dataset_size(&user_data.user_name, dataset_uuid, dataset_size) {
        Ok(_) => (),
        Err(e) => {
            let error_json = log_error!(
                "Could not set dataset size for user {}, dataset {}",
                user_data.user_name,
                dataset_uuid
            );
            return HttpResponse::InternalServerError().body(error_json);
        }
    }

    return HttpResponse::Ok().body(format!("{{uuid: {}}}", dataset_uuid.to_string()));
}

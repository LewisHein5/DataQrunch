use crate::authenticate_user::get_user_session_data;
use crate::dataset::Dataset;
use crate::log_error;
use crate::redis_manager::RedisManager;
use crate::user_session_data_cache::UserSessionDataCache;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct DatasetRequest {
    session_key: String,
}

pub(crate) async fn get_dataset(
    req: web::Json<DatasetRequest>,
    path: web::Path<String>,
    user_data_cache: web::Data<UserSessionDataCache>,
    redis_manager: web::Data<RedisManager>,
) -> impl Responder {
    let user_data = match get_user_session_data(&req.session_key, user_data_cache) {
        Ok(val) => val,
        Err(e) => return e,
    };

    let dataset_uuid = match Uuid::parse_str(path.as_str()) {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest().body("Invalid dataset name (Must be a UUID)"),
    };

    let dataset_path = match redis_manager.get_dataset_path(&user_data.user_name, &dataset_uuid) {
        Some(val) => match val {
            Ok(val) => val,
            Err(e) => {
                let error_json = log_error!(
                    "Could not get path for user {}, dataset id {}. Error condition: {}",
                    user_data.user_name,
                    dataset_uuid,
                    e.to_string()
                );
                return HttpResponse::InternalServerError().body(error_json);
            }
        },
        None => {
            return HttpResponse::NotFound().body("No such dataset");
        }
    };

    if !dataset_path.exists() {
        return HttpResponse::NotFound().body("");
    }

    let dataset = match Dataset::load_from(dataset_path.to_string_lossy().to_string()) {
        Ok(val) => val,
        Err(e) => {
            let error_json = log_error!(
                "Could not load dataset {} for user {}. Error condition: {}",
                dataset_uuid,
                user_data.user_name,
                e.to_string()
            );
            return HttpResponse::InternalServerError().body(error_json);
        }
    };

    return match dataset.to_json() {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(e) => {
            let error_json = log_error!(
                "Error converting dataset {} for user {} to JSON. Error condition: {}",
                dataset_uuid,
                user_data.user_name,
                e.to_string()
            );
            HttpResponse::InternalServerError().body(error_json)
        }
    };
}

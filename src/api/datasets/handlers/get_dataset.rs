use actix_web::{HttpRequest, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::log_error;
use crate::redis_manager::RedisManager;

use super::super::models::Dataset;
use super::super::super::get_authenticated_user_id::get_authenticated_user_id;

pub async fn get_dataset(
    req: HttpRequest,
    path: web::Path<String>,
    redis_manager: web::Data<RedisManager>,
) -> impl Responder {
    let user_id = get_authenticated_user_id(req);
    let dataset_uuid = match Uuid::parse_str(path.as_str()) {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest().body("Invalid dataset name (Must be a UUID)"),
    };

    let dataset_path = match redis_manager.get_dataset_path(&user_id, &dataset_uuid) {
        Some(val) => match val {
            Ok(val) => val,
            Err(e) => {
                let error_json = log_error!(
                    "Could not get path for user {}, dataset id {}. Error condition: {}",
                    user_id,
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
                user_id,
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
                user_id,
                e.to_string()
            );
            HttpResponse::InternalServerError().body(error_json)
        }
    };
}
use crate::get_user_session_data::get_user_session_data;
use crate::log_error;
use crate::redis_manager::RedisManager;
use crate::user_session_data_cache::UserSessionDataCache;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError, HttpResponseBuilder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Serialize};
use uuid::Uuid;
use crate::authentication::handlers::get_user_id::get_authenticated_user_id;

pub(crate) struct DatasetsListRequest {
    session_key: String,
}

#[derive(Serialize)]
pub(crate) struct DatasetInfo {
    id: String,
    size: u64,
}
pub(crate) async fn get_datasets_list(
    req: HttpRequest,
    credentials: BearerAuth,
    redis_manager: web::Data<RedisManager>,
) -> impl Responder {
    let user_id = match get_authenticated_user_id(credentials.token()).await {
        Ok(val) => val,
        Err(e) => return e.error_response()
    };

    let session_key = req
        .headers()
        .get("UserId")
        .expect("Grr")
        .to_str()
        .expect("grr"); //TODO: don't panic

    let dataset_ids_list = match redis_manager.list_datasets(&user_id) {
        Ok(val) => val,
        Err(e) => {
            let error_json = log_error!(
                "Could not list datasets for user {}, Error condition: {}",
                user_id,
                e.to_string()
            );
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(error_json);
        }
    };

    let mut datasets_info_list = Vec::<DatasetInfo>::new();
    for dataset_id in dataset_ids_list {
        let dataset_size = match redis_manager.get_dataset_size(&user_id, dataset_id) {
            Ok(val) => val,
            Err(e) => {
                let error_json = log_error!(
                    "Could not get dataset size for user {}, dataset {}",
                    user_id,
                    dataset_id
                );
                return HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(error_json);
            }
        };
        datasets_info_list.push(DatasetInfo {
            id: dataset_id.to_string(),
            size: dataset_size,
        })
    }

    let datasets_info_json = match serde_json::to_string(&datasets_info_list) {
        Ok(val) => val,
        Err(e) => {
            let error_json = log_error!("Could not serialize response");
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(error_json);
        }
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(datasets_info_json);
}

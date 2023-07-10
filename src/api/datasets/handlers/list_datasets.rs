use actix_web::{error, get, HttpRequest, HttpResponse, web};

use crate::redis_manager::RedisManager;
use crate::services::datasets::list_datasets;

use super::super::super::get_authenticated_user_id::get_authenticated_user_id;

#[get("/datasets")]
pub async fn listDatasets(
    req: HttpRequest,
    redis_manager: web::Data<RedisManager>,
) -> actix_web::Result<HttpResponse, error::Error>{
    let user_id = get_authenticated_user_id(req);

    let datasets_info_list = list_datasets::listDatasets(redis_manager, &user_id)?;

    return Ok(HttpResponse::Ok().json(datasets_info_list));
}

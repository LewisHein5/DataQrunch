use actix_web::{error, get, HttpRequest, HttpResponse, Responder, ResponseError, web};
use uuid::Uuid;
use crate::services::datasets::get_dataset;

use crate::redis_manager::RedisManager;

use super::super::models::Dataset;
use super::super::super::get_authenticated_user_id::get_authenticated_user_id;

#[get("/datasets/{id}")]
pub async fn getDataset(
    req: HttpRequest,
    path: web::Path<String>,
    redis_manager: web::Data<RedisManager>,
) -> actix_web::Result<HttpResponse, error::Error> {
    let user_id = get_authenticated_user_id(req);
    let dataset_uuid = Uuid::parse_str(path.as_str())
        .map_err(|_| error::ErrorBadRequest("Invalid dataset name (Must be a UUID)"))?;

    let dataset = get_dataset::getDataset(redis_manager, &user_id, &dataset_uuid).await?;

    return Ok(HttpResponse::Ok().json(dataset));
}
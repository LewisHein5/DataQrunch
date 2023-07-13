use actix_web::{error, HttpRequest, HttpResponse, put, web};
use uuid::Uuid;

use crate::redis_manager::RedisManager;
use crate::services::datasets::update_dataset;
use crate::services::datasets::get_dataset::getDataset;

use super::super::models::Dataset;
use super::super::super::get_authenticated_user_id::get_authenticated_user_id;

#[put("/datasets/{id}")]
pub async fn updateDataset(
    req: HttpRequest,
    path: web::Path<String>,
    dataset: web::Json<Dataset>,
    redis_manager: web::Data<RedisManager>,
) -> actix_web::Result<HttpResponse, error::Error> {
    let user_id = get_authenticated_user_id(req);
    let path = Uuid::parse_str(path.as_str()).map_err(|_| error::ErrorBadRequest("Invalid dataset name (Must be a UUID)"))?;
    let dataset_uuid = update_dataset::updateDataset(dataset, &redis_manager, &user_id,path)?;

    return Ok(HttpResponse::Ok().json(getDataset(redis_manager,&user_id,&dataset_uuid).await?));
}
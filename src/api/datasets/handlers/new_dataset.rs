use actix_web::{error, HttpRequest, HttpResponse, post, web};

use crate::redis_manager::RedisManager;
use crate::services::datasets::create_dataset;
use crate::services::datasets::get_dataset::getDataset;

use super::super::models::Dataset;
use super::super::super::get_authenticated_user_id::get_authenticated_user_id;

#[post("/datasets/new")]
pub async fn createDataset(
    req: HttpRequest,
    dataset: web::Json<Dataset>,
    redis_manager: web::Data<RedisManager>,
) -> actix_web::Result<HttpResponse, error::Error> {
    let user_id = get_authenticated_user_id(req);


    let dataset_uuid = create_dataset::createDataset(dataset, &redis_manager, &user_id)?;

    return Ok(HttpResponse::Created().json(getDataset(redis_manager,&user_id,&dataset_uuid).await?));
}

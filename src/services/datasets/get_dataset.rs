use actix_web::error;
use actix_web::web::Data;
use uuid::Uuid;
use crate::redis_manager::RedisManager;
use crate::api::datasets::models::dataset::Dataset;

pub async fn getDataset(redis_manager: Data<RedisManager>, user_id: &String, dataset_uuid: &Uuid) -> Result<Dataset, error::Error> {
    let dataset_path = redis_manager.get_dataset_path(&user_id, &dataset_uuid)
        .ok_or(error::ErrorNotFound(""))?
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let dataset = Dataset::load_from(
        &dataset_path.to_string_lossy().to_string()
    ).await?;
    Ok(dataset)
}
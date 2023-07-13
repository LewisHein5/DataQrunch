use std::fs::create_dir_all;
use std::path::Path;
use actix_web::error;
use actix_web::web::{Data, Json};
use uuid::Uuid;
use crate::api::datasets::models::dataset::Dataset;
use crate::redis_manager::RedisManager;
use crate::services::datasets::get_user_dir_name::get_user_dir_name;

pub(crate) fn updateDataset(dataset: Json<Dataset>, redis_manager: &Data<RedisManager>, user_id: &String, dataset_uuid: Uuid) -> Result<Uuid, error::Error> {
    let user_dir = get_user_dir_name(user_id);

    let out_path = Path::new("/datasets")
        .join(&user_dir)
        .join(dataset_uuid.to_string());

    redis_manager.update_dataset_modify_time(&user_id, &dataset_uuid)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let dataset_size = dataset.try_save_to(&out_path)?;
    redis_manager.set_dataset_size(&user_id, dataset_uuid, dataset_size)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(dataset_uuid)
}


use std::fs::create_dir_all;
use std::path::Path;
use actix_web::error;
use actix_web::web::{Data, Json};
use uuid::Uuid;
use crate::api::datasets::models::dataset::Dataset;
use crate::redis_manager::RedisManager;
use crate::services::datasets::get_user_dir_name::get_user_dir_name;

pub(crate) fn createDataset(dataset: Json<Dataset>, redis_manager: &Data<RedisManager>, user_id: &String) -> Result<Uuid, error::Error> {
    let user_dir = get_user_dir_name(user_id);


    let dataset_uuid = Uuid::new_v4();
    let out_dir = Path::new("/datasets")
        .join(&user_dir);
    create_dir_all(&out_dir)?;
    let out_path = out_dir.join(dataset_uuid.to_string());

    redis_manager.new_dataset_for_user(&user_id, &dataset_uuid, &out_path)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let dataset_size = dataset.try_save_to(&out_path)?;
    redis_manager.set_dataset_size(&user_id, dataset_uuid, dataset_size)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(dataset_uuid)
}


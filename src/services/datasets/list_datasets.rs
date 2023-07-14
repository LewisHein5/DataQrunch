use actix_web::error;
use actix_web::web::Data;
use chrono::DateTime;
use crate::redis_manager::RedisManager;
use crate::api::datasets::models::dataset_info::DatasetInfo;

pub(crate) fn listDatasets(redis_manager: Data<RedisManager>, user_id: &String) -> Result<Vec<DatasetInfo>, error::Error> {
    let dataset_ids_list = redis_manager.list_datasets(&user_id)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let mut datasets_info_list = Vec::<DatasetInfo>::new();
    for dataset_id in dataset_ids_list {
        let dataset_size = redis_manager.get_dataset_size(
            &user_id, dataset_id
        ).map_err(|e| error::ErrorInternalServerError(e))?;
        let dataset_time = redis_manager.get_dataset_timestamp(user_id, &dataset_id).map_err(|e| error::ErrorInternalServerError(e))?;
        let dataset_name = redis_manager.get_dataset_name(user_id, &dataset_id).map_err(|e| error::ErrorInternalServerError(e))?;
        datasets_info_list.push(DatasetInfo {
            name: dataset_name,
            id: dataset_id.to_string(),
            size: dataset_size,
            modified: dataset_time
        })
    }
    Ok(datasets_info_list)
}
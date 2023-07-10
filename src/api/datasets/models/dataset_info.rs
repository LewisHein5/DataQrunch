use actix_web::error;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DatasetInfo {
    pub(crate) id: String,
    pub(crate) size: u64,
}
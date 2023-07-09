use serde::Serialize;

#[derive(Serialize)]
pub struct DatasetInfo {
    pub(crate) id: String,
    pub(crate) size: u64,
}
use serde::Deserialize;


#[derive(Deserialize)]
pub(crate) struct DatasetDto {
    pub(crate) session_key: String,
    pub(crate) header: Vec<String>,
    pub(crate) data_types: Vec<String>,
    pub(crate) columns: Vec<Vec<String>>
}
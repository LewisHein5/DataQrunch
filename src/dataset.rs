use rmp_serde;
use std::fs::File;
use std::io::Read;
use std::io;

use flate2::{GzBuilder, Compression};
use serde::{Serialize, Deserialize};
use crate::dataset_dto::DatasetDto;
use actix_web::web;
use flate2::read::GzDecoder;

#[derive(Serialize,Deserialize)]
pub(crate) struct Dataset {
    pub(crate) header: Vec<String>,
    pub(crate) data_types: Vec<String>,
    pub(crate) columns: Vec<Vec<String>>
}

impl From<web::Json<DatasetDto>> for Dataset {
    fn from(dataset_dto: web::Json<DatasetDto>) -> Dataset {
        return Dataset {
            header: dataset_dto.header.clone(),
            data_types: dataset_dto.data_types.clone(),
            columns: dataset_dto.columns.clone()
        };
    }
}

impl Dataset {
    fn _try_save_to(&self, file: File) -> Result<(), rmp_serde::encode::Error> {
        let mut gz = GzBuilder::new()
            .write(file, Compression::fast());

        return self.serialize(&mut rmp_serde::Serializer::new(&mut gz));
    }

    pub(crate) fn try_save_to(&self, file_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(file_name)?;
        self._try_save_to(file)?;

        return Ok(())
    }

    pub(crate) fn load_from(file_name: String) -> Result<Dataset, Box<dyn std::error::Error>> {
        let file = File::open(file_name)?; //Todo: Match error here

        let mut decoder_buf = io::BufReader::new(GzDecoder::new(file));
        //let mut binding = decoder_buf.buffer();


        let mut deserializer = rmp_serde::decode::Deserializer::new(&mut decoder_buf);
        return match  Dataset::deserialize(&mut deserializer) {
            Ok(val) => Ok(val),
            Err(e) => Err(e)?
        };
    }

    pub(crate) fn to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(&self);
    }
}
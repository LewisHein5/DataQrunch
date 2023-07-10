
use std::io;

use std::fs::File;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{error};
use flate2::{Compression, GzBuilder};
use flate2::read::GzDecoder;
use rmp_serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub(crate) header: Vec<String>,
    pub(crate) data_types: Vec<String>,
    pub(crate) columns: Vec<Vec<String>>,
}

impl Dataset {
    fn _try_save_to(&self, file: &File) -> Result<(), rmp_serde::encode::Error> {
        let mut gz = GzBuilder::new().write(file, Compression::fast());

        return self.serialize(&mut rmp_serde::Serializer::new(&mut gz));
    }

    pub(crate) fn try_save_to(&self, file_name: &Path) -> Result<u64, error::Error> {
        let file = File::create(file_name)?;
        self._try_save_to(&file).map_err(|e| error::ErrorBadRequest(e))?;
        return Ok(file.metadata()?.len());
    }

    pub(crate) async fn load_from(file_name: &String) -> Result<Dataset, error::Error> {
        let file = NamedFile::open_async(file_name).await?;
        let mut decoder_buf = io::BufReader::new(GzDecoder::new(file.file()));
        //let mut binding = decoder_buf.buffer();

        let mut deserializer = rmp_serde::decode::Deserializer::new(&mut decoder_buf);
        return Dataset::deserialize(&mut deserializer)
            .map_err(
                |_| error::ErrorInternalServerError(format!("Corrupted file: {}", file_name))
            );
    }

    pub(crate) fn to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(&self);
    }
}

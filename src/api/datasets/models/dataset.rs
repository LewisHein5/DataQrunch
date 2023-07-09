use rmp_serde;
use std::fs::File;
use std::path::Path;
use std::{fmt, io};

use flate2::read::GzDecoder;
use flate2::{Compression, GzBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Dataset {
    pub(crate) header: Vec<String>,
    pub(crate) data_types: Vec<String>,
    pub(crate) columns: Vec<Vec<String>>,
}

#[derive(Debug)]
pub(crate) enum DatasetFormatError {
    DataTypesWrongLength(usize, usize),
    WrongNumberOfColumns(usize, usize),
    ColumnLengthsDiffer
}

impl fmt::Display for DatasetFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatasetFormatError::DataTypesWrongLength(h,d) => write!(f, "Header has {} columns but data types list has {}", h, d),
            DatasetFormatError::WrongNumberOfColumns(h, n_col) => write!(f, "Header has {} columns but data contains {} columns", h, n_col),
            DatasetFormatError::ColumnLengthsDiffer => write!(f, "Not all columns contain the same number of rows")
        }
    }
}

impl Dataset {
    fn _try_save_to(&self, file: &File) -> Result<(), rmp_serde::encode::Error> {
        let mut gz = GzBuilder::new().write(file, Compression::fast());

        return self.serialize(&mut rmp_serde::Serializer::new(&mut gz));
    }

    pub(crate) fn try_save_to(&self, file_name: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let file = File::create(file_name)?;
        self._try_save_to(&file)?;
        return Ok(file.metadata()?.len());
    }

    pub(crate) fn load_from(file_name: String) -> Result<Dataset, Box<dyn std::error::Error>> {
        let file = File::open(file_name)?; //Todo: Match error here

        let mut decoder_buf = io::BufReader::new(GzDecoder::new(file));
        //let mut binding = decoder_buf.buffer();

        let mut deserializer = rmp_serde::decode::Deserializer::new(&mut decoder_buf);
        return match Dataset::deserialize(&mut deserializer) {
            Ok(val) => Ok(val),
            Err(e) => Err(e)?,
        };
    }

    pub(crate) fn to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(&self);
    }
}

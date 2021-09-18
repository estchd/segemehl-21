use rayon::iter::{ParallelIterator, ParallelBridge};
use bam::{Record, BamReader};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaderCreationError {
    #[error("could not create bam reader at: {path}")]
    BamReader {
        path: String,
        source: std::io::Error
    },
    #[allow(dead_code)]
    #[error("could not create bai reader at: {path}")]
    BaiReader {
        path: String,
        source: std::io::Error
    }
}

pub fn get_parallel_reader(bam_path: &str, bai_path: Option<String>) -> Result<Box<impl ParallelIterator<Item = Result<Record, std::io::Error>> + Sized>, ReaderCreationError> {
    match bai_path {
        None => {
            let reader = BamReader::from_path(bam_path, 16u16).map_err(|err| ReaderCreationError::BamReader {
                path: bam_path.to_string(),
                source: err
            })?;
            Ok(Box::from(reader.into_iter().par_bridge()))
        }
        Some(_path) => {
            let reader = BamReader::from_path(bam_path, 16u16).map_err(|err| ReaderCreationError::BamReader {
                path: bam_path.to_string(),
                source: err
            })?;
            Ok(Box::from(reader.into_iter().par_bridge()))
        }
    }
}
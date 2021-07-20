use rayon::iter::{ParallelIterator, ParallelBridge};
use bam::{Record, BamReader};

pub fn get_parallel_reader(bam_path: &str, bai_path: Option<String>) -> Result<Box<impl ParallelIterator<Item = Result<Record, std::io::Error>> + Sized>, ()> {
    match bai_path {
        None => {
            let reader = BamReader::from_path(bam_path, 16u16).map_err(|_| ())?;
            Ok(Box::from(reader.into_iter().par_bridge()))
        }
        Some(_path) => {
            let reader = BamReader::from_path(bam_path, 16u16).map_err(|_| ())?;
            Ok(Box::from(reader.into_iter().par_bridge()))
        }
    }
}
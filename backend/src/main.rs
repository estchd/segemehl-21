#![feature(linked_list_cursors)]

use std::{
    convert::TryFrom,
    fs::File,
    num::{NonZeroU32}
};
use std::io::Write;

use bam::{BamReader};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use segemehl_21_core::{
    header::header::Header,
};
use crate::command_line::CommandLineParameters;
use crate::reader::get_parallel_reader;
use segemehl_21_core::{
    statistics::calculation::CalculationData,
    statistics::calculation::binned::BinConfig,
    util::get_record_length,
    statistics::presentation::PresentationData
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::convert::TryInto;

mod old_formatting;
mod util;
mod command_line;
mod reader;

fn main() {
    let params = CommandLineParameters::read();

    let bam_path = params.bam_path.as_str();
    let bai_path = params.bai_path;

    let reader = get_parallel_reader(bam_path, bai_path).unwrap();

    let header_reader = BamReader::from_path(bam_path,0u16).unwrap();

    let header = Header::try_from(header_reader.header()).unwrap();

    let bin_config = BinConfig::NumberOfBins(NonZeroU32::new(1000).unwrap());

    let calculation_data = CalculationData::new(&header, bin_config).unwrap();

    let total_record_stats: (AtomicUsize, AtomicUsize) = (AtomicUsize::new(0), AtomicUsize::new(0));

    reader.into_par_iter().filter_map(|item| item.ok()).for_each(|record| {
        let total_records = total_record_stats.0.fetch_add(1, Ordering::Relaxed);
        let _ = total_record_stats.1.fetch_add(get_record_length(&record) as usize, Ordering::Relaxed);

        calculation_data.add_record(record).unwrap();

        if total_records % 10000 == 0 {
            println!("Records read: {}", total_records);
        }
    });

    let (record_count, total_record_length) = (total_record_stats.0.into_inner(), total_record_stats.1.into_inner());

    println!("Converting to Presentation Data");

    let presentation_data: PresentationData = calculation_data.try_into().unwrap();

    println!("Reading Finished");
    println!("Record Count: {}", record_count);
    println!("Total Record Length: {}", total_record_length);
    println!();

    println!("Calculating per Reference Statistics");

    for statistic in presentation_data.get_per_reference_data() {
        println!();
        println!("Reference Name: {}", statistic.get_reference_name());
        println!("Reference Length: {}", statistic.get_reference_length());
        println!("Records for Reference: {}", statistic.get_read_length_map().get_frequency_sum());
        println!("Total Record Length for Reference: {}", statistic.get_read_length_map().get_weighted_frequency_sum());
        println!("Mean Read Length: {}", statistic.get_read_length_map().get_mean_frequency().unwrap_or(0.0));
        println!("Median Read Length: {}", statistic.get_read_length_map().get_median_frequency().unwrap_or(0.0));
        println!("Mode Read Length: {}", statistic.get_read_length_map().get_max_frequency().unwrap_or((0, 0)).0);
        println!("Smallest Read Length: {}", statistic.get_read_length_map().get_min_entry().unwrap_or((0,0)).0);
        println!("Biggest Read Length: {}", statistic.get_read_length_map().get_max_entry().unwrap_or((0,0)).0);
        println!("Total Covered Length: {}", statistic.get_covered_length());
        println!();
    }

    let json = false;

    let mut out_file = File::create(params.output_path).unwrap();

    if json {
        serde_json::to_writer(out_file, &presentation_data).unwrap();
    }
    else {
        let serialized = bincode::serialize(&presentation_data).unwrap();

        out_file.write_all(&serialized).unwrap();
    }

    println!("Finished");
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use bam::BamReader;
    use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

    #[test]
    #[ignore]
    fn iter_speed_test() {
        let total_records = AtomicUsize::new(0);

        let reader = BamReader::from_path("../../example_files/rep1.bam", 8u16).unwrap();

        reader.par_bridge().into_par_iter().filter_map(|item| item.ok()).for_each(|_record| {
            let _current_total = total_records.fetch_add(1, Ordering::Relaxed);
        });

        println!("Total Records: {}", total_records.into_inner());
    }
}
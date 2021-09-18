use std::{
    convert::TryFrom,
    fs::File,
    num::NonZeroU32
};
use std::convert::TryInto;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use bam::BamReader;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use anyhow::{Context};

use segemehl_21_core::{
    statistics::calculation::binned::BinConfig,
    statistics::calculation::CalculationData,
    statistics::presentation::PresentationData,
    util::get_record_length_on_reference
};
pub use segemehl_21_core::header::Header;

use crate::command_line::CommandLineParameters;
use crate::reader::{get_parallel_reader};

mod old_formatting;
mod util;
mod command_line;
mod reader;

fn main() -> anyhow::Result<()> {
    let params = CommandLineParameters::read();

    let bam_path = params.bam_path.as_str();
    let bai_path = params.bai_path;
    let expected_record_count = params.expected_record_count;
    let info_dump = params.info_dump;

    println!();

    if bai_path.is_some() {
        println!(
            "{} Parsing BAM and BAI File Headers...",
            style("[1/4]").bold().dim()
        )
    }
    else {
        println!(
            "{} Parsing BAM File Header...",
            style("[1/4]").bold().dim()
        )
    }

    let reader = get_parallel_reader(bam_path, bai_path)
        .context("could not create record readers")?;

    let header_reader = BamReader::from_path(bam_path,0u16)
        .with_context(|| format!("could not create bam header reader at: {}", bam_path))?;

    let header = Header::try_from(header_reader.header())
        .context("could not read bam header")?;

    println!(
        "{} Reading Records...",
        style("[2/4]").bold().dim()
    );

    let pb = match expected_record_count {
        None => {
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner} [{elapsed_precise}] Records read: {pos}")
                .progress_chars("#>-")
                .tick_chars("/-\\|"));
            pb.set_draw_rate(15);
            pb
        }
        Some(expected_count) => {
            let pb = ProgressBar::new(expected_count as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
                .progress_chars("#>-")
                .tick_chars("/-\\|"));
            pb.set_draw_rate(15);
            pb
        }
    };


    let bin_config = BinConfig::NumberOfBins(NonZeroU32::new(1000).unwrap());

    let calculation_data = CalculationData::new(&header, bin_config).unwrap();

    let total_record_stats: (AtomicUsize, AtomicUsize) = (AtomicUsize::new(0), AtomicUsize::new(0));



    reader.into_par_iter().filter_map(|item| item.ok()).for_each(|record| {
        let total_records = total_record_stats.0.fetch_add(1, Ordering::Relaxed);
        let _ = total_record_stats.1.fetch_add(get_record_length_on_reference(&record) as usize, Ordering::Relaxed);

        calculation_data.add_record(record).unwrap();

        if total_records % 10000 == 0 {
            pb.set_position(total_records as u64);
        }
    });

    let (record_count, total_record_length) = (total_record_stats.0.into_inner(), total_record_stats.1.into_inner());

    pb.finish_and_clear();

    println!(
        "{} Calculating Statistics...",
        style("[3/4]").bold().dim()
    );

    let presentation_data: PresentationData = calculation_data.try_into().unwrap();

    println!();
    println!("Record Count: {}", record_count);
    println!("Total Record Length: {}", total_record_length);
    println!();

    if info_dump {
        println!("Dumping Per Reference Statistics");

        for statistic in presentation_data.get_per_reference_data() {
            println!();
            println!("Reference Name: {}", statistic.get_reference_name());
            println!("Reference Length: {}", statistic.get_reference_length());
            println!("Records for Reference: {}", statistic.get_read_length_on_reference_map().get_frequency_sum());
            println!("Total Record Length for Reference: {}", statistic.get_read_length_on_reference_map().get_weighted_frequency_sum());
            println!("Mean Read Length: {}", statistic.get_read_length_on_reference_map().get_mean_frequency().unwrap_or(0.0));
            println!("Median Read Length: {}", statistic.get_read_length_on_reference_map().get_median_frequency().unwrap_or(0.0));
            println!("Mode Read Length: {}", statistic.get_read_length_on_reference_map().get_max_frequency().unwrap_or((0, 0)).0);
            println!("Smallest Read Length: {}", statistic.get_read_length_on_reference_map().get_min_entry().unwrap_or((0, 0)).0);
            println!("Biggest Read Length: {}", statistic.get_read_length_on_reference_map().get_max_entry().unwrap_or((0, 0)).0);
            println!();
        }
    }


    println!(
        "{} Writing to File...",
        style("[4/4]").bold().dim()
    );


    let pb = ProgressBar::new(3);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner} [{elapsed_precise}] [{bar}] {pos}/{len} ({eta})")
        .progress_chars("#>-")
        .tick_chars("/-\\|"));
    pb.enable_steady_tick(60/15);

    pb.set_message("Creating File...");

    let mut out_file = File::create(params.output_path).unwrap();

    pb.set_position(1);
    pb.set_message("Serializing Data...");

    let json = true;

    if json {
        let serialized = serde_json::to_string(&presentation_data).unwrap();

        pb.set_position(2);
        pb.set_message("Writing JSON Data...");

        out_file.write(serialized.as_bytes()).unwrap();
    }
    else {
        let serialized = bincode::serialize(&presentation_data).unwrap();

        pb.set_position(2);
        pb.set_message("Writing Bytecode Data...");

        out_file.write_all(&serialized).unwrap();
    }

    pb.finish_and_clear();

    println!("Finished");

    Ok(())
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
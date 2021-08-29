pub mod single_read;
pub mod split_read;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::per_reference::single_read::SingleReadPerReferencePresentationData;
use crate::statistics::calculation::per_reference::PerReferenceCalculationData;
use crate::statistics::presentation::per_reference::split_read::SplitReadPerReferencePresentationData;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

#[derive(Debug, Deserialize, Serialize)]
pub struct PerReferencePresentationData {
    reference_name: String,
    reference_length: u32,
    single_read_data: SingleReadPerReferencePresentationData,
    split_read_data: SplitReadPerReferencePresentationData,
    covered_length: u32
}

impl PerReferencePresentationData {
    pub fn get_reference_name(&self) -> String {
        self.reference_name.clone()
    }

    pub fn get_reference_length(&self) -> u32 {
        self.reference_length
    }

    pub fn get_single_read_data(&self) -> &SingleReadPerReferencePresentationData {
        &self.single_read_data
    }

    pub fn get_split_read_data(&self) -> &SplitReadPerReferencePresentationData {
        &self.split_read_data
    }

    pub fn get_covered_length(&self) -> u32 {
        self.covered_length
    }

    pub fn get_covered_percentage(&self) -> f64 {
        self.covered_length as f64 / self.reference_length as f64
    }

    pub fn get_read_length_map(&self) -> PresentationFrequencyMap<u32> {
        PresentationFrequencyMap::merge(
            self.single_read_data.get_read_length_map(),
            self.split_read_data.get_read_length_map()
        )
    }

    pub fn calculate_from_data(value: PerReferenceCalculationData, mpb: &MultiProgress) -> PerReferencePresentationData {
        let pb = mpb.add(ProgressBar::new_spinner());

        pb.set_message("Calculating Reference Statistics...");
        pb.set_prefix("[1/4]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}         {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));
        pb.enable_steady_tick(60 / 15);

        let reference_name = value.reference_name;
        let reference_length = value.reference_length;

        pb.reset_elapsed();
        pb.reset_eta();

        pb.set_message("Calculating Reference Single Read Statistics...");
        pb.set_prefix("[2/4]");

        let single_read_data = value.single_read_data.into();

        pb.reset_eta();
        pb.reset_elapsed();

        pb.set_message("Calculating Reference Split Read Statistics...");
        pb.set_prefix("[3/4]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}         {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));

        let split_read_data = SplitReadPerReferencePresentationData::from_calculation_data(value.split_read_data, value.reference_length, mpb);

        pb.set_message("Calculating Reference Coverage Statistics...");
        pb.set_prefix("[4/4]");

        pb.reset_elapsed();
        pb.reset_eta();

        let coverage_lock = value.coverage_map.into_inner().unwrap();
        let coverage_map = coverage_lock.combine();
        let covered_length = coverage_map.get_total_covered_length();

        pb.finish_with_message("Completed, waiting...");

        Self {
            reference_name,
            reference_length,
            single_read_data,
            split_read_data,
            covered_length
        }
    }
}
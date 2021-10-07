pub mod single_read;
pub mod split_read;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::per_reference::single_read::SingleReadPerReferencePresentationData;
use crate::statistics::calculation::per_reference::PerReferenceCalculationData;
use crate::statistics::presentation::per_reference::split_read::SplitReadPerReferencePresentationData;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use crate::statistics::presentation::binned::map::BinnedStatisticsPresentationMap;
use crate::statistics::presentation::cigar_operations::CigarOperations;
use crate::util::get_quality_frequency_map;

#[derive(Debug, Deserialize, Serialize)]
pub struct PerReferencePresentationData {
    reference_name: String,
    reference_length: u32,
    single_read_data: SingleReadPerReferencePresentationData,
    split_read_data: SplitReadPerReferencePresentationData
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

    pub fn get_read_length_on_reference_map(&self) -> PresentationFrequencyMap<u32> {
        PresentationFrequencyMap::merge(
            self.single_read_data.get_read_length_on_reference_map(),
            self.split_read_data.get_read_length_on_reference_map()
        )
    }

    pub fn get_read_length_sequence_map(&self) -> PresentationFrequencyMap<u32> {
        PresentationFrequencyMap::merge(
            self.single_read_data.get_read_length_sequence_map(),
            self.split_read_data.get_read_length_sequence_map()
        )
    }

    pub fn get_quality_frequency(&self) -> PresentationFrequencyMap<u8> {
        PresentationFrequencyMap::merge(
            self.single_read_data.get_quality_frequency(),
            self.split_read_data.get_quality_frequency()
        )
    }
    pub fn get_quality_frequency_map(&self) -> Vec<(u8, u64)> {
        get_quality_frequency_map(&self.get_quality_frequency())
    }


    pub fn get_binned_statistics(&self) -> BinnedStatisticsPresentationMap {
        BinnedStatisticsPresentationMap::merge(
            self.single_read_data.get_binned_statistics(),
            self.split_read_data.get_binned_statistics()
        ).unwrap()
    }

    pub fn get_cigar_operations(&self) -> CigarOperations {
        CigarOperations::merge(
            &self.single_read_data.get_cigar_operations(),
            &self.split_read_data.get_cigar_operations()
        )
    }

    pub fn calculate_from_data(value: PerReferenceCalculationData, mpb: &MultiProgress) -> Result<PerReferencePresentationData, ()> {
        let pb = mpb.add(ProgressBar::new_spinner());

        pb.set_message("Calculating Reference Statistics...");
        pb.set_prefix("[1/3]");
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
        pb.set_prefix("[2/3]");

        let single_read_data = value.single_read_data.into();

        pb.reset_eta();
        pb.reset_elapsed();

        pb.set_message("Calculating Reference Split Read Statistics...");
        pb.set_prefix("[3/3]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}         {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));

        let split_read_data = SplitReadPerReferencePresentationData::from_calculation_data(value.split_read_data, value.reference_length, mpb)?;

        pb.finish_with_message("Completed, waiting...");

        Ok(Self {
            reference_name,
            reference_length,
            single_read_data,
            split_read_data
        })
    }
}
use serde_derive::{Deserialize, Serialize};

use single_read::UnmappedSingleReadPresentationData;

use std::convert::TryFrom;
use crate::statistics::calculation::unmapped::UnmappedCalculationData;
use indicatif::{ProgressBar, ProgressStyle};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;

pub mod single_read;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmappedPresentationData {
    single_read: UnmappedSingleReadPresentationData,
    split_read: UnmappedSingleReadPresentationData
}

impl UnmappedPresentationData {
    pub fn get_read_length_map(&self) -> PresentationFrequencyMap<u32> {
        PresentationFrequencyMap::merge(
            self.single_read.get_read_length_map(),
            self.split_read.get_read_length_map()
        )
    }

    pub fn get_single_read(&self) -> &UnmappedSingleReadPresentationData {
        &self.single_read
    }

    pub fn get_split_read(&self) -> &UnmappedSingleReadPresentationData {
        &self.split_read
    }
}

impl TryFrom<UnmappedCalculationData> for UnmappedPresentationData {
    type Error = ();

    fn try_from(calculation: UnmappedCalculationData) -> Result<Self, Self::Error> {
        let pb = ProgressBar::new_spinner();
        pb.set_message("Calculating Unmapped Single Read Statistics...");
        pb.set_prefix("[1/2]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}         {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));
        pb.enable_steady_tick(60/15);

        let single_read = calculation.single_read.into();

        pb.reset_eta();
        pb.reset_elapsed();

        pb.set_message("Calculating Unmapped Split Read Statistics...");
        pb.set_prefix("[2/2]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}         {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));
        pb.enable_steady_tick(60/15);

        let split_read = calculation.split_read.into();

        pb.finish_and_clear();

        Ok(Self {
            single_read,
            split_read
        })
    }
}
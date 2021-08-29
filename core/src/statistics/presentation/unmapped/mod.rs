use serde_derive::{Deserialize, Serialize};

use single_read::UnmappedSingleReadPresentationData;

use crate::statistics::presentation::unmapped::split_read::UnmappedSplitReadPresentationData;
use std::convert::{TryFrom, TryInto};
use crate::statistics::calculation::unmapped::UnmappedCalculationData;
use indicatif::{ProgressBar, ProgressStyle};

pub mod single_read;
pub mod split_read;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmappedPresentationData {
    single_read: UnmappedSingleReadPresentationData,
    split_read: UnmappedSplitReadPresentationData
}

impl UnmappedPresentationData {
    pub fn get_single_read(&self) -> &UnmappedSingleReadPresentationData {
        &self.single_read
    }

    pub fn get_split_read(&self) -> &UnmappedSplitReadPresentationData {
        &self.split_read
    }
}

impl AsRef<UnmappedSingleReadPresentationData> for UnmappedPresentationData {
    fn as_ref(&self) -> &UnmappedSingleReadPresentationData {
        self.get_single_read()
    }
}

impl AsRef<UnmappedSplitReadPresentationData> for UnmappedPresentationData {
    fn as_ref(&self) -> &UnmappedSplitReadPresentationData {
        self.get_split_read()
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

        let split_read = calculation.split_read.try_into()?;

        pb.finish_and_clear();

        Ok(Self {
            single_read,
            split_read
        })
    }
}
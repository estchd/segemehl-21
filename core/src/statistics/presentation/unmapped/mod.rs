use serde_derive::{Deserialize, Serialize};

use single_read::UnmappedSingleReadPresentationData;

use crate::statistics::calculation::unmapped::UnmappedSingleReadCalculationData;
use crate::statistics::presentation::unmapped::split_read::UnmappedSplitReadPresentationData;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use std::convert::{TryFrom, TryInto};

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

impl TryFrom<(UnmappedSingleReadCalculationData, CalculationAssemblerMap)> for UnmappedPresentationData {
    type Error = ();

    fn try_from(calculation: (UnmappedSingleReadCalculationData, CalculationAssemblerMap)) -> Result<Self, Self::Error> {
        let single_read = calculation.0.into();
        let split_read = calculation.1.try_into()?;

        Ok(Self {
            single_read,
            split_read
        })
    }
}
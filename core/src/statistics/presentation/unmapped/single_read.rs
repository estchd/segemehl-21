use crate::statistics::calculation::unmapped::UnmappedSingleReadCalculationData;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmappedSingleReadPresentationData {
    read_length_map: PresentationFrequencyMap<u32>
}

impl UnmappedSingleReadPresentationData {
    pub fn get_read_length_map(&self) -> &PresentationFrequencyMap<u32> {
        &self.read_length_map
    }
}

impl AsRef<PresentationFrequencyMap<u32>> for UnmappedSingleReadPresentationData {
    fn as_ref(&self) -> &PresentationFrequencyMap<u32> {
        self.get_read_length_map()
    }
}

impl From<UnmappedSingleReadCalculationData> for UnmappedSingleReadPresentationData {
    fn from(value: UnmappedSingleReadCalculationData) -> Self {
        let read_length_map = value.read_length_map.into();

        Self {
            read_length_map
        }
    }
}

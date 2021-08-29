use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::binned::map::BinnedStatisticsPresentationMap;
use crate::statistics::calculation::per_reference::single_read::SingleReadPerReferenceCalculationData;
use crate::util::get_quality_frequency_map;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SingleReadPerReferencePresentationData {
    quality_map: PresentationFrequencyMap<u8>,
    read_length_map: PresentationFrequencyMap<u32>,
    binned_statistics: BinnedStatisticsPresentationMap
}

impl SingleReadPerReferencePresentationData {
    pub fn get_quality_frequency(&self) -> &PresentationFrequencyMap<u8> {
        &self.quality_map
    }

    pub fn get_quality_frequency_map(&self) -> Vec<(u8, u64)> {
        get_quality_frequency_map(&self.quality_map)
    }

    pub fn get_read_length_map(&self) -> &PresentationFrequencyMap<u32> {
        &self.read_length_map
    }

    pub fn get_binned_statistics(&self) -> &BinnedStatisticsPresentationMap {
        &self.binned_statistics
    }
}

impl From<SingleReadPerReferenceCalculationData> for SingleReadPerReferencePresentationData {
    fn from(value: SingleReadPerReferenceCalculationData) -> Self {
        let quality_map = value.quality_map.into();
        let read_length_map = value.read_length_map.into();
        let binned_statistics = value.binned_statistics.into();

        Self {
            quality_map,
            read_length_map,
            binned_statistics
        }
    }
}
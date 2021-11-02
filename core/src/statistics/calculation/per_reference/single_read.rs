use bam::Record;
use thiserror::Error;

use crate::header::reference_sequence_line::ReferenceSequence;
use crate::statistics::calculation::binned::BinConfig;
use crate::statistics::calculation::binned::map::{BinnedStatisticsCalculationMap, BinnedStatisticsCalculationMapNewError};
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::util::{get_record_mapping_quality, get_record_length_on_reference, get_record_length_sequence};

#[derive(Error, Debug)]
pub enum SingleReadPerReferenceCalculationNewError {
    #[error("could not create binned statistics")]
    CalculationMap {
        source: BinnedStatisticsCalculationMapNewError
    }
}

#[derive(Debug)]
pub struct SingleReadPerReferenceCalculationData {
    pub(crate) quality_map: CalculationFrequencyMap<u8>,
    pub(crate) read_length_on_reference_map: CalculationFrequencyMap<u32>,
    pub(crate) read_length_sequence_map: CalculationFrequencyMap<u32>,
    pub(crate) binned_statistics: BinnedStatisticsCalculationMap
}

impl SingleReadPerReferenceCalculationData {
    pub fn new(ref_line: &ReferenceSequence, bin_config: BinConfig) -> Result<Self, SingleReadPerReferenceCalculationNewError> {
        let quality_map = CalculationFrequencyMap::new();
        let read_length_on_reference_map = CalculationFrequencyMap::new();
        let read_length_sequence_map = CalculationFrequencyMap::new();
        let binned_statistics = BinnedStatisticsCalculationMap::new(0, ref_line.length, bin_config)
            .map_err(|source|{
                SingleReadPerReferenceCalculationNewError::CalculationMap {
                    source
                }
            })?;

        Ok(Self {
            quality_map,
            read_length_on_reference_map,
            read_length_sequence_map,
            binned_statistics
        })
    }

    pub fn add_record(&self, record: Record) {
        let quality = get_record_mapping_quality(&record);
        let sequence_length = get_record_length_sequence(&record);
        let on_reference_length = get_record_length_on_reference(&record);

        self.quality_map.add_entry(quality);
        self.read_length_on_reference_map.add_entry(on_reference_length);
        self.read_length_sequence_map.add_entry(sequence_length);
        self.binned_statistics.add_record(&record);
    }
}
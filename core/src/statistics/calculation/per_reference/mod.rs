pub mod single_read;

use crate::statistics::calculation::per_reference::single_read::{SingleReadPerReferenceCalculationData, SingleReadPerReferenceCalculationNewError};
use crate::header::reference_sequence_line::ReferenceSequence;
use bam::Record;
use thiserror::Error;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::util::{get_record_length_on_reference};
use crate::statistics::calculation::binned::BinConfig;

#[derive(Error, Debug)]
pub enum PerReferenceCalculationNewError {
    #[error("could not create split read calculation data")]
    SplitRead {
        source: SingleReadPerReferenceCalculationNewError
    },
    #[error("could not create single read calculation data")]
    SingleRead {
        source: SingleReadPerReferenceCalculationNewError
    }
}

#[derive(Debug)]
pub struct PerReferenceCalculationData {
    pub(crate) reference_name: String,
    pub(crate) reference_length: u32,
    pub(crate) read_length_map: CalculationFrequencyMap<u32>,
    pub(crate) single_read_data: SingleReadPerReferenceCalculationData,
    pub(crate) split_read_data: SingleReadPerReferenceCalculationData
}

impl PerReferenceCalculationData {
    pub fn new(ref_line: &ReferenceSequence, bin_config: BinConfig) -> Result<Self, PerReferenceCalculationNewError> {
        let reference_name = ref_line.name.clone();
        let reference_length = ref_line.length;
        let read_length_map = CalculationFrequencyMap::new();
        let single_read_data = SingleReadPerReferenceCalculationData::new(ref_line, bin_config)
            .map_err(|source|  {
                PerReferenceCalculationNewError::SingleRead {
                    source
                }
            })?;
        let split_read_data = SingleReadPerReferenceCalculationData::new(ref_line, bin_config)
            .map_err(|source| {
                PerReferenceCalculationNewError::SplitRead {
                    source
                }
            })?;
        
        Ok(Self {
            reference_name,
            reference_length,
            read_length_map,
            single_read_data,
            split_read_data
        })
    }

    pub fn add_record(&self, record: Record) {
        let read_length = get_record_length_on_reference(&record);

        self.read_length_map.add_entry(read_length);

        let split = record.flag().is_paired();
        return if split {
            self.split_read_data.add_record(record);
        }
        else {
            self.single_read_data.add_record(record);
        }
    }
}
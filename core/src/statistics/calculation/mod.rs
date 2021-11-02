use std::num::{NonZeroU32};
use bam::Record;
use thiserror::Error;
use per_reference::PerReferenceCalculationData;

use crate::header::Header;
use binned::BinConfig;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::calculation::per_reference::PerReferenceCalculationNewError;
use crate::statistics::calculation::unmapped::UnmappedCalculationData;
use crate::statistics::shared::meta::Meta;

pub mod assembler;
pub mod unmapped;
pub mod per_reference;
pub mod binned;
pub mod frequency_map;

#[derive(Error, Debug)]
pub enum CalculationDataNewError {
    #[error("could not create per reference calculation data for reference: {reference_name}")]
    PerReferenceError {
        reference_name: String,
        source: PerReferenceCalculationNewError
    }
}

#[derive(Error, Debug)]
pub enum CalculationAddRecordError {
    #[error("could not add mapped record")]
    AddMappedError {
        source: CalculationAddMappedRecordError
    }
}

#[derive(Error, Debug)]
pub enum CalculationAddMappedRecordError {
    #[error("tried to add an unmapped record as a mapped record")]
    NegativeRefID,
    #[error("tried to add a mapped record to a reference that doesnt exist")]
    InvalidRefID
}

#[derive(Debug)]
pub struct CalculationData {
    pub(crate) unmapped: UnmappedCalculationData,
    pub(crate) split_read: CalculationAssemblerMap,
    pub(crate) per_reference: Vec<PerReferenceCalculationData>,
    pub(crate) meta: Meta
}

impl CalculationData {
    pub fn new(header: &Header, bin_size: NonZeroU32) -> Result<Self, CalculationDataNewError> {
        let unmapped = UnmappedCalculationData::new();
        let split_read = CalculationAssemblerMap::new();
        let mut per_reference = Vec::new();

        for ref_sequence in header.reference_sequences.iter() {
            let per_reference_data = PerReferenceCalculationData::new(ref_sequence, BinConfig::LengthOfBins(bin_size))
                .map_err(|source| {
                    CalculationDataNewError::PerReferenceError {
                        reference_name: ref_sequence.name.clone(),
                        source
                    }
                })?;
            per_reference.push(per_reference_data);
        }

        Ok(Self {
            unmapped,
            split_read,
            per_reference,
            meta: Meta {
                bin_size
            }
        })
    }

    pub fn add_record(&self, record: Record) -> Result<(),CalculationAddRecordError> {
        let is_mapped = record.flag().is_mapped();
        let is_split = record.flag().is_paired();

        if is_split {
            self.split_read.add_record(record.clone());
        }

        if is_mapped {
            self.add_mapped_record(record).map_err(|source|
                CalculationAddRecordError::AddMappedError {
                    source
                }
            )
        }
        else {
            self.unmapped.add_record(record);
            Ok(())
        }
    }

    fn add_mapped_record(&self, record: Record) -> Result<(),CalculationAddMappedRecordError> {
        let ref_id = record.ref_id();

        if ref_id < 0 {
            return Err(CalculationAddMappedRecordError::NegativeRefID);
        }

        let ref_id = ref_id as usize;

        if ref_id >= self.per_reference.len() {
            return Err(CalculationAddMappedRecordError::InvalidRefID);
        }

        let per_reference = &self.per_reference[ref_id as usize];

        per_reference.add_record(record);
        Ok(())
    }
}
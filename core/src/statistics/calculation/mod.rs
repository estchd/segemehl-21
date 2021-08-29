use bam::Record;

use per_reference::PerReferenceCalculationData;

use crate::header::header::Header;
use binned::BinConfig;
use crate::statistics::calculation::unmapped::UnmappedCalculationData;

pub mod assembler;
pub mod unmapped;
pub mod per_reference;
pub mod coverage_map;
pub mod binned;
pub mod frequency_map;

#[derive(Debug)]
pub struct CalculationData {
    pub(crate) unmapped: UnmappedCalculationData,
    pub(crate) per_reference: Vec<PerReferenceCalculationData>
}

impl CalculationData {
    pub fn new(header: &Header, bin_config: BinConfig) -> Result<Self, ()> {
        let unmapped = UnmappedCalculationData::new();
        let mut per_reference = Vec::new();

        for ref_sequence in header.reference_sequences.iter() {
            let per_reference_data = PerReferenceCalculationData::new(ref_sequence, bin_config)?;
            per_reference.push(per_reference_data);
        }

        Ok(Self {
            unmapped,
            per_reference
        })
    }

    pub fn add_record(&self, record: Record) -> Result<(),()> {
        let is_mapped = record.flag().is_mapped();

        if is_mapped {
            self.add_mapped_record(record)
        }
        else {
            self.unmapped.add_record(record)
        }
    }

    fn add_mapped_record(&self, record: Record) -> Result<(),()> {
        let ref_id = record.ref_id();

        if ref_id < 0 {
            return Err(());
        }

        let ref_id = ref_id as usize;

        if ref_id >= self.per_reference.len() {
            return Err(());
        }

        let per_reference = &self.per_reference[ref_id as usize];

        per_reference.add_record(record)
    }
}
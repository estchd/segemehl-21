pub mod single_read;

use std::sync::Mutex;
use crate::statistics::calculation::per_reference::single_read::SingleReadPerReferenceCalculationData;
use crate::header::reference_sequence_line::reference_sequence::ReferenceSequence;
use bam::Record;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::util::{get_record_length, get_record_start, get_record_end};
use crate::statistics::calculation::binned::BinConfig;
use crate::statistics::calculation::coverage_map::split_coverage_map::SplitCoverageMap;
use std::num::{NonZeroU32, NonZeroUsize};

#[derive(Debug)]
pub struct PerReferenceCalculationData {
    pub(crate) reference_name: String,
    pub(crate) reference_length: u32,
    pub(crate) read_length_map: CalculationFrequencyMap<u32>,
    pub(crate) single_read_data: SingleReadPerReferenceCalculationData,
    pub(crate) split_read_assembler_map: CalculationAssemblerMap,
    pub(crate) coverage_map: Mutex<SplitCoverageMap>
}

impl PerReferenceCalculationData {
    pub fn new(ref_line: &ReferenceSequence, bin_config: BinConfig) -> Result<Self, ()> {
        let reference_name = ref_line.name.clone();
        let reference_length = ref_line.length;
        let read_length_map = CalculationFrequencyMap::new();
        let single_read_data = SingleReadPerReferenceCalculationData::new(ref_line, bin_config)?;
        let split_read_assembler_map = CalculationAssemblerMap::new();
        
        Ok(Self {
            reference_name,
            reference_length,
            read_length_map,
            single_read_data,
            split_read_assembler_map,
            coverage_map: Mutex::new(SplitCoverageMap::new(0, NonZeroU32::new(ref_line.length).unwrap(), NonZeroUsize::new(1000).unwrap()))
        })
    }

    pub fn add_record(&self, record: Record) -> Result<(),()> {
        let read_length = get_record_length(&record);

        self.read_length_map.add_entry(read_length);

        let mut coverage_lock = self.coverage_map.lock().unwrap();
        coverage_lock.add_coverage(get_record_start(&record), get_record_end(&record));

        let split = record.flag().is_paired();
        if split {
            self.split_read_assembler_map.add_record(record)
        }
        else {
            self.single_read_data.add_record(record)
        }
    }
}
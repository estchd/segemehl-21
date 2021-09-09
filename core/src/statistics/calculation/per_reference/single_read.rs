use bam::Record;

use crate::header::reference_sequence_line::reference_sequence::ReferenceSequence;
use crate::statistics::calculation::binned::BinConfig;
use crate::statistics::calculation::binned::map::BinnedStatisticsCalculationMap;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::util::{get_record_mapping_quality, get_record_length};

#[derive(Debug)]
pub struct SingleReadPerReferenceCalculationData {
    pub(crate) quality_map: CalculationFrequencyMap<u8>,
    pub(crate) read_length_map: CalculationFrequencyMap<u32>,
    pub(crate) binned_statistics: BinnedStatisticsCalculationMap
}

impl SingleReadPerReferenceCalculationData {
    pub fn new(ref_line: &ReferenceSequence, bin_config: BinConfig) -> Result<Self, ()> {
        let quality_map = CalculationFrequencyMap::new();
        let read_length_map = CalculationFrequencyMap::new();
        let binned_statistics = BinnedStatisticsCalculationMap::new(0, ref_line.length, bin_config)?;

        Ok(Self {
            quality_map,
            read_length_map,
            binned_statistics
        })
    }

    pub fn add_record(&self, record: Record) {
        let quality = get_record_mapping_quality(&record);
        let length = get_record_length(&record);

        self.quality_map.add_entry(quality);
        self.read_length_map.add_entry(length);
        self.binned_statistics.add_record(&record);
    }
}
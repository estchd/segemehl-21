use bam::Record;

use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::util::get_record_length;

#[derive(Debug)]
pub struct UnmappedSingleReadCalculationData {
    pub(crate) read_length_map: CalculationFrequencyMap<u32>,
}

impl UnmappedSingleReadCalculationData {
    pub fn new() -> Self {
        Self {
            read_length_map: CalculationFrequencyMap::new()
        }
    }

    pub fn add_record(&self, record: Record) -> Result<(),()> {
        let read_length = get_record_length(&record);
        self.read_length_map.add_entry(read_length);

        Ok(())
    }
}
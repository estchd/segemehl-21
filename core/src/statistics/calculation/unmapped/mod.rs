pub mod single_read;

use bam::Record;
use crate::statistics::calculation::unmapped::single_read::UnmappedSingleReadCalculationData;

#[derive(Debug)]
pub struct UnmappedCalculationData {
	pub(crate) single_read: UnmappedSingleReadCalculationData,
	pub(crate) split_read: UnmappedSingleReadCalculationData
}

impl UnmappedCalculationData {
	pub fn new() -> Self {
		Self {
			single_read: UnmappedSingleReadCalculationData::new(),
			split_read: UnmappedSingleReadCalculationData::new()
		}
	}

	pub fn add_record(&self, record: Record) {
		if record.flag().is_paired() {
			self.split_read.add_record(record);
		}
		else {
			self.single_read.add_record(record);
		}
	}
}
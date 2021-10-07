pub mod split_read;
pub mod single_read;

use bam::Record;
use crate::statistics::calculation::unmapped::split_read::UnmappedSplitReadCalculationData;
use crate::statistics::calculation::unmapped::single_read::UnmappedSingleReadCalculationData;

#[derive(Debug)]
pub struct UnmappedCalculationData {
	pub(crate) single_read: UnmappedSingleReadCalculationData,
	pub(crate) split_read: UnmappedSplitReadCalculationData
}

impl UnmappedCalculationData {
	pub fn new() -> Self {
		Self {
			single_read: UnmappedSingleReadCalculationData::new(),
			split_read: UnmappedSplitReadCalculationData::new()
		}
	}

	pub fn add_record(&self, record: Record) -> Result<(),()> {
		let is_split = record.flag().is_paired();

		if is_split {
			self.split_read.add_record(record);
			Ok(())
		}
		else {
			self.single_read.add_record(record)
		}
	}
}
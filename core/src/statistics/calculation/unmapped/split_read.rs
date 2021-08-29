use bam::Record;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;

#[derive(Debug)]
pub struct UnmappedSplitReadCalculationData {
	pub(crate) assembler: CalculationAssemblerMap
}

impl UnmappedSplitReadCalculationData {
	pub fn new() -> Self {
		Self {
			assembler: CalculationAssemblerMap::new()
		}
	}

	pub fn add_record(&self, record: Record) -> Result<(),()> {
		self.assembler.add_record(record)
	}
}
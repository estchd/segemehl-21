use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use crate::statistics::presentation::split_read::SplitRead;

pub struct SplitReadCollection {
	split_reads: Vec<SplitRead>
}

impl SplitReadCollection {
	pub fn inner(&self) -> &Vec<SplitRead> {
		&self.split_reads
	}

	pub fn into_inner(self) -> Vec<SplitRead> {
		self.split_reads
	}
}

impl From<PresentationAssemblerCollection> for SplitReadCollection {
	fn from(value: PresentationAssemblerCollection) -> Self {
		let assemblers = value.into_inner();

		let split_reads = assemblers.into_iter().map(|item| item.into()).collect();

		SplitReadCollection {
			split_reads
		}
	}
}
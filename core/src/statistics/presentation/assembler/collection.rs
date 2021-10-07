use std::convert::TryFrom;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::statistics::presentation::assembler::PresentationAssembler;
use crate::statistics::presentation::assembler::presentation_record_collection::PresentationRecordCollection;


pub struct PresentationAssemblerCollection {
    assemblers: Vec<PresentationAssembler>
}

impl PresentationAssemblerCollection {
	pub fn inner(&self) -> &Vec<PresentationAssembler> {
		&self.assemblers
	}

	pub fn into_inner(self) -> Vec<PresentationAssembler> {
		self.assemblers
	}
}

impl TryFrom<PresentationRecordCollection> for PresentationAssemblerCollection {
	type Error = ();

	fn try_from(value: PresentationRecordCollection) -> Result<Self, Self::Error> {
		let PresentationRecordCollection {
			map,
			starts
		} = value;

		let assemblers: Result<Vec<PresentationAssembler>, ()> = starts
			.into_par_iter()
			.map(|item| {
				PresentationAssembler::try_from_start_record_with_map(item, &map)
			})
			.collect();

		assemblers.map(|item| PresentationAssemblerCollection {
			assemblers: item
		})
	}
}
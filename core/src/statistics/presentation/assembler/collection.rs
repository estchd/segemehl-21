use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::presentation::assembler::PresentationAssembler;
use crate::statistics::presentation::record::PresentationRecord;

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

impl From<CalculationAssemblerMap> for PresentationAssemblerCollection {
	fn from(value: CalculationAssemblerMap) -> Self {
		let map = value.map.into_inner().unwrap();

		let assemblers = map.into_par_iter()
			.map(|(_, value)| {
				let values = value.into_inner().unwrap();
				let values: Vec<PresentationRecord> = values.into_par_iter().map(|item| item.into()).collect();

				values.into()
			})
			.collect();

		Self {
			assemblers
		}
	}
}
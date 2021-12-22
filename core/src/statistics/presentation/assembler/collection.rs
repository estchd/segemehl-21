use std::collections::HashMap;
use std::sync::Mutex;
use bam::Record;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::presentation::assembler::PresentationAssembler;
use crate::statistics::presentation::record::PresentationRecord;

pub struct PresentationAssemblerCollection {
    pub(crate) normals: Vec<PresentationAssembler>,
	pub(crate) supplementaries: Vec<PresentationAssembler>,
	pub(crate) secondaries: Vec<PresentationAssembler>,
	pub(crate) duplicates: Vec<PresentationAssembler>
}

impl PresentationAssemblerCollection {
	pub fn normals(&self) -> &Vec<PresentationAssembler> {
		&self.normals
	}

	pub fn supplementaries(&self) -> &Vec<PresentationAssembler> {
		&self.supplementaries
	}

	pub fn secondaries(&self) -> &Vec<PresentationAssembler> {
		&self.secondaries
	}

	pub fn duplicates(&self) -> &Vec<PresentationAssembler> {
		&self.duplicates
	}

	fn unwrap_mutex_map(mutex_map: HashMap<u32, Mutex<Vec<Record>>>) -> Vec<PresentationAssembler> {
		mutex_map.into_par_iter()
			.map(|(_, value)| {
				let values = value.into_inner().unwrap();
				let values: Vec<PresentationRecord> = values.into_par_iter().map(|item| item.into()).collect();

				values.into()
			})
			.collect()
	}
}

impl From<CalculationAssemblerMap> for PresentationAssemblerCollection {
	fn from(value: CalculationAssemblerMap) -> Self {
		let normals = value.normals.into_inner().unwrap();
		let supplementaries = value.supplementaries.into_inner().unwrap();
		let secondaries = value.secondaries.into_inner().unwrap();
		let duplicates = value.duplicates.into_inner().unwrap();

		let normals = Self::unwrap_mutex_map(normals);

		let supplementaries = Self::unwrap_mutex_map(supplementaries);

		let secondaries = Self::unwrap_mutex_map(secondaries);

		let duplicates = Self::unwrap_mutex_map(duplicates);

		Self {
			normals,
			supplementaries,
			secondaries,
			duplicates
		}
	}
}
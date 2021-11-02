use std::convert::TryFrom;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use thiserror::Error;
use crate::statistics::presentation::assembler::{PresentationAssembler, PresentationAssemblerTryFromError};
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

#[derive(Error, Debug)]
pub enum PresentationAssemblerCollectionTryFromError {
	#[error("could not assemble presentation record vector")]
	PresentationAssembler {
		source: PresentationAssemblerTryFromError
	}
}

impl TryFrom<PresentationRecordCollection> for PresentationAssemblerCollection {
	type Error = PresentationAssemblerCollectionTryFromError;

	fn try_from(value: PresentationRecordCollection) -> Result<Self, Self::Error> {
		let PresentationRecordCollection {
			map
		} = value;

		let assemblers: Vec<PresentationAssembler> = map
			.into_par_iter()
			.map(|(_,item)| {
				PresentationAssembler::try_from(item)
			})
			.collect::<Result<Vec<PresentationAssembler>, PresentationAssemblerTryFromError>>()
			.map_err(|source| {
				PresentationAssemblerCollectionTryFromError::PresentationAssembler {
					source
				}
			})?;

		Ok(PresentationAssemblerCollection {
			assemblers
		})
	}
}
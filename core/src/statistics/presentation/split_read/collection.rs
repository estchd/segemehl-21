use std::convert::{TryFrom, TryInto};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde_derive::{Serialize, Deserialize};
use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use crate::statistics::presentation::assembler::PresentationAssembler;
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::presentation::split_read::SplitRead;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

	pub fn combine(mut a: SplitReadCollection, mut b: SplitReadCollection) -> SplitReadCollection {
		a.split_reads.append(&mut b.split_reads);

		Self {
			split_reads: a.split_reads
		}
	}
}

impl TryFrom<PresentationAssembler> for SplitReadCollection {
	type Error = ();

	fn try_from(value: PresentationAssembler) -> Result<Self, Self::Error> {
		let PresentationAssembler {
			template_length_map
		} = value;

		let mut split_reads: Vec<SplitRead> = vec![];

		for template_length in template_length_map {
			let mut start: Option<&PresentationRecord> = None;
			let mut end: Option<&PresentationRecord> = None;

			let (template_length, associated_records) = template_length;

			for record in &associated_records {
				if record.get_flags().get_is_first_mate() {
					match start {
						None => { start = Some(record) }
						Some(_) => {
							println!("Multiple Start Records with template name {}, template length {}", record.get_name(), template_length);
							return Err(())
						}
					}
				}
				if record.get_flags().get_is_last_mate() {
					match end {
						None => { end = Some(record)}
						Some(_) => {
							println!("Multiple End Records with template name {}, template length {}", record.get_name(), template_length);
							return Err(())
						}
					}
				}
			}

			split_reads.push(SplitRead::from(associated_records));
		}

		Ok(Self {
			split_reads
		})
	}
}

impl TryFrom<PresentationAssemblerCollection> for SplitReadCollection {
	type Error = ();

	fn try_from(value: PresentationAssemblerCollection) -> Result<Self, Self::Error> {
		let assemblers = value.into_inner();

		let collections = assemblers.into_iter().par_bridge().fold(
			|| {
				Ok(Self {
					split_reads: vec![]
				})
			},
			|a,b| {
				let a = a?;
				let b = b.try_into()?;
				Ok(Self::combine(a,b))
			}
		).collect::<Result<Vec<SplitReadCollection>, ()>>()?;
		
		Ok(collections.into_iter().fold(
			Self {
				split_reads: vec![]
			},
			|a,b| {
				Self::combine(a,b)
			}
		))
	}
}
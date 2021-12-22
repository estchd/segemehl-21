use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::presentation::split_read::partial::PartialSplitRead;

pub struct PartialSplitReadMap {
	inner: HashMap<(i32, u32), Vec<PartialSplitRead>>
}

impl Deref for PartialSplitReadMap {
	type Target = HashMap<(i32, u32), Vec<PartialSplitRead>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for PartialSplitReadMap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl From<Vec<PresentationRecord>> for PartialSplitReadMap {
	fn from(value: Vec<PresentationRecord>) -> Self {
		let partial_split_reads: Vec<PartialSplitRead> = value.into_iter().map(|record| {
			PartialSplitRead::from_read(record)
		}).collect();

		let mut partial_split_read_map: HashMap<(i32, u32), Vec<PartialSplitRead>> = HashMap::new();

		for partial_split_read in partial_split_reads {
			let ref_id = partial_split_read.get_ref_id();
			let start = partial_split_read.get_start();

			if partial_split_read_map.contains_key(&(ref_id, start)) {
				let records = partial_split_read_map.get_mut(&(ref_id, start)).unwrap();
				records.push(partial_split_read);
			}
			else {
				partial_split_read_map.insert((ref_id, start), vec![partial_split_read]);
			}
		}

		Self {
			inner: partial_split_read_map
		}
	}
}
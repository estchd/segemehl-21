use std::convert::TryInto;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_derive::{Serialize, Deserialize};
use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use crate::statistics::presentation::assembler::PresentationAssembler;
use crate::statistics::presentation::split_read::partial::map::PartialSplitReadMap;
use crate::statistics::presentation::split_read::partial::PartialSplitRead;
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

	pub fn combine(mut a: SplitReadCollection, b: SplitReadCollection) -> SplitReadCollection {
		a.split_reads.extend(b.split_reads);

		Self {
			split_reads: a.split_reads
		}
	}
}

impl Into<(SplitReadCollection, usize)> for PresentationAssembler {
	fn into(self) -> (SplitReadCollection, usize) {
		let PresentationAssembler {
			associated_records
		} = self;

		let mut partial_split_read_map = PartialSplitReadMap::from(associated_records);

		let (
			completed_split_reads,
			dropped_reads
		) = merge_partial_split_read_map(&mut partial_split_read_map);

		let split_reads: Vec<SplitRead> = completed_split_reads.into_iter().map(|completed_split_read| {
			completed_split_read.try_into().unwrap()
		}).collect();

		(SplitReadCollection {
			split_reads
		}, dropped_reads)
	}
}

fn remove_complete_reads(partial_split_read_map: &mut PartialSplitReadMap) -> Vec<PartialSplitRead> {
	let mut complete_reads = vec![];

	loop {
		let next_complete_read = find_next_complete_read(partial_split_read_map);
		match next_complete_read {
			None => break,
			Some((key, i)) => {
				let mut vec = partial_split_read_map.remove(&key).unwrap();

				let read = vec.remove(i);

				if !vec.is_empty() {
					partial_split_read_map.insert(key, vec);
				}

				complete_reads.push(read);
			}
		}
	}

	complete_reads
}

fn find_next_complete_read(partial_split_read_map: &PartialSplitReadMap) -> Option<((i32, u32), usize)> {
	for (key, vec) in partial_split_read_map.iter() {
		for (i, read) in vec.iter().enumerate() {
			if read.is_complete() {
				return Some((*key, i));
			}
		}
	}
	None
}

fn merge_partial_split_read_map(partial_split_read_map: &mut PartialSplitReadMap) -> (Vec<PartialSplitRead>, usize) {
	let mut completed_split_reads: Vec<PartialSplitRead> = remove_complete_reads(partial_split_read_map);

	while !partial_split_read_map.is_empty() {
		let pair = get_next_pair(partial_split_read_map);

		match pair {
			Some((first_key, second_key)) => {
				let completed_read = merge_pair(first_key, second_key, partial_split_read_map);
				if let Some(completed) = completed_read {
					completed_split_reads.push(completed);
				}
			}
			None => {
				let (completed, dropped_reads) =
					handle_unmergeables(partial_split_read_map);
				completed_split_reads.extend(completed);
				return (completed_split_reads, dropped_reads);
			}
		}
	}

	return (completed_split_reads, 0);
}

fn merge_pair(first_key: (i32, u32, usize), second_key: (i32, u32), partial_split_read_map: &mut PartialSplitReadMap) -> Option<PartialSplitRead> {
	if (first_key.0, first_key.1) == second_key {
		merge_pair_at_same_position(first_key, partial_split_read_map)
	}
	else {
		merge_pair_at_different_positions(first_key, second_key, partial_split_read_map)
	}
}

fn merge_pair_at_same_position(key: (i32, u32, usize), partial_split_read_map: &mut PartialSplitReadMap) -> Option<PartialSplitRead> {
	let mut vec = partial_split_read_map.remove(&(key.0, key.1)).unwrap();
	let first = vec.remove(key.2);
	let second = vec.remove(0);

	let combined = PartialSplitRead::combine(first, second).unwrap();

	if combined.is_complete() {
		return Some(combined);
	}
	else {
		partial_split_read_map.insert((key.0, key.1), vec![combined]);
	}
	return None;
}

fn merge_pair_at_different_positions(first_key: (i32, u32, usize), second_key: (i32, u32), partial_split_read_map: &mut PartialSplitReadMap) -> Option<PartialSplitRead> {
	let mut first_vec = partial_split_read_map.remove(&(first_key.0, first_key.1)).unwrap();
	let first = first_vec.remove(first_key.2);
	let mut second_vec = partial_split_read_map.remove(&second_key).unwrap();
	let second = second_vec.remove(0);

	if !first_vec.is_empty() {
		partial_split_read_map.insert((first_key.0, first_key.1), first_vec);
	}

	let combined = PartialSplitRead::combine(first, second).unwrap();

	if combined.is_complete() {
		return Some(combined);
	}

	let start = combined.get_start();
	let ref_id = combined.get_ref_id();

	if partial_split_read_map.contains_key(&(ref_id, start)) {
		let records = partial_split_read_map.get_mut(&(ref_id, start)).unwrap();
		records.push(combined);
	} else {
		partial_split_read_map.insert((ref_id, start), vec![combined]);
	}

	None
}

fn handle_unmergeables(partial_split_read_map: &mut PartialSplitReadMap) -> (Vec<PartialSplitRead>, usize) {
	let mut count = 0;
	for (_, value) in partial_split_read_map.iter_mut() {
		count += value.len();
	}

	if let Ok(read) = brute_force_merge(partial_split_read_map) {
		return (vec![read], 0);
	}

	(vec![], count)
}

fn brute_force_merge(partial_split_read_map: &mut PartialSplitReadMap) -> Result<PartialSplitRead, usize> {
	let mut record = None;

	let mut combined_count = 0usize;

	while !partial_split_read_map.is_empty() {
		let first_key = *partial_split_read_map.keys().next().unwrap();
		let vec = partial_split_read_map.remove(&first_key).unwrap();

		let vec_len = vec.len();
		for (i, new_record) in vec.into_iter().enumerate() {
			match record {
				None => {
					record = Some(new_record);
					combined_count += 1;
				}
				Some(existing) => {
					let combined = PartialSplitRead::combine(existing, new_record);
					match combined {
						Ok(combined) => {
							record = Some(combined);
							combined_count += 1;
						}
						Err((_,_)) => {
							let remaining_record_count = count_remaining_records(partial_split_read_map);
							let remaining_vec_count = vec_len - i;
							return Err(combined_count + remaining_record_count + remaining_vec_count);
						}
					}
				}
			}
		}
	}
	let record = record.unwrap();

	return if record.is_complete() || record.is_next_unmapped() {
		Ok(record)
	}
	else {
		Err(combined_count)
	}

}

fn count_remaining_records(partial_split_read_map: &PartialSplitReadMap) -> usize {
	let mut count = 0;

	for (_, vec) in partial_split_read_map.iter() {
		count += vec.len();
	}

	count
}

fn get_next_pair(partial_split_read_map: &PartialSplitReadMap) -> Option<((i32, u32, usize), (i32, u32))> {
	let mut all_no_next = None;
	let mut all_missing_chain = None;

	for (key, value) in partial_split_read_map.iter() {
		for (i, read) in value.iter().enumerate() {
			let p_next = read.get_p_next();
			let r_next = read.get_r_next();

			if p_next == -1 || r_next == -1 {
				if let None = all_missing_chain {
					all_missing_chain = Some(true);
				}
				continue;
			}

			if (r_next, p_next as u32) == (key.0, key.1) {
				match value.len() {
					1 => {
						if let None = all_no_next {
							all_no_next = Some(true);
						}
						continue;
					}
					2 => {
						let pair = Some(((key.0, key.1, i), (key.0, key.1)));
						return pair;
					}
					_ => {
						continue;
					}
				}
			}

			all_missing_chain = Some(false);

			let next = partial_split_read_map.get(&(r_next, p_next as u32));

			let next = match next {
				None => {
					if let None = all_no_next {
						all_no_next = Some(true);
					}
					continue;
				}
				Some(next) => {
					all_no_next = Some(false);
					next
				}
			};

			if next.len() > 1 { continue; }

			let pair = Some(((key.0, key.1, i), (r_next, p_next as u32)));
			return pair;
		}
	}

	None
}

pub struct SplitReadCollections {
	pub(crate) normals: SplitReadCollection,
	pub(crate) supplementaries: SplitReadCollection,
	pub(crate) secondaries: SplitReadCollection,
	pub(crate) duplicates: SplitReadCollection
}

impl From<PresentationAssemblerCollection> for (SplitReadCollections, usize) {
	fn from(value: PresentationAssemblerCollection) -> Self {
		let PresentationAssemblerCollection {
			normals,
			supplementaries,
			secondaries,
			duplicates,
		} = value;

		let merged_normals = 
			presentation_assembler_collection_into_split_read_collection(normals);
		let merged_supplementaries = 
			presentation_assembler_collection_into_split_read_collection(supplementaries);
		let merged_secondaries = 
			presentation_assembler_collection_into_split_read_collection(secondaries);
		let merged_duplicates = 
			presentation_assembler_collection_into_split_read_collection(duplicates);

		let normals = merged_normals.0;
		let supplementaries = merged_supplementaries.0;
		let secondaries = merged_secondaries.0;
		let duplicates = merged_duplicates.0;

		let dropped_reads = merged_normals.1 + merged_supplementaries.1 + merged_secondaries.1 + merged_duplicates.1;

		(
			SplitReadCollections {
				normals,
				supplementaries,
				secondaries,
				duplicates
			},
			dropped_reads
		)
	}
}

fn presentation_assembler_collection_into_split_read_collection(value: Vec<PresentationAssembler>) -> (SplitReadCollection, usize) {
	let collections = value.into_par_iter().fold(
		|| {
			(SplitReadCollection {
				split_reads: vec![]
			}, 0usize)
		},
		|a,b| {
			let (a1, a2) = a;
			let (b1, b2): (SplitReadCollection, usize) = b.into();
			(SplitReadCollection::combine(a1,b1), a2 + b2)
		}
	).collect::<Vec<(SplitReadCollection, usize)>>();

	collections.into_iter().fold(
		(SplitReadCollection {
			split_reads: vec![]
		}, 0usize),
		|(a1, a2),(b1, b2)| {
			(SplitReadCollection::combine(a1,b1), a2 + b2)
		}
	)
}
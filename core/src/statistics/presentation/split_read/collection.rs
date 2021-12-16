use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::Mutex;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde_derive::{Serialize, Deserialize};
use thiserror::Error;
use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use crate::statistics::presentation::assembler::PresentationAssembler;
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

	pub fn combine(mut a: SplitReadCollection, mut b: SplitReadCollection) -> SplitReadCollection {
		a.split_reads.append(&mut b.split_reads);

		Self {
			split_reads: a.split_reads
		}
	}
}

#[derive(Error, Debug)]
pub enum SplitReadCollectionTryFromAssemblerError {
	#[error("too many overlapping records, more than one possible solution")]
	Unsolvable {

	},
	#[error("couldn't find next in sequence")]
	Incomplete {

	}
}

impl TryFrom<PresentationAssembler> for (SplitReadCollection, usize, usize, usize, usize) {
	type Error = SplitReadCollectionTryFromAssemblerError;

	fn try_from(value: PresentationAssembler) -> Result<Self, Self::Error> {
		let remove_supplementary = false;

		let PresentationAssembler {
			associated_records
		} = value;

		let split_reads: Mutex<Vec<SplitRead>> = Mutex::new(vec![]);

		let partial_split_reads: Vec<PartialSplitRead> = associated_records.into_iter().map(|record| {
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

		let mut supplementary_dropped = 0usize;

		if remove_supplementary {
			supplementary_dropped += remove_supplementary_reads(&mut partial_split_read_map);
		}

		let (
			completed_split_reads,
			dropped_no_next,
			dropped_missing_info,
			dropped_unmergeable,
			dropped_supplementary
		) = merge_partial_split_read_map(&mut partial_split_read_map);

		supplementary_dropped += dropped_supplementary;

		for completed_split_read in completed_split_reads {
			let split_read = completed_split_read.try_into().unwrap();
			let mut split_reads_lock = split_reads.lock().unwrap();
			split_reads_lock.push(split_read);
		}

		Ok((SplitReadCollection {
			split_reads: split_reads.into_inner().unwrap()
		}, dropped_no_next, dropped_missing_info, dropped_unmergeable, supplementary_dropped))
	}
}

fn remove_supplementary_reads(partial_split_read_map: &mut HashMap<(i32, u32), Vec<PartialSplitRead>>) -> usize {
	let mut removed_reads = 0usize;
	loop {
		let next_supplementary_read = find_next_supplementary_read(partial_split_read_map);
		match next_supplementary_read {
			None => {
				break;
			}
			Some((key, i)) => {
				removed_reads += 1;
				let vec = partial_split_read_map.get_mut(&key).unwrap();
				if vec.len() == 1 {
					partial_split_read_map.remove(&key);
				}
				else {
					vec.remove(i);
				}
			}
		}
	}
	removed_reads
}

fn find_next_supplementary_read(partial_split_read_map: &HashMap<(i32, u32), Vec<PartialSplitRead>>) -> Option<((i32, u32), usize)> {
	for (key, vec) in partial_split_read_map {
		for (i, read) in vec.iter().enumerate() {
			if read.is_supplementary() {
				return Some((*key, i));
			}
		}
	}
	None
}

fn merge_partial_split_read_map(partial_split_read_map: &mut HashMap<(i32, u32), Vec<PartialSplitRead>>) -> (Vec<PartialSplitRead>, usize, usize, usize, usize) {
	merge_partial_split_read_map_inner(partial_split_read_map, false)
}

fn merge_partial_split_read_map_inner(partial_split_read_map: &mut HashMap<(i32, u32), Vec<PartialSplitRead>>, removed_supplementary: bool) -> (Vec<PartialSplitRead>, usize, usize, usize, usize) {
	let brute_force_unmergeable = true;

	let mut completed_split_reads: Vec<PartialSplitRead> = vec![];

	let mut dropped_no_next = 0usize;
	let mut dropped_missing_info = 0usize;
	let mut dropped_unmergeable = 0usize;
	let mut dropped_supplementary = 0usize;

	loop {
		if partial_split_read_map.is_empty() { break; }

		let (pair, all_no_next, all_missing_chain) = get_next_pair(partial_split_read_map);

		match pair {
			None => {
				let mut count = 0;
				for (_, value) in partial_split_read_map.iter_mut() {
					count += value.len();
				}

				//let read = partial_split_read_map.values().next().unwrap().first().unwrap();
				//let qname = read.get_name();
				//let tlen = read.get_template_length();

				if removed_supplementary {
					if brute_force_unmergeable {
						let brute_force_merged = brute_force_merge(partial_split_read_map);
						match brute_force_merged {
							Ok(read) => {
								completed_split_reads.push(read)
							}
							Err(count) => {
								dropped_unmergeable += count;
							}
						}
					}
					else {
						if let Some(true) = all_no_next {
							dropped_no_next += count;
							//println!("WARN: Template with split reads that have pnext and rnext information that do not have corresponding reads, {} unmergeable reads were dropped, qname {}, tlen: {}", count, qname, tlen);
						} else if let Some(true) = all_missing_chain {
							dropped_missing_info += count;
							//println!("WARN: Template with split reads that do not have pnext or rnext information, {} unmergeable reads were dropped, qname: {}, tlen: {}", count, qname, tlen);
						} else {
							dropped_unmergeable += count;
							//println!("WARN: Template that cannot be solver because of too many overlapping reads, {} unmergeable reads were dropped, qname: {}, tlen: {}", count, qname, tlen);
						}
					}
				}
				else {
					dropped_supplementary += remove_supplementary_reads(partial_split_read_map);
					let (
						recurse_completed_split_reads,
						recurse_dropped_no_next,
						recurse_dropped_missing_info,
						recurse_dropped_unmergeable,
						recurse_dropped_supplementary
					) = merge_partial_split_read_map_inner(partial_split_read_map, true);

					completed_split_reads.extend(recurse_completed_split_reads);
					dropped_no_next += recurse_dropped_no_next;
					dropped_missing_info += recurse_dropped_missing_info;
					dropped_unmergeable += recurse_dropped_unmergeable;
					dropped_supplementary += recurse_dropped_supplementary;
				}
				break;
			}
			Some((first_key, second_key)) => {
				if (first_key.0, first_key.1) == second_key {
					let mut vec = partial_split_read_map.remove(&(second_key)).unwrap();
					if vec.len() == 1 {
						panic!()
					} else if vec.len() == 2 {
						let first = vec.remove(first_key.2);
						let second = vec.remove(0);

						let combined = match PartialSplitRead::combine(first, second) {
							Ok(split) => {
								split
							}
							Err(_) => {
								panic!()
							}
						};

						if combined.is_complete() {
							completed_split_reads.push(combined);
						}
						else {
							partial_split_read_map.insert(second_key, vec![combined]);
						}
					} else {
						panic!("")
					}
				} else {
					let mut first_vec = partial_split_read_map.remove(&(first_key.0, first_key.1)).unwrap();
					let first = first_vec.remove(first_key.2);
					let mut second_vec = partial_split_read_map.remove(&second_key).unwrap();
					let second = second_vec.remove(0);

					if !first_vec.is_empty() {
						partial_split_read_map.insert((first_key.0, first_key.1), first_vec);
					}

					let combined = match PartialSplitRead::combine(first, second) {
						Ok(split_read) => {
							split_read
						}
						Err((_first, _second)) => {
							println!("ERROR: Combine Error");
							panic!("Combine Error");
						}
					};

					if combined.is_complete() {
						completed_split_reads.push(combined);
					} else {
						let start = combined.get_start();
						let ref_id = combined.get_ref_id();

						if partial_split_read_map.contains_key(&(ref_id, start)) {
							let records = partial_split_read_map.get_mut(&(ref_id, start)).unwrap();
							records.push(combined);
						} else {
							partial_split_read_map.insert((ref_id, start), vec![combined]);
						}
					}
				}
			}
		}
	}

	return (completed_split_reads, dropped_no_next, dropped_missing_info, dropped_unmergeable, dropped_supplementary);
}

fn brute_force_merge(partial_split_read_map: &mut HashMap<(i32, u32), Vec<PartialSplitRead>>) -> Result<PartialSplitRead, usize> {
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
						Err(_) => {
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

	return if record.is_complete() {
		Ok(record)
	}
	else {
		Err(combined_count)
	}

}

fn count_remaining_records(partial_split_read_map: &HashMap<(i32, u32), Vec<PartialSplitRead>>) -> usize {
	let mut count = 0;

	for (_, vec) in partial_split_read_map {
		count += vec.len();
	}

	count
}

fn get_next_pair(partial_split_read_map: &HashMap<(i32, u32), Vec<PartialSplitRead>>) -> (Option<((i32, u32, usize), (i32, u32))>, Option<bool>, Option<bool>) {
	let mut all_no_next = None;
	let mut all_missing_chain = None;

	for (key, value) in partial_split_read_map {
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
						return (pair, Some(false), Some(false));
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
			return (pair, Some(false), Some(false));
		}
	}

	(None, all_no_next, all_missing_chain)
}

#[derive(Error, Debug)]
pub enum SplitReadCollectionTryFromAssemblerCollectionError {
	#[error("could not convert single assembler into split read collection")]
	FromAssembler {
		source: SplitReadCollectionTryFromAssemblerError
	}
}

impl TryFrom<PresentationAssemblerCollection> for (SplitReadCollection, usize, usize, usize, usize) {
	type Error = SplitReadCollectionTryFromAssemblerCollectionError;

	fn try_from(value: PresentationAssemblerCollection) -> Result<Self, Self::Error> {
		let assemblers = value.into_inner();

		let collections = assemblers.into_iter().par_bridge().fold(
			|| {
				Ok((SplitReadCollection {
					split_reads: vec![]
				}, 0usize, 0usize, 0usize, 0usize))
			},
			|a,b| {
				let (a1, a2, a3, a4, a5) = a?;
				let (b1, b2, b3, b4, b5): (SplitReadCollection, usize, usize, usize, usize) = b.try_into().map_err(|source| {
					SplitReadCollectionTryFromAssemblerCollectionError::FromAssembler {
						source
					}
				})?;
				Ok((SplitReadCollection::combine(a1,b1), a2 + b2, a3 + b3, a4 + b4, a5 + b5))
			}
		).collect::<Result<Vec<(SplitReadCollection, usize, usize, usize, usize)>, SplitReadCollectionTryFromAssemblerCollectionError>>()?;
		
		Ok(collections.into_iter().fold(
			(SplitReadCollection {
				split_reads: vec![]
			}, 0usize, 0usize, 0usize, 0usize),
			|(a1, a2, a3, a4,a5),(b1, b2, b3, b4,b5)| {
				(SplitReadCollection::combine(a1,b1), a2 + b2, a3 + b3, a4 + b4, a5 + b5)
			}
		))
	}
}
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde_derive::{Serialize, Deserialize};
use thiserror::Error;
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

#[derive(Error, Debug)]
pub enum SplitReadCollectionTryFromAssemblerError {
	#[error("too many overlapping records, more than one possible solution")]
	Unsolvable {

	},
	#[error("couldn't find next in sequence")]
	Incomplete {

	}
}

impl TryFrom<PresentationAssembler> for SplitReadCollection {
	type Error = SplitReadCollectionTryFromAssemblerError;

	fn try_from(value: PresentationAssembler) -> Result<Self, Self::Error> {
		let PresentationAssembler {
			template_length_map
		} = value;

		let mut split_reads: Vec<SplitRead> = vec![];

		for template_length in template_length_map {
			let (_, associated_records) = template_length;

			let partial_split_reads: Vec<PartialSplitRead> = associated_records.into_iter().map(|record| {
				PartialSplitRead::from_read(record)
			}).collect();

			let mut completed_split_reads: Vec<PartialSplitRead> = vec![];

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

			loop {
				if partial_split_read_map.is_empty() { break; }

				let mut pair: Option<((i32, u32, usize),(i32, u32))> = None;

				let mut all_missing_chain = None;

				for (key, value) in &partial_split_read_map {
					for (i, read) in value.iter().enumerate() {
						let p_next = read.get_p_next();
						let r_next = read.get_r_next();

						if p_next == -1 || r_next == -1 {
							if let None = all_missing_chain {
								all_missing_chain = Some(true);
							}
							continue;
						}

						all_missing_chain = Some(false);

						let next = partial_split_read_map.get(&(r_next, p_next as u32)).unwrap();

						if next.len() > 1 { continue; }

						pair = Some(((key.0, key.1, i), (r_next, p_next as u32)));
						break;
					}
				}

				match pair {
					None => {
						if let Some(true) = all_missing_chain {
							let mut count = 0;
							for (_, value) in &partial_split_read_map {
								count += value.len();
							}
							let read = partial_split_read_map.values().next().unwrap().first().unwrap();
							let qname = read.get_name();
							let tlen = read.get_template_length();
							println!("WARN: Template with split reads that do not have pnext or rnext information, {} unmergeable reads were dropped, qname: {}, tlen: {}", count, qname, tlen);
							break;
						}
						else {
							return Err(SplitReadCollectionTryFromAssemblerError::Unsolvable {})
						}
					}
					Some((first_key, second_key)) => {
						let mut first_vec = partial_split_read_map.remove(&(first_key.0, first_key.1)).unwrap();
						let first = first_vec.remove(first_key.2);
						let mut second_vec = partial_split_read_map.remove(&second_key).unwrap();
						let second = second_vec.remove(0);

						if !first_vec.is_empty() {
							partial_split_read_map.insert((first_key.0, first_key.1), first_vec);
						}

						let combined = match PartialSplitRead::combine(first,second) {
							Ok(split_read) => {
								split_read
							}
							Err(_) => {
								panic!("Combine Error");
							}
						};

						if combined.is_complete() {
							completed_split_reads.push(combined);
						}
						else {
							let start = combined.get_start();
							let ref_id = combined.get_ref_id();

							if partial_split_read_map.contains_key(&(ref_id, start)) {
								let records = partial_split_read_map.get_mut(&(ref_id, start)).unwrap();
								records.push(combined);
							}
							else {
								partial_split_read_map.insert((ref_id, start), vec![combined]);
							}
						}
					}
				}
			}

			for completed_split_read in completed_split_reads {
				let split_read = completed_split_read.try_into().unwrap();
				split_reads.push(split_read);
			}
		}

		Ok(Self {
			split_reads
		})
	}
}

enum PartialSplitRead {
	SingleSplitRead(PresentationRecord),
	StartOnly(PresentationRecord),
	StartAndMiddle(PresentationRecord, Vec<PresentationRecord>),
	MiddleOnly(Vec<PresentationRecord>),
	MiddleAndEnd(Vec<PresentationRecord>, PresentationRecord),
	EndOnly(PresentationRecord),
	StartAndEnd(PresentationRecord, PresentationRecord),
	StartMiddleEnd(PresentationRecord, Vec<PresentationRecord>, PresentationRecord),
	EndAndStart(PresentationRecord, PresentationRecord),
	EndAndStartAndMiddle(PresentationRecord, PresentationRecord, Vec<PresentationRecord>),
	MiddleAndEndAndStart(Vec<PresentationRecord>, PresentationRecord, PresentationRecord),
	MiddleAndEndAndStartAndMiddle(Vec<PresentationRecord>, PresentationRecord, PresentationRecord, Vec<PresentationRecord>)
}

impl PartialSplitRead {
	pub fn from_read(read: PresentationRecord) -> Self {
		let flags = read.get_flags();

		match (flags.get_is_first_mate(), flags.get_is_last_mate()) {
			(true, true) => {
				Self::SingleSplitRead(read)
			},
			(true, false) => {
				Self::StartOnly(read)
			},
			(false, true) => {
				Self::EndOnly(read)
			},
			(false, false) => {
				Self::MiddleOnly(vec![read])
			}
		}
	}

	pub fn get_name(&self) -> String {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::StartAndMiddle(read, _) |
			PartialSplitRead::MiddleAndEnd(_, read) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(read, _) |
			PartialSplitRead::StartMiddleEnd(read, _, _) |
			PartialSplitRead::EndAndStart(read, _) |
			PartialSplitRead::EndAndStartAndMiddle(read, _, _) |
			PartialSplitRead::MiddleAndEndAndStart(_, read, _) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, read, _, _) => {
				read.get_name()
			}
			PartialSplitRead::MiddleOnly(middle) => {
				middle.first().unwrap().get_name()
			}
		}
	}

	pub fn get_template_length(&self) -> i32 {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::StartAndMiddle(read, _) |
			PartialSplitRead::MiddleAndEnd(_, read) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(read, _) |
			PartialSplitRead::StartMiddleEnd(read, _, _) |
			PartialSplitRead::EndAndStart(read, _) |
			PartialSplitRead::EndAndStartAndMiddle(read, _, _) |
			PartialSplitRead::MiddleAndEndAndStart(_, read, _) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, read, _, _) => {
				read.get_template_length()
			}
			PartialSplitRead::MiddleOnly(middle) => {
				middle.first().unwrap().get_template_length()
			}
		}
	}

	pub fn get_start(&self) -> u32 {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::StartAndMiddle(read, _) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(read, _) |
			PartialSplitRead::StartMiddleEnd(read, _, _) |
			PartialSplitRead::EndAndStart(read, _) |
			PartialSplitRead::EndAndStartAndMiddle(read, _, _) => {
				read.get_start()
			}
			PartialSplitRead::MiddleOnly(middle) |
			PartialSplitRead::MiddleAndEnd(middle, _) |
			PartialSplitRead::MiddleAndEndAndStart(middle, _, _) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(middle, _, _, _) => {
				middle.first().unwrap().get_start()
			}
		}
	}

	pub fn get_p_next(&self) -> i32 {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::MiddleAndEnd(_, read) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(_, read) |
			PartialSplitRead::StartMiddleEnd(_, _, read) |
			PartialSplitRead::EndAndStart(_, read) |
			PartialSplitRead::MiddleAndEndAndStart(_, _, read) => {
				read.get_p_next()
			}
			PartialSplitRead::StartAndMiddle(_, middle) |
			PartialSplitRead::MiddleOnly(middle) |
			PartialSplitRead::EndAndStartAndMiddle(_, _, middle) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, _, _, middle) => {
				middle.last().unwrap().get_p_next()
			}
		}
	}

	pub fn get_ref_id(&self) -> i32 {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::StartAndMiddle(read, _) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(read, _) |
			PartialSplitRead::StartMiddleEnd(read, _, _) |
			PartialSplitRead::EndAndStart(read, _) |
			PartialSplitRead::EndAndStartAndMiddle(read, _, _) => {
				read.get_ref_id()
			}
			PartialSplitRead::MiddleOnly(middle) |
			PartialSplitRead::MiddleAndEnd(middle, _) |
			PartialSplitRead::MiddleAndEndAndStart(middle, _, _) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(middle, _, _, _) => {
				middle.first().unwrap().get_ref_id()
			}
		}
	}

	pub fn get_r_next(&self) -> i32 {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::StartAndMiddle(read, _) |
			PartialSplitRead::MiddleAndEnd(_, read) |
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::StartAndEnd(_, read) |
			PartialSplitRead::StartMiddleEnd(_, _, read) |
			PartialSplitRead::EndAndStart(_, read) |
			PartialSplitRead::MiddleAndEndAndStart(_, _, read) => {
				read.get_r_next()
			}
			PartialSplitRead::MiddleOnly(middle) |
			PartialSplitRead::EndAndStartAndMiddle(_, _, middle) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, _, _, middle) => {
				middle.last().unwrap().get_r_next()
			}
		}
	}

	pub fn is_complete(&self) -> bool {
		match self {
			PartialSplitRead::StartOnly(_) |
			PartialSplitRead::StartAndMiddle(_, _) |
			PartialSplitRead::MiddleOnly(_) |
			PartialSplitRead::MiddleAndEnd(_, _) |
			PartialSplitRead::EndAndStart(_, _) |
			PartialSplitRead::EndAndStartAndMiddle(_, _, _) |
			PartialSplitRead::MiddleAndEndAndStart(_, _, _) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, _, _, _) |
			PartialSplitRead::EndOnly(_) => {
				false
			},
			PartialSplitRead::SingleSplitRead(_) |
			PartialSplitRead::StartAndEnd(_, _) |
			PartialSplitRead::StartMiddleEnd(_, _, _) => {
				true
			}
		}
	}

	pub fn combine(a: Self, b: Self) -> Result<Self, (Self,Self)> {
		match (a,b) {
			(Self::StartOnly(start), Self::MiddleOnly(middle)) |
			(Self::MiddleOnly(middle), Self::StartOnly(start)) => {
				let middle_first = middle.first().unwrap();
				if start.get_p_next() as u32 == middle_first.get_start() &&
					start.get_r_next() == middle_first.get_ref_id() {
					Ok(Self::StartAndMiddle(start, middle))
				}
				else {
					Err((Self::StartOnly(start), Self::MiddleOnly(middle)))
				}
			},

			(Self::StartOnly(start), Self::MiddleAndEnd(middle, end)) |
			(Self::MiddleAndEnd(middle, end), Self::StartOnly(start)) => {
				let middle_first = middle.first().unwrap();

				let end_continue = end.get_p_next() as u32 == start.get_start() &&
					end.get_r_next() == start.get_ref_id();
				let start_continue = start.get_p_next() as u32 == middle_first.get_start() &&
					start.get_r_next() == middle_first.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						Ok(Self::StartMiddleEnd(start, middle, end))
					},
					(false , true) => {
						Ok(Self::MiddleAndEndAndStart(middle, end, start))
					}
					(true, false) |
					(false, false) => {
						Err((Self::StartOnly(start), Self::MiddleAndEnd(middle, end)))
					}
				}
			},

			(Self::StartOnly(start), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::StartOnly(start)) => {
				let start_continue = start.get_p_next() as u32 == end.get_start() &&
					start.get_r_next() == end.get_ref_id();
				let end_continue = end.get_p_next() as u32 == start.get_start() &&
					end.get_r_next() == start.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						Ok(Self::StartAndEnd(start, end))
					},
					(false, true) => {
						Ok(Self::EndAndStart(end, start))
					},
					(true, false) |
					(false, false) => {
						Err((Self::StartOnly(start), Self::EndOnly(end)))
					}
				}
			}

			(Self::StartAndMiddle(start, mut start_middle), Self::MiddleOnly(middle)) |
			(Self::MiddleOnly(middle), Self::StartAndMiddle(start, mut start_middle)) => {
				let start_middle_last = start_middle.last().unwrap();
				let middle_first = middle.first().unwrap();
				if start_middle_last.get_p_next() as u32 == middle_first.get_start() &&
					start_middle_last.get_r_next() == middle_first.get_ref_id() {
					start_middle.extend(middle);
					Ok(Self::StartAndMiddle(start, start_middle))
				}
				else {
					Err((Self::StartAndMiddle(start, start_middle), Self::MiddleOnly(middle)))
				}
			},

			(Self::StartAndMiddle(start, mut start_middle), Self::MiddleAndEnd(end_middle, end)) |
			(Self::MiddleAndEnd(end_middle, end), Self::StartAndMiddle(start, mut start_middle)) => {
				let start_middle_last = start_middle.last().unwrap();
				let end_middle_first = end_middle.first().unwrap();

				let start_continue = start_middle_last.get_p_next() as u32 == end_middle_first.get_start() &&
					start_middle_last.get_r_next() == end_middle_first.get_ref_id();
				let end_continue = end.get_p_next() as u32 == start.get_start() &&
					end.get_r_next() == start.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						start_middle.extend(end_middle);
						Ok(Self::StartMiddleEnd(start, start_middle, end))
					},
					(false, true) => {
						Ok(Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, start_middle))
					}
					(true, false) |
					(false, false) => {
						Err((Self::StartAndMiddle(start, start_middle), Self::MiddleAndEnd(end_middle, end)))
					}
				}
			},

			(Self::StartAndMiddle(start, middle), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::StartAndMiddle(start, middle)) => {
				let middle_last = middle.last().unwrap();

				let start_continue =  middle_last.get_p_next() as u32 == end.get_start() &&
					middle_last.get_r_next() == end.get_ref_id();
				let end_continue = end.get_p_next() as u32 == start.get_start() &&
					end.get_r_next() == start.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						Ok(Self::StartMiddleEnd(start, middle, end))
					}
					(false, true) => {
						Ok(Self::EndAndStartAndMiddle(end, start, middle))
					}
					(true, false) |
					(false, false) => {
						Err((Self::StartAndMiddle(start, middle), Self::EndOnly(end)))
					}
				}
			},

			(Self::MiddleOnly(mut a), Self::MiddleOnly(mut b)) => {
				let a_first = a.first().unwrap();
				let a_last = a.last().unwrap();
				let b_first = b.first().unwrap();
				let b_last = b.last().unwrap();
				if a_last.get_p_next() as u32 == b_first.get_start() &&
					a_last.get_r_next() == b_first.get_ref_id() {
					a.extend(b);
					Ok(Self::MiddleOnly(a))
				}
				else if b_last.get_p_next() as u32 == a_first.get_start() &&
					b_last.get_r_next() == a_first.get_ref_id() {
					b.extend(a);
					Ok(Self::MiddleOnly(b))
				}
				else {
					Err((Self::MiddleOnly(a), Self::MiddleOnly(b)))
				}
			},

			(Self::MiddleOnly(mut middle), Self::MiddleAndEnd(end_middle, end)) |
			(Self::MiddleAndEnd(end_middle, end), Self::MiddleOnly(mut middle)) => {
				let middle_last = middle.last().unwrap();
				let end_middle_first = end_middle.first().unwrap();
				if middle_last.get_p_next() as u32 == end_middle_first.get_start() &&
					middle_last.get_r_next() == end_middle_first.get_ref_id() {
					middle.extend(end_middle);
					Ok(Self::MiddleAndEnd(middle, end))
				}
				else {
					Err((Self::MiddleOnly(middle), Self::MiddleAndEnd(end_middle, end)))
				}
			},

			(Self::MiddleOnly(middle), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::MiddleOnly(middle)) => {
				let middle_last = middle.last().unwrap();
				if middle_last.get_p_next() as u32 == end.get_start() &&
					middle_last.get_r_next() == end.get_ref_id() {
					Ok(Self::MiddleAndEnd(middle, end))
				}
				else {
					Err((Self::MiddleOnly(middle), Self::EndOnly(end)))
				}
			},

			(Self::MiddleOnly(middle), Self::EndAndStart(end, start)) |
			(Self::EndAndStart(end, start), Self::MiddleOnly(middle)) => {
				let middle_first = middle.first().unwrap();
				let middle_last = middle.last().unwrap();

				let start_continue = start.get_p_next() as u32 == middle_first.get_start() &&
					start.get_r_next() == middle_first.get_ref_id();
				let end_continue = middle_last.get_p_next() as u32 == end.get_start() &&
					middle_last.get_r_next() == end.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						Ok(Self::StartMiddleEnd(start, middle, end))
					},
					(true, false) => {
						Ok(Self::EndAndStartAndMiddle(end, start, middle))
					},
					(false, true) => {
						Ok(Self::MiddleAndEndAndStart(middle, end, start))
					},
					(false, false) => {
						Err((Self::MiddleOnly(middle), Self::EndAndStart(end, start)))
					}
				}
			},

			(Self::MiddleOnly(middle), Self::EndAndStartAndMiddle(end, start, mut start_middle)) |
			(Self::EndAndStartAndMiddle(end, start, mut start_middle), Self::MiddleOnly(middle)) => {
				let middle_first = middle.first().unwrap();
				let middle_last = middle.last().unwrap();

				let start_middle_last = start_middle.last().unwrap();

				let start_continue = start_middle_last.get_p_next() as u32 == middle_first.get_start() &&
					start_middle_last.get_r_next() == middle_first.get_ref_id();
				let end_continue = middle_last.get_p_next() as u32 == end.get_start() &&
					middle_last.get_r_next() == end.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						start_middle.extend(middle);
						Ok(Self::StartMiddleEnd(start, start_middle, end))
					},
					(true, false) => {
						start_middle.extend(middle);
						Ok(Self::EndAndStartAndMiddle(end, start, start_middle))
					},
					(false, true) => {
						Ok(Self::MiddleAndEndAndStartAndMiddle(middle, end, start, start_middle))
					},
					(false, false) => {
						Err((Self::MiddleOnly(middle), Self::EndAndStartAndMiddle(end, start, start_middle)))
					}
				}
			},

			(Self::MiddleOnly(mut middle), Self::MiddleAndEndAndStart(end_middle, end, start)) |
			(Self::MiddleAndEndAndStart(end_middle, end, start), Self::MiddleOnly(mut middle)) => {
				let middle_first = middle.first().unwrap();
				let middle_last = middle.last().unwrap();

				let end_middle_first = end_middle.first().unwrap();

				let start_continue = start.get_p_next() as u32 == middle_first.get_start() &&
					start.get_r_next() == middle_first.get_ref_id();
				let end_continue = middle_last.get_p_next() as u32 == end_middle_first.get_start() &&
					middle_last.get_r_next() == end_middle_first.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						middle.extend(end_middle);
						Ok(Self::StartMiddleEnd(start, middle, end))
					},
					(true, false) => {
						Ok(Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, middle))
					},
					(false, true) => {
						middle.extend(end_middle);
						Ok(Self::MiddleAndEndAndStart(middle, end, start))
					},
					(false, false) => {
						Err((Self::MiddleOnly(middle), Self::MiddleAndEndAndStart(end_middle,end, start)))
					}
				}
			},

			(Self::MiddleOnly(mut middle), Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, mut start_middle)) |
			(Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, mut start_middle), Self::MiddleOnly(mut middle)) => {
				let middle_first = middle.first().unwrap();
				let middle_last = middle.last().unwrap();

				let end_middle_first = end_middle.first().unwrap();
				let start_middle_last = start_middle.last().unwrap();

				let start_continue = start_middle_last.get_p_next() as u32 == middle_first.get_start() &&
					start_middle_last.get_r_next() == middle_first.get_ref_id();
				let end_continue = middle_last.get_p_next() as u32 == end_middle_first.get_start() &&
					middle_last.get_r_next() == end_middle_first.get_ref_id();

				match (start_continue, end_continue) {
					(true, true) => {
						start_middle.extend(middle);
						start_middle.extend(end_middle);
						Ok(Self::StartMiddleEnd(start, start_middle, end))
					},
					(true, false) => {
						start_middle.extend(middle);
						Ok(Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, start_middle))
					},
					(false, true) => {
						middle.extend(end_middle);
						Ok(Self::MiddleAndEndAndStart(middle, end, start))
					},
					(false, false) => {
						Err((Self::MiddleOnly(middle), Self::MiddleAndEndAndStartAndMiddle(end_middle,end, start, start_middle)))
					}
				}
			},

			(a, b) => Err((a,b))
		}
	}
}

impl TryInto<SplitRead> for PartialSplitRead {
	type Error = ();

	fn try_into(self) -> Result<SplitRead, Self::Error> {
		let records = match self {
			PartialSplitRead::SingleSplitRead(read) => {
				vec![read]
			}
			PartialSplitRead::StartAndEnd(start, end) => {
				vec![start, end]
			}
			PartialSplitRead::StartMiddleEnd(start, mut middle, end) => {
				middle.push(start);
				middle.push(end);
				middle
			}
			_ => {
				return Err(());
			}
		};

		Ok(SplitRead::from(records))
	}
}

#[derive(Error, Debug)]
pub enum SplitReadCollectionTryFromAssemblerCollectionError {
	#[error("could not convert single assembler into split read collection")]
	FromAssembler {
		source: SplitReadCollectionTryFromAssemblerError
	}
}

impl TryFrom<PresentationAssemblerCollection> for SplitReadCollection {
	type Error = SplitReadCollectionTryFromAssemblerCollectionError;

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
				let b = b.try_into().map_err(|source| {
					SplitReadCollectionTryFromAssemblerCollectionError::FromAssembler {
						source
					}
				})?;
				Ok(Self::combine(a,b))
			}
		).collect::<Result<Vec<SplitReadCollection>, SplitReadCollectionTryFromAssemblerCollectionError>>()?;
		
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
pub mod map;

use std::convert::TryInto;
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::presentation::split_read::SplitRead;

#[derive(Debug, Clone)]
pub enum PartialSplitRead {
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

	pub fn get_template_length(&self) -> u32 {
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

	pub fn is_supplementary(&self) -> bool {
		match self {
			PartialSplitRead::SingleSplitRead(read) |
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::EndOnly(read) => {
				read.get_flags().get_is_supplementary()
			}
			PartialSplitRead::MiddleOnly(middle) => {
				for read in middle {
					if !read.get_flags().get_is_supplementary() {
						return false;
					}
				}
				true
			}
			PartialSplitRead::StartAndMiddle(read, middle) |
			PartialSplitRead::MiddleAndEnd(middle, read) => {
				if !read.get_flags().get_is_supplementary() {
					return false;
				}

				for read in middle {
					if !read.get_flags().get_is_supplementary() {
						return false;
					}
				}
				true
			}
			PartialSplitRead::StartAndEnd(start, end) |
			PartialSplitRead::EndAndStart(end, start) => {
				start.get_flags().get_is_supplementary() && end.get_flags().get_is_supplementary()
			}
			PartialSplitRead::StartMiddleEnd(start, middle, end) |
			PartialSplitRead::EndAndStartAndMiddle(end, start, middle) |
			PartialSplitRead::MiddleAndEndAndStart(middle, end, start) => {
				if !(start.get_flags().get_is_supplementary() && end.get_flags().get_is_supplementary()) {
					return false;
				}

				for read in middle {
					if !read.get_flags().get_is_supplementary() {
						return false;
					}
				}
				true
			}
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(end_middle, end, start, start_middle) => {
				if !(start.get_flags().get_is_supplementary() && end.get_flags().get_is_supplementary()) {
					return false;
				}

				for read in end_middle.iter().chain(start_middle.iter()) {
					if !read.get_flags().get_is_supplementary() {
						return false;
					}
				}
				true
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

	pub fn is_next_unmapped(&self) -> bool {
		match self {
			PartialSplitRead::StartMiddleEnd(_, _, _) |
			PartialSplitRead::StartAndEnd(_, _) |
			PartialSplitRead::StartOnly(_) => {
				false
			}
			PartialSplitRead::EndOnly(read) |
			PartialSplitRead::MiddleAndEnd(_, read) |
			PartialSplitRead::MiddleAndEndAndStart(_, _, read) |
			PartialSplitRead::EndAndStart(_, read) |
			PartialSplitRead::SingleSplitRead(read) => {
				read.get_flags().get_is_next_unmapped()
			}
			PartialSplitRead::MiddleOnly(middle) |
			PartialSplitRead::StartAndMiddle(_, middle) |
			PartialSplitRead::EndAndStartAndMiddle(_, _, middle) |
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, _, _, middle) => {
				middle.last().unwrap().get_flags().get_is_next_unmapped()
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

	pub fn combine_non_continuous(a: Self, b: Self) -> Result<Self, (Self, Self)> {
		match (a,b) {
			(Self::StartOnly(start), Self::MiddleOnly(middle)) |
			(Self::MiddleOnly(middle), Self::StartOnly(start)) => {
				Ok(Self::StartAndMiddle(start, middle))
			}
			(Self::StartOnly(start), Self::MiddleAndEnd(middle, end)) |
			(Self::MiddleAndEnd(middle, end), Self::StartOnly(start)) => {
				Ok(Self::StartMiddleEnd(start, middle, end))
			}
			(Self::StartOnly(start), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::StartOnly(start)) => {
				Ok(Self::StartAndEnd(start, end))
			}
			(Self::StartAndMiddle(start, mut start_middle), Self::MiddleOnly(middle)) |
			(Self::MiddleOnly(middle), Self::StartAndMiddle(start, mut start_middle)) => {
				start_middle.extend(middle);
				Ok(Self::StartAndMiddle(start, start_middle))
			}
			(Self::StartAndMiddle(start, mut start_middle), Self::MiddleAndEnd(end_middle, end)) |
			(Self::MiddleAndEnd(end_middle, end), Self::StartAndMiddle(start, mut start_middle)) => {
				start_middle.extend(end_middle);
				Ok(Self::StartMiddleEnd(start, start_middle, end))
			}
			(Self::StartAndMiddle(start, middle), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::StartAndMiddle(start, middle)) => {
				Ok(Self::StartMiddleEnd(start, middle, end))
			}
			(Self::MiddleOnly(mut middle_a), Self::MiddleOnly(middle_b)) => {
				middle_a.extend(middle_b);
				Ok(Self::MiddleOnly(middle_a))
			}
			(Self::MiddleOnly(mut middle), Self::MiddleAndEnd(end_middle, end)) |
			(Self::MiddleAndEnd(end_middle, end), Self::MiddleOnly(mut middle)) => {
				middle.extend(end_middle);
				Ok(Self::MiddleAndEnd(middle, end))
			}
			(Self::MiddleOnly(middle), Self::EndOnly(end)) |
			(Self::EndOnly(end), Self::MiddleOnly(middle)) => {
				Ok(Self::MiddleAndEnd(middle, end))
			}
			(Self::MiddleOnly(middle), Self::StartAndEnd(start, end)) |
			(Self::StartAndEnd(start, end), Self::MiddleOnly(middle)) => {
				Ok(Self::StartMiddleEnd(start, middle, end))
			}
			(Self::MiddleOnly(mut middle_a), Self::StartMiddleEnd(start, middle_b, end)) |
			(Self::StartMiddleEnd(start, middle_b, end), Self::MiddleOnly(mut middle_a)) => {
				middle_a.extend(middle_b);
				Ok(Self::StartMiddleEnd(start, middle_a, end))
			}
			(Self::MiddleOnly(middle), Self::EndAndStart(end, start)) |
			(Self::EndAndStart(end, start), Self::MiddleOnly(middle)) => {
				Ok(Self::StartMiddleEnd(start, middle, end))
			}
			(Self::MiddleOnly(middle), Self::EndAndStartAndMiddle(end, start, mut start_middle)) |
			(Self::EndAndStartAndMiddle(end, start, mut start_middle), Self::MiddleOnly(middle)) => {
				start_middle.extend(middle);
				Ok(Self::StartMiddleEnd(start, start_middle, end))
			}
			(Self::MiddleOnly(mut middle), Self::MiddleAndEndAndStart(end_middle, end, start)) |
			(Self::MiddleAndEndAndStart(end_middle, end, start), Self::MiddleOnly(mut middle)) => {
				middle.extend(end_middle);
				Ok(Self::StartMiddleEnd(start, middle, end))
			}
			(Self::MiddleOnly(middle), Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, mut start_middle)) |
			(Self::MiddleAndEndAndStartAndMiddle(end_middle, end, start, mut start_middle), Self::MiddleOnly(middle)) => {
				start_middle.extend(middle);
				start_middle.extend(end_middle);
				Ok(Self::StartMiddleEnd(start, start_middle, end))
			}
			(a, b) => {
				Err((a,b))
			}
		}
	}
}

impl TryInto<SplitRead> for PartialSplitRead {
	type Error = ();

	fn try_into(self) -> Result<SplitRead, Self::Error> {
		let records = match self {
			PartialSplitRead::SingleSplitRead(read) => {
				(vec![read], 0)
			}
			PartialSplitRead::StartAndEnd(start, end) => {
				(vec![start, end], 0)
			}
			PartialSplitRead::StartMiddleEnd(start, mut middle, end) => {
				middle.push(start);
				middle.push(end);
				(middle, 0)
			}
			PartialSplitRead::StartOnly(read) |
			PartialSplitRead::EndOnly(read) => {
				(vec![read], 0)
			}
			PartialSplitRead::MiddleOnly(middle) => {
				(middle, 2)
			}
			PartialSplitRead::MiddleAndEnd(mut middle, end) => {
				middle.push(end);
				(middle, 1)
			}
			PartialSplitRead::MiddleAndEndAndStart(middle, end, start)  => {
				let mut vec = vec![start];
				vec.extend(middle);
				vec.push(end);

				(vec, 1)
			}
			PartialSplitRead::EndAndStart(end, start) => {
				(vec![start, end], 1)
			}
			PartialSplitRead::StartAndMiddle(start, middle) => {
				let mut vec = vec![start];
				vec.extend(middle);
				(vec, 1)
			}
			PartialSplitRead::EndAndStartAndMiddle(end, start, middle) => {
				let mut vec = vec![start];
				vec.extend(middle);
				vec.push(end);
				(vec, 1)
			}
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(end_middle, end, start, start_middle) => {
				let mut vec = vec![start];
				vec.extend(start_middle);
				vec.extend(end_middle);
				vec.push(end);
				(vec, 1)
			}
		};

		Ok(SplitRead::from(records))
	}
}

#[cfg(test)]
mod partial_split_read_tests {
	use crate::statistics::presentation::record::flags::PresentationFlags;
	use crate::statistics::presentation::record::PresentationRecord;
	use rstest::{rstest};
	use crate::statistics::presentation::split_read::partial::PartialSplitRead;

	#[rstest]
	#[case(false, false, "MiddleOnly")]
	#[case(true, false, "StartOnly")]
	#[case(false, true, "EndOnly")]
	#[case(true, true, "SingleSplitRead")]
	fn from_read_test(
		#[values(true, false)]
		is_mapped: bool,
		#[values(true, false)]
		is_split: bool,
		#[values(true, false)]
		is_reverse_strand: bool,
		#[case]
		is_first_mate: bool,
		#[case]
		is_last_mate: bool,
		#[values(true, false)]
		is_supplementary: bool,
		#[values(true, false)]
		is_next_unmapped: bool,
		#[case]
		expected_type: &str
	) {
		let flags = PresentationFlags::new(is_mapped, is_split, is_reverse_strand, is_last_mate, is_first_mate, is_supplementary, is_next_unmapped, false, false, false, false);
		let read = PresentationRecord::new(
			"test".to_string(),
			flags,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0
		);

		let partial_split_read = PartialSplitRead::from_read(read);

		let resulting_type = match partial_split_read {
			PartialSplitRead::SingleSplitRead(_) => {
				"SingleSplitRead"
			}
			PartialSplitRead::StartOnly(_) => {
				"StartOnly"
			}
			PartialSplitRead::StartAndMiddle(_, _) => {
				"StartAndMiddle"
			}
			PartialSplitRead::MiddleOnly(_) => {
				"MiddleOnly"
			}
			PartialSplitRead::MiddleAndEnd(_, _) => {
				"MiddleAndEnd"
			}
			PartialSplitRead::EndOnly(_) => {
				"EndOnly"
			}
			PartialSplitRead::StartAndEnd(_, _) => {
				"StartAndEnd"
			}
			PartialSplitRead::StartMiddleEnd(_, _, _) => {
				"StartMiddleEnd"
			}
			PartialSplitRead::EndAndStart(_, _) => {
				"EndAndStart"
			}
			PartialSplitRead::EndAndStartAndMiddle(_, _, _) => {
				"EndAndStartAndMiddle"
			}
			PartialSplitRead::MiddleAndEndAndStart(_, _, _) => {
				"MiddleAndEndAndStart"
			}
			PartialSplitRead::MiddleAndEndAndStartAndMiddle(_, _, _, _) => {
				"MiddleAndEndAndStartAndMiddle"
			}
		};

		assert_eq!(resulting_type, expected_type);
	}
}
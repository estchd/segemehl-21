use std::num::NonZeroU32;
use bam::Record;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use bam::record::cigar::{Operation, CigarIter};
use std::iter::{Peekable, Rev};

pub fn length(start: u32, end: u32) -> u32 {
	if end < start {
		return 0;
	}

	(end - start) + 1
}

pub fn get_quality_frequency_map(map: &PresentationFrequencyMap<u8>) -> Vec<(u8, u64)> {
	let range = 0..=255u8;

	let mut vec = Vec::<(u8, u64)>::with_capacity(256);

	for i in range {
		let value = map.get(&i).unwrap_or(0);

		vec.push((i, value));
	}

	vec
}

pub struct BinCoordinates {
	pub bin_index: u32,
	pub position_in_bin: u32
}

pub fn calculate_bin(map_start: u32, bin_size: NonZeroU32, entry_index: u32) -> Result<BinCoordinates,()> {
	if entry_index < map_start {return Err(());}

	let shifted_entry_index = entry_index - map_start;

	let bin_index = shifted_entry_index / bin_size.get();
	let position_in_bin = shifted_entry_index % bin_size.get();

	Ok(BinCoordinates {
		bin_index,
		position_in_bin
	})
}
pub fn get_record_name_as_string(record: &Record) -> Result<String, ()> {
	let name = record.name();
	let vec = Vec::from(name);
	String::from_utf8(vec).map_err(|_| ())
}

pub fn get_record_length_on_reference(record: &Record) -> u32 {
	(get_record_end(record) - get_record_start(record)) + 1
}

pub fn get_record_length_sequence(record: &Record) -> u32 {
	record.query_len()
}

pub fn get_record_mapping_quality(record: &Record) -> u8 {
	record.mapq()
}

pub fn get_record_start(record: &Record) -> u32 {
	record.start() as u32
}

pub fn get_record_end(record: &Record) -> u32 {
	record.calculate_end() as u32
}

enum CigarPeekableReversableIter<'a> {
	Norm(Peekable<CigarIter<'a>>),
	Rev(Peekable<Rev<CigarIter<'a>>>)
}

impl<'a> CigarPeekableReversableIter<'a> {
	fn from_norm(iter: Peekable<CigarIter<'a>>) -> CigarPeekableReversableIter<'a> {
		CigarPeekableReversableIter::Norm(iter)
	}

	fn from_rev(iter: Peekable<Rev<CigarIter<'a>>>) -> CigarPeekableReversableIter<'a> {
		CigarPeekableReversableIter::Rev(iter)
	}

	fn peek(&mut self) -> Option<&(u32,Operation)> {
		match self {
			CigarPeekableReversableIter::Norm(iter) => {
				iter.peek()
			},
			CigarPeekableReversableIter::Rev(iter) => {
				iter.peek()
			}
		}
	}

	fn next(&mut self) -> Option<(u32,Operation)> {
		match self {
			CigarPeekableReversableIter::Norm(iter) => {
				iter.next()
			},
			CigarPeekableReversableIter::Rev(iter) => {
				iter.next()
			}
		}
	}
}

pub struct CigarMaxLengthIter<'a> {
	remaining_last: Option<(u32, Operation)>,
	inner: CigarPeekableReversableIter<'a>
}

impl<'a> CigarMaxLengthIter<'a> {
	pub fn new(record: &'a Record) -> CigarMaxLengthIter<'a> {
		let iter = CigarPeekableReversableIter::from_norm(record.cigar().iter().peekable());
		Self::new_internal(iter)
	}

	pub fn new_reversed(record: &'a Record) -> CigarMaxLengthIter<'a> {
		let iter = CigarPeekableReversableIter::from_rev(record.cigar().iter().rev().peekable());
		Self::new_internal(iter)
	}

	fn new_internal(mut inner: CigarPeekableReversableIter<'a>) -> CigarMaxLengthIter<'a> {
		let next_peek = inner.peek();

		// Skip Hard Clips at the beginning of the CIGAR string
		if let Some((_, peek)) = next_peek {
			if let Operation::Hard = *peek {
				let _ = inner.next();
			}
		}
		else {
			return Self {
				remaining_last: None,
				inner
			};
		}

		let next_peek = inner.peek();

		// Skip Soft Clips at the beginning of the CIGAR string
		if let Some((_, peek)) = next_peek {
			if let Operation::Soft = *peek {
				let _ = inner.next();
			}
		}
		Self {
			remaining_last: None,
			inner
		}
	}

	pub fn next(&mut self, max_length: u32) -> Option<(u32, Operation)> {
		if let Some((len, op)) = self.remaining_last {
			return if len > max_length {
				self.remaining_last = Some((len-max_length, op));
				Some((max_length, op))
			}
			else {
				self.remaining_last = None;
				Some((len,op))
			}
		}

		loop {
			let next = self.inner.next();

			return match next {
				None => {
					None
				}
				Some((len, op)) => {
					match op {
						Operation::Deletion |
						Operation::AlnMatch |
						Operation::SeqMatch |
						Operation::SeqMismatch => {
							if len > max_length {
								self.remaining_last = Some((len-max_length, op));
								Some((max_length, op))
							}
							else {
								Some((len,op))
							}
						},
						Operation::Insertion => {
							Some((len,op))
						},
						Operation::Skip |
						Operation::Padding => {
							continue;
						},
						Operation::Soft |
						Operation::Hard => {
							return None;
						}
					}
				}
			}
		}
	}

	pub fn next_for_ref_length(&mut self, length: u32) -> Vec<(u32, Operation)> {
		let mut remaining_length = length;
		let mut collection = Vec::new();

		while remaining_length > 0 {

			let next = self.next(remaining_length);
			match next {
				None => {
					break;
				}
				Some((len,op)) => {
					match op {
						Operation::SeqMatch |
						Operation::SeqMismatch |
						Operation::Deletion |
						Operation::Skip |
						Operation::AlnMatch => {
							collection.push((len,op));
							remaining_length -= len;
						},
						Operation::Insertion => {
							collection.push((len,op));
						},
						Operation::Padding => {
							continue;
						},
						Operation::Soft |
						Operation::Hard => {
							break;
						}
					}
				}
			}
		}

		collection
	}

	pub fn collect(mut self) -> Vec<(u32,Operation)>{
		let mut collection = Vec::new();

		if let Some(rem) = self.remaining_last {
			collection.push(rem);
		}

		loop {
			let next = self.inner.next();

			match next {
				None => {
					break;
				}
				Some((len,op)) => {
					match op {
						Operation::SeqMatch |
						Operation::SeqMismatch |
						Operation::AlnMatch => {
							collection.push((len, Operation::AlnMatch));
						},
						Operation::Insertion |
						Operation::Deletion |
						Operation::Skip => {
							collection.push((len, op));
						}
						Operation::Padding => {},
						Operation::Soft |
						Operation::Hard => {
							break;
						}
					}
				}
			}
		}

		collection
	}
}

#[cfg(test)]
mod tests {
	use crate::util::calculate_bin;
	use std::num::NonZeroU32;
	use rstest::rstest;

	#[rstest]
	#[should_panic]
	#[case::panic(100,100,0,0,0)]
	#[case(0,500,0,0,0)]
	#[case(0,500,250,0,250)]
	#[case(0,500,499,0,499)]
	#[case(0,500,500,1,0)]
	fn calculate_bin_test(
		#[case] map_start: u32,
		#[case] bin_size: u32,
		#[case] entry_index: u32,
		#[case] expected_bin_index: u32,
		#[case] expected_position_in_bin: u32
	) {
		let bin_size = NonZeroU32::new(bin_size).unwrap();
		let bin_coordinates = calculate_bin(
			map_start,
			bin_size,
			entry_index
		).unwrap();

		assert_eq!(
			expected_bin_index,
			bin_coordinates.bin_index,
			"Got Wrong Bin Index, Expected: {}, Actual: {}",
			expected_bin_index,
			bin_coordinates.bin_index
		);

		assert_eq!(
			expected_position_in_bin,
			bin_coordinates.position_in_bin,
			"Got Wrong Position in Bin, Expected: {}, Actual: {}",
			expected_position_in_bin,
			bin_coordinates.position_in_bin
		)
	}
}
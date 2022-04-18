use core::option::Option;
use core::option::Option::{None, Some};
use std::str::FromStr;
use ascii::AsciiString;
use rand::{Rng, thread_rng};
use crate::sequence_entry::SequenceEntry;
use crate::util;

pub struct Sequence {
	start_hard_clip: u32,
	entries: Vec<SequenceEntry>,
	end_hard_clip: u32
}

impl Sequence {
	pub fn generate_random(length_sequence: Option<u32>, length_reference: Option<u32>) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);

		let entries = match (length_sequence, length_reference) {
			(None, None) => {
				Self::generate_open_sequence()
			},
			(Some(sequence), None) => {
				Self::generate_reference_open_sequence(sequence)
			},
			(None, Some(reference)) => {
				Self::generate_sequence_open_sequence(reference)
			},
			(Some(sequence), Some(reference)) => {
				Self::generate_bounded_sequence(reference, sequence)
			}
		};

		Self {
			start_hard_clip: start_clip,
			entries,
			end_hard_clip: end_clip
		}
	}

	pub fn generate_random_only_matches(length: Option<u32>) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);

		let length = match length {
			None => {
				rand.gen_range(10..=1000)
			}
			Some(length) => {
				length
			}
		};

		let mut entries = Vec::<SequenceEntry>::with_capacity(length as usize);

		for _ in 0..length {
			let entry = SequenceEntry::generate_random_no_skips();
			entries.push(entry);
		}

		Self {
			start_hard_clip: start_clip,
			entries,
			end_hard_clip: end_clip
		}
	}

	pub fn generate_random_only_insertions(length: Option<u32>) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);

		let length = match length {
			None => {
				rand.gen_range(10..=1000)
			}
			Some(length) => {
				length
			}
		};

		let mut entries = Vec::<SequenceEntry>::with_capacity(length as usize);

		let start_entry = SequenceEntry::generate_random_no_skips();
		entries.push(start_entry);

		for _ in 0..(length - 2) {
			let entry = SequenceEntry::generate_sequence_skip();
			entries.push(entry);
		}

		let end_entry = SequenceEntry::generate_random_no_skips();
		entries.push(end_entry);


		Self {
			start_hard_clip: start_clip,
			entries,
			end_hard_clip: end_clip
		}
	}

	pub fn generate_random_only_deletions(length: Option<u32>) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);

		let length = match length {
			None => {
				rand.gen_range(10..=1000)
			}
			Some(length) => {
				length
			}
		};

		let mut entries = Vec::<SequenceEntry>::with_capacity(length as usize);

		let start_entry = SequenceEntry::generate_random_no_skips();
		entries.push(start_entry);

		for _ in 0..(length - 2) {
			let entry = SequenceEntry::generate_deletion();
			entries.push(entry);
		}

		let end_entry = SequenceEntry::generate_random_no_skips();
		entries.push(end_entry);

		Self {
			start_hard_clip: start_clip,
			entries,
			end_hard_clip: end_clip
		}
	}

	pub fn generate_random_only_skips(length: Option<u32>) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);

		let length = match length {
			None => {
				rand.gen_range(10..=1000)
			}
			Some(length) => {
				length
			}
		};

		let mut entries = Vec::<SequenceEntry>::with_capacity(length as usize);

		let start_entry = SequenceEntry::generate_random_no_skips();
		entries.push(start_entry);

		for _ in 0..(length - 2) {
			let entry = SequenceEntry::generate_skip();
			entries.push(entry);
		}

		let end_entry = SequenceEntry::generate_random_no_skips();
		entries.push(end_entry);

		Self {
			start_hard_clip: start_clip,
			entries,
			end_hard_clip: end_clip
		}
	}

	fn generate_open_sequence() -> Vec<SequenceEntry> {
		let mut rand = thread_rng();
		let sequence_length = rand.gen_range(10..1000u32);
		let reference_length = rand.gen_range(10..1000u32);

		Self::generate_bounded_sequence(reference_length, sequence_length)
	}

	fn generate_reference_open_sequence(sequence_length: u32) -> Vec<SequenceEntry> {
		let reference_length = thread_rng().gen_range(10..1000u32);

		Self::generate_bounded_sequence(reference_length, sequence_length)
	}

	fn generate_sequence_open_sequence(reference_length: u32) -> Vec<SequenceEntry> {
		let sequence_length = thread_rng().gen_range(10..1000u32);

		Self::generate_bounded_sequence(reference_length, sequence_length)
	}

	fn generate_bounded_sequence(reference_length: u32, sequence_length: u32) -> Vec<SequenceEntry> {
		if sequence_length == reference_length {
			Self::generate_equal_sequence(sequence_length)
		}
		else if sequence_length > reference_length {
			Self::generate_sequence_longer_sequence(reference_length, sequence_length - reference_length)
		}
		else {
			Self::generate_reference_longer_sequence(sequence_length, reference_length - sequence_length)
		}
	}

	fn generate_equal_sequence(length: u32) -> Vec<SequenceEntry> {
		// Reference Skips = Sequence Skips

		// Reference Skips: Consumes Reference, not Sequence
		// Sequence Skips: Consumes Sequence, not Reference

		let (skip_length, regular_length) =
			util::split_random(length, None, Some(0..=20));

		let (soft_clip_length, insertion_length) =
			util::split_random(skip_length, None, None);

		let (soft_clip_start, soft_clip_end) =
			util::split_random(soft_clip_length, None, None);

		Self::generate_sequence(soft_clip_start, regular_length, insertion_length, skip_length, soft_clip_end)
	}

	fn generate_reference_longer_sequence(sequence_length: u32, difference: u32) -> Vec<SequenceEntry>{
		// Reference Skips = Sequence Skips + Difference

		// Reference Skips: Consumes Reference, not Sequence
		// Sequence Skips: Consumes Sequence, not Reference

		let (skip_length, regular_length) =
			util::split_random(sequence_length, None, Some(0..=20));

		let (soft_clip_length, insertion_length) =
			util::split_random(skip_length, None, None);

		let (soft_clip_start, soft_clip_end) =
			util::split_random(soft_clip_length, None, None);

		Self::generate_sequence(soft_clip_start, regular_length, insertion_length, skip_length + difference, soft_clip_end)
	}

	fn generate_sequence_longer_sequence(reference_length: u32, difference: u32) -> Vec<SequenceEntry> {
		// Sequence Skips = Reference Skips + Difference

		// Reference Skips: Consumes Reference, not Sequence
		// Sequence Skips: Consumes Sequence, not Reference

		let (skip_length, regular_length) =
			util::split_random(reference_length, None, Some(0..=20));

		let (soft_clip_length, insertion_length) =
			util::split_random(skip_length + difference, None, None);

		let (soft_clip_start, soft_clip_end) =
			util::split_random(soft_clip_length, None, None);

		Self::generate_sequence(soft_clip_start, regular_length, insertion_length, skip_length, soft_clip_end)

	}

	fn generate_sequence(soft_clip_start: u32, regular_bases: u32, sequence_skips: u32, reference_skips: u32, soft_clip_end: u32) -> Vec<SequenceEntry> {
		// Reference Skips: Consumes Reference, not Sequence (Reference Skips, Deletions)
		// Sequence Skips: Consumes Sequence, not Reference (Insertions)

		let mut soft_clip_start = Self::generate_soft_clip(soft_clip_start);
		let mut middle_sequence = Self::generate_regular_sequence(regular_bases, sequence_skips, reference_skips);
		let mut soft_clip_end = Self::generate_soft_clip(soft_clip_end);

		soft_clip_start.append(&mut middle_sequence);
		soft_clip_start.append(&mut soft_clip_end);

		soft_clip_start
	}

	fn generate_regular_sequence(regular_length: u32, sequence_skips: u32, reference_skips: u32) -> Vec<SequenceEntry> {
		// Reference Skips: Consumes Reference, not Sequence (Reference Skips, Deletions)
		// Sequence Skips: Consumes Sequence, not Reference (Insertions)

		let total_length = regular_length + sequence_skips + reference_skips;

		let mut entries: Vec<SequenceEntry> = Vec::with_capacity(total_length as usize);

		let mut regular = 0;
		let mut sequence = 0;
		let mut reference = 0;

		for _ in 0..total_length {
			let gen_regular = regular < regular_length;
			let gen_sequence = sequence < sequence_skips;
			let gen_reference = reference < reference_skips;

			let mut rand = thread_rng();

			let entry = match (gen_regular, gen_sequence, gen_reference) {
				(true, true, true) => {
					let select = rand.gen_range(0..3);

					if select == 0 {
						regular += 1;
						SequenceEntry::generate_random_no_skips()
					}
					else if select == 1 {
						sequence += 1;
						SequenceEntry::generate_sequence_skip()
					}
					else {
						reference += 1;
						SequenceEntry::generate_reference_skip()
					}
				},
				(true, true, false) => {
					let select = rand.gen_bool(0.5);

					if select {
						regular += 1;
						SequenceEntry::generate_random_no_skips()
					}
					else {
						sequence += 1;
						SequenceEntry::generate_sequence_skip()
					}
				},
				(true, false, true) => {
					let select = rand.gen_bool(0.5);

					if select {
						regular += 1;
						SequenceEntry::generate_random_no_skips()
					}
					else {
						reference += 1;
						SequenceEntry::generate_reference_skip()
					}
				},
				(false, true, true) => {
					let select = rand.gen_bool(0.5);

					if select {
						sequence += 1;
						SequenceEntry::generate_sequence_skip()
					}
					else {
						reference += 1;
						SequenceEntry::generate_reference_skip()
					}
				},
				(true, false, false) => {
					regular += 1;
					SequenceEntry::generate_random_no_skips()
				},
				(false, true, false) => {
					sequence += 1;
					SequenceEntry::generate_sequence_skip()
				},
				(false, false, true) => {
					reference += 1;
					SequenceEntry::generate_reference_skip()
				},
				(false, false, false) => {
					panic!()
				}
			};

			entries.push(entry);
		}

		entries
	}

	fn generate_soft_clip(length: u32) -> Vec<SequenceEntry>{
		let mut entries: Vec<SequenceEntry> = Vec::new();

		for _ in 0..length {
			let entry = SequenceEntry::generate_random_soft_clip();
			entries.push(entry);
		}

		entries
	}

	pub(crate) fn get_sequence(&self) -> Vec<u8> {
		let sequence: Vec<u8> = self.entries.iter()
			.filter_map(|item| if let SequenceEntry::Base(base) = item {
				Some(base)
			}
			else{
				None
			})
			.map(|item| item.base.to_u8()).collect();

		sequence
	}

	pub fn get_cigar(&self) -> Vec<u8> {
		let extended_cigar: Vec<u8> = self.entries.iter()
			.map(|item| match item {
				SequenceEntry::Base(base) => {
					vec![base.cigar.to_u8()]
				}
				SequenceEntry::Skip => {
					vec![0x4E]
				}
				SequenceEntry::Deletion => {
					vec![0x44]
				}
			})
			.flatten()
			.collect();

		let mut compressed_cigar = Self::compress_cigar(extended_cigar.clone());
		if self.start_hard_clip != 0 {
			compressed_cigar.insert(0, (self.start_hard_clip, 0x48))
		}
		if self.end_hard_clip != 0 {
			compressed_cigar.push((self.end_hard_clip, 0x48))
		}

		let mut compressed_cigar_u8: Vec<u8> = Vec::new();
		for (len, cigar) in compressed_cigar {
			let length = AsciiString::from_str(&format!("{}", len)).unwrap();
			let length_bytes = length.as_bytes();
			for byte in length_bytes {
				compressed_cigar_u8.push(*byte);
			}
			compressed_cigar_u8.push(cigar);
		}

		compressed_cigar_u8
	}

	fn compress_cigar(extended_cigar: Vec<u8>) -> Vec<(u32, u8)> {
		let mut compressed: Vec<(u32,u8)> = Vec::new();
		let mut cur_compress: Option<(u32,u8)> = None;

		for cigar in extended_cigar {
			if let Some((len, cur_cigar)) = cur_compress {
				if cigar == cur_cigar {
					cur_compress = Some((len + 1, cigar));
				}
				else {
					compressed.push((len, cur_cigar));
					cur_compress = Some((1, cigar));
				}
			}
			else {
				cur_compress = Some((1, cigar))
			}

		}

		if let Some((len, cur_cigar)) = cur_compress {
			compressed.push((len,cur_cigar));
		}

		compressed
	}

	pub fn get_quality(&self) -> Vec<u8> {
		self.entries.iter()
			.filter_map(|item| if let SequenceEntry::Base(base) = item {
				Some(base)
			}
			else{
				None
			})
			.map(|item| item.quality).collect()
	}
}
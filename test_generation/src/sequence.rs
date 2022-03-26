use core::option::Option;
use core::option::Option::{None, Some};
use std::str::FromStr;
use ascii::AsciiString;
use rand::{Rng, thread_rng};
use crate::{Base, CIGAREntry, generate_random_quality, SequenceEntry};

pub struct Sequence {
	start_hard_clip: u32,
	entries: Vec<SequenceEntry>,
	end_hard_clip: u32
}

impl Sequence {
	pub(crate) fn generate_random(length: usize) -> Self {
		let mut rand = thread_rng();
		let start_clip = rand.gen_range(0..10);
		let end_clip = rand.gen_range(0..10);
		let start_soft_clip = rand.gen_range(0..length);
		let remaining_length = length - start_soft_clip;
		let end_soft_clip = rand.gen_range(0..remaining_length);
		let regular_length = remaining_length - end_soft_clip;
		let mut start_soft_clip = Self::generate_soft_clip(start_soft_clip);
		let mut end_soft_clip = Self::generate_soft_clip(end_soft_clip);
		let mut middle = Self::generate_reqular_sequence(regular_length);
		start_soft_clip.append(&mut middle);
		start_soft_clip.append(&mut end_soft_clip);
		Self {
			start_hard_clip: start_clip,
			entries: start_soft_clip,
			end_hard_clip: end_clip
		}
	}

	fn generate_reqular_sequence(length: usize) -> Vec<SequenceEntry> {
		let mut entries: Vec<SequenceEntry> = Vec::new();

		for _ in 0..length {
			let entry = SequenceEntry::generate_random();
			entries.push(entry);
		}

		entries
	}

	fn generate_soft_clip(length: usize) -> Vec<SequenceEntry>{
		let mut entries: Vec<SequenceEntry> = Vec::new();

		for _ in 0..length {
			let base = Base::generate_random();
			let quality = generate_random_quality();
			let sequence_base = SequenceBase {
				base,
				cigar: CIGAREntry::S,
				quality
			};
			let entry = SequenceEntry::Base(sequence_base);
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

		let sequence_chars: String = sequence.iter().map(|item| *item as char).collect();
		println!("se: {}", sequence_chars);

		sequence
	}

	pub(crate) fn get_cigar(&self) -> Vec<u8> {
		let extended_cigar: Vec<u8> = self.entries.iter()
			.map(|item| match item {
				SequenceEntry::Base(base) => {
					vec![base.cigar.to_u8()]
				}
				SequenceEntry::Skip(len) => {
					vec![0x53; (*len) as usize]
				}
				SequenceEntry::Deletion(len) => {
					vec![0x44; (*len) as usize]
				}
			})
			.flatten()
			.collect();

		let extended_cigar_string = String::from_utf8(extended_cigar.clone()).unwrap();

		println!("ex: {}", extended_cigar_string);

		let mut compressed_cigar = Self::compress_cigar(extended_cigar);
		if self.start_hard_clip != 0 {
			compressed_cigar.insert(0, (self.start_hard_clip, 0x48))
		}
		if self.end_hard_clip != 0 {
			compressed_cigar.push((self.end_hard_clip, 0x48))
		}

		let compressed_cigar_chars: String = compressed_cigar.iter()
			.map(|item| (item.0, item.1 as char))
			.map(|item| format!("{}{}", item.0, item.1))
			.collect();

		println!("cm: {}", compressed_cigar_chars);

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

		compressed
	}

	pub(crate) fn get_quality(&self) -> Vec<u8> {
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

pub struct SequenceBase {
	pub(crate) base: Base,
	pub(crate) cigar: CIGAREntry,
	pub(crate) quality: u8
}

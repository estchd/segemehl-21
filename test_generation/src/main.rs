#![allow(dead_code)]

use bam::{Header};
use bam::header::HeaderEntry;
use rand::{Rng, thread_rng};
use base::Base;
use crate::cigar::CIGAREntry;
use crate::command_line::{CommandLineParameters, TestGenerationMode};
use crate::sequence::Sequence;

mod command_line;
mod sequence;
mod generation_methods;
mod files;
mod cigar;
mod base;
mod sequence_entry;
mod sequence_base;
mod record_generation;
mod util;

fn main() {
	let params = CommandLineParameters::read();
	let sam = format!("{}.sam",params.output_path);
	let bam = format!("{}.bam",params.output_path);
	let count = params.record_count;

	let mut header = Header::new();

	header.push_entry(HeaderEntry::header_line("1.6".to_string())).unwrap();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	let ref_sequences = ReferenceSequences::new_random_with_length(vec![None;10]);

	for sequence in &ref_sequences.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	match params.mode {
		TestGenerationMode::Default => {
			generation_methods::default_generation(sam, bam, count, ref_sequences, header);
		}
		TestGenerationMode::Reference => {
			generation_methods::per_reference_generation(sam, bam);
		}
		TestGenerationMode::Bin => {
			generation_methods::per_bin_generation(sam, bam);
		}
		TestGenerationMode::ReferenceReadLength => {
			generation_methods::per_reference_length_generation(sam, bam);
		}
		TestGenerationMode::CIGAR => {
			generation_methods::cigar_generation(sam, bam);
		}
		TestGenerationMode::MapQ => {
			generation_methods::read_quality_generation(sam,bam);
		}
		TestGenerationMode::MapQPerRef => {
			generation_methods::read_quality_per_ref_generation(sam, bam);
		}
		TestGenerationMode::ReadLenSeq => {
			generation_methods::read_length_sequence_generation(sam, bam);
		}
		TestGenerationMode::ReadLenRef => {
			generation_methods::read_length_reference_generation(sam, bam);
		}
		TestGenerationMode::Unmapped => {
			generation_methods::unmapped_generation(sam, bam);
		}
		TestGenerationMode::RefLen => {
			generation_methods::ref_len_generation(sam,bam);
		}
		TestGenerationMode::Split => {
			generation_methods::split_read_generation(count, sam, bam);
		}
		TestGenerationMode::Mixed => {
			generation_methods::mixed_read_generation(count, sam, bam);
		}
		TestGenerationMode::SplitGap => {
			generation_methods::split_read_gap_generation(sam, bam);
		}
		TestGenerationMode::SplitCount => {
			generation_methods::split_read_count_generation(sam, bam);
		}
		TestGenerationMode::SplitTlen => {
			generation_methods::split_read_tlen_generation(sam, bam);
		}
	}
}

pub struct ReferenceSequences {
	sequences: Vec<(String, u32)>
}

impl ReferenceSequences {
	fn new_random_with_length(lengths: Vec<Option<u32>>) -> Self {
		let sequences: Vec<(String, u32)> = lengths.iter().map(|item|
			match item {
				None => {
					thread_rng().gen_range(1..=100000000u32)
				}
				Some(length) => {
					*length
				}
			}
		).map(|item| {

			let name = record_generation::generate_random_name_string(20);
			(name, item)
		}).collect();

		Self {
			sequences
		}
	}

	fn get_sequence_count(&self) -> usize {
		self.sequences.len()
	}

	fn get_sequence_length(&self, index: usize) -> Option<u32> {
		self.sequences.get(index).map(|(_a,b)| *b)
	}

	fn get_sequence_name(&self, index: usize) -> Option<&String> {
		self.sequences.get(index).map(|(a,_b)| a)
	}

	fn get_sequence(&self, index: usize) -> Option<&(String, u32)> {
		self.sequences.get(index)
	}

	fn get_random_index(&self) -> usize {
		thread_rng().gen_range(0..self.get_sequence_count())
	}

	fn get_random_name(&self) -> &String {
		 self.get_sequence_name(self.get_random_index()).unwrap()
	}

	fn get_random_reference(&self) -> (&(String, u32), usize) {
		let index = self.get_random_index();
		(self.get_sequence(index).unwrap(), index)
	}
}

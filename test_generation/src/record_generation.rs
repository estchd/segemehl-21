use bam::Record;
use bam::record::Flag;
use rand::{Rng, thread_rng};
use std::cmp::min;
use rand::distributions::Alphanumeric;
use ascii::AsciiString;
use std::str::FromStr;
use crate::{ReferenceSequences, Sequence};

pub fn generate_random_read(references: &ReferenceSequences, reference_id: Option<(u32, usize)>, start: Option<u32>, sequence_length: Option<u32>, length_on_reference: Option<u32>, mapq: Option<u8>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(true);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let (reference_length, index) = reference_id.unwrap_or_else(
		|| {
			let ((_, len), id) = references.get_random_reference();
			(*len, id)
		}
	);

	let start_pos = match start {
		None => {
			thread_rng().gen_range(1..=reference_length)
		}
		Some(start) => {
			start
		}
	};

	let available_space = reference_length - start_pos;

	let mapq = match mapq {
		None => {
			thread_rng().gen_range(0..255u8)
		},
		Some(mapq) => {
			mapq
		}
	};


	let seq_length = match sequence_length {
		None => {
			let seq_length_min = min(available_space, 10);
			let seq_length_max = min(available_space, 100);
			thread_rng().gen_range(seq_length_min..=seq_length_max)
		}
		Some(length) => {
			min(available_space, length)
		}
	};

	let sequence = Sequence::generate_random(Some(seq_length), length_on_reference);

	read_from_parts(&name, flags, Some(index as u32), Some(start_pos), Some(mapq), None, None, None, sequence)
}

pub fn generate_random_unmapped_read(sequence_length: Option<u32>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(false);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let seq_length = match sequence_length {
		None => {
			thread_rng().gen_range(10..=100)
		}
		Some(length) => {
			length
		}
	};

	let sequence = Sequence::generate_random(Some(seq_length), None);

	read_from_parts(&name, flags, None, None, None, None, None, None, sequence)
}

pub fn generate_random_only_matches(references: &ReferenceSequences, reference_id: Option<(u32, usize)>, start: Option<u32>, length_on_reference: Option<u32>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(true);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let (reference_length, index) = reference_id.unwrap_or_else(
		|| {
			let ((_, len), id) = references.get_random_reference();
			(*len, id)
		}
	);

	let start_pos = match start {
		None => {
			thread_rng().gen_range(1..=reference_length)
		}
		Some(start) => {
			start
		}
	};

	let mapq = thread_rng().gen_range(0..255u8);

	let sequence = Sequence::generate_random_only_matches(length_on_reference);

	read_from_parts(&name, flags, Some(index as u32), Some(start_pos), Some(mapq), None, None, None, sequence)
}

pub fn generate_random_only_insertions(references: &ReferenceSequences, reference_id: Option<(u32, usize)>, start: Option<u32>, length_on_reference: Option<u32>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(true);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let (reference_length, index) = reference_id.unwrap_or_else(
		|| {
			let ((_, len), id) = references.get_random_reference();
			(*len, id)
		}
	);

	let start_pos = match start {
		None => {
			thread_rng().gen_range(1..=reference_length)
		}
		Some(start) => {
			start
		}
	};

	let mapq = thread_rng().gen_range(0..255u8);

	let sequence = Sequence::generate_random_only_insertions(length_on_reference);

	read_from_parts(&name, flags, Some(index as u32), Some(start_pos), Some(mapq), None, None, None, sequence)
}

pub fn generate_random_only_deletions(references: &ReferenceSequences, reference_id: Option<(u32, usize)>, start: Option<u32>, length_on_reference: Option<u32>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(true);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let (reference_length, index) = reference_id.unwrap_or_else(
		|| {
			let ((_, len), id) = references.get_random_reference();
			(*len, id)
		}
	);

	let start_pos = match start {
		None => {
			thread_rng().gen_range(1..=reference_length)
		}
		Some(start) => {
			start
		}
	};

	let mapq = thread_rng().gen_range(0..255u8);

	let sequence = Sequence::generate_random_only_deletions(length_on_reference);

	read_from_parts(&name, flags, Some(index as u32), Some(start_pos), Some(mapq), None, None, None, sequence)
}

pub fn generate_random_only_skips(references: &ReferenceSequences, reference_id: Option<(u32, usize)>, start: Option<u32>, length_on_reference: Option<u32>) -> Record {
	let name = generate_random_name_string(20);
	let mut flags = Flag(0);

	flags.set_paired(false);
	flags.set_first_in_pair(false);
	flags.set_last_in_pair(false);
	flags.set_mapped(true);
	flags.set_duplicate(false);
	flags.set_mate_strand(false);
	flags.set_supplementary(false);

	let (reference_length, index) = reference_id.unwrap_or_else(
		|| {
			let ((_, len), id) = references.get_random_reference();
			(*len, id)
		}
	);

	let start_pos = match start {
		None => {
			thread_rng().gen_range(1..=reference_length)
		}
		Some(start) => {
			start
		}
	};

	let mapq = thread_rng().gen_range(0..255u8);

	let sequence = Sequence::generate_random_only_skips(length_on_reference);

	read_from_parts(&name, flags, Some(index as u32), Some(start_pos), Some(mapq), None, None, None, sequence)
}

pub fn generate_random_name_string(length: usize) -> String {
	thread_rng()
		.sample_iter(&Alphanumeric)
		.take(length)
		.map(char::from)
		.collect()
}

fn read_from_parts(
	name: &str,
	flags: Flag,
	ref_id: Option<u32>,
	start: Option<u32>,
	map_q: Option<u8>,
	mate_ref_id: Option<u32>,
	mate_start: Option<u32>,
	template_len: Option<u32>,
	sequence: Sequence
) -> Record {
	let name = AsciiString::from_str(&name).unwrap();
	let name: Vec<u8> = name.into();
	let ref_id = ref_id.map(|item| item as i32).unwrap_or(-1);
	let start = start.map(|item| item as i32).unwrap_or(-1);
	let mate_ref_id = mate_ref_id.map(|item| item as i32).unwrap_or(-1);
	let mate_start = mate_start.map(|item| item as i32).unwrap_or(-1);
	let template_len = template_len.map(|item| item as i32).unwrap_or(-1);

	let seq = sequence.get_sequence();
	let qualities = sequence.get_quality();
	let cigar = sequence.get_cigar();

	let mut read = Record::new();
	read.set_name(name);
	read.set_flag(flags.0);
	read.set_ref_id(ref_id);
	read.set_start(start);
	read.set_mapq(map_q.unwrap_or(255));
	read.set_seq_qual(seq, qualities).unwrap();

	if flags.is_mapped() {
		read.set_cigar(cigar).unwrap();
	}

	read.set_mate_ref_id(mate_ref_id);
	read.set_mate_start(mate_start);
	read.set_template_len(template_len);

	read
}

pub fn generate_random_quality() -> u8 {
	thread_rng().gen_range(0..=0x5D)
}

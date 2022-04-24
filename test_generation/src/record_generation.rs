use bam::Record;
use bam::record::Flag;
use rand::{Rng, thread_rng};
use std::cmp::{min};
use rand::distributions::Alphanumeric;
use ascii::AsciiString;
use std::str::FromStr;
use crate::{ReferenceSequences, Sequence};
use crate::util::split_random;

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

pub fn generate_random_split_reads(references: &ReferenceSequences, max_split_count: Option<usize>, gap_length: Option<u32>, split_count: Option<usize>, tlen: Option<u32>) -> Vec<Record> {
	let mut rand = thread_rng();

	let max_split_count = match max_split_count {
		None => {
			5
		}
		Some(max_count) => {
			max_count
		}
	};

	let split_count = match split_count {
		None => {
			rand.gen_range(1..=max_split_count)
		}
		Some(count) => {
			count
		}
	};

	let ((_, reference_length), reference_id) = references.get_random_reference();

	let record_count = split_count + 1;

	let gap_length_min = match gap_length {
		None => {
			50
		}
		Some(length) => {
			length
		}
	};

	let gap_length_max = match gap_length {
		None => {
			1000
		}
		Some(length) => {
			length
		}
	};

	let split_length_min = record_count * 50 + split_count * gap_length_min as usize;
	let split_length_max = record_count * 1000 + split_count * gap_length_max as usize;

	let template_length_min = min(*reference_length as usize, split_length_min);
	let template_length_max = min(*reference_length as usize, split_length_max);

	let template_length = match tlen {
		None => {
			rand.gen_range(template_length_min..=template_length_max)
		}
		Some(len) => {
			len as usize
		}
	};

	let (template_record_length, template_gap_length) = match gap_length {
		None => {
			split_random(template_length as u32, None, Some(40..=60))
		}
		Some(length) => {
			let gap_length = split_count * length as usize;
			let record_length = template_length - gap_length;

			(record_length as u32, gap_length as u32)
		}
	};

	let record_length = template_record_length / record_count as u32;
	let record_remainder = template_record_length % record_count as u32;

	let last_record_length = record_length + record_remainder;

	let gap_length = match gap_length {
		None => {
			template_gap_length / split_count as u32
		}
		Some(length) => {
			length
		}
	};

	let gap_remainder = template_gap_length % split_count as u32;

	let current_record_start_max = reference_length - template_length as u32;

	let mut current_record_start = rand.gen_range(0..current_record_start_max);
	let first_record_start = current_record_start;

	let mut flag = Flag(0);
	flag.set_mapped(true);
	flag.set_paired(true);
	flag.set_all_segments_aligned(true);
	flag.set_mate_strand(true);
	flag.set_mate_mapped(true);

	let record_name = generate_random_name_string(20);
	let record_ascii = AsciiString::from_ascii(record_name.as_str()).unwrap();
	let record_bytes: Vec<u8> = record_ascii.into();

	let mut records: Vec<Record> = vec![];

	for i in 0..split_count {
		let mut record = generate_random_read(references, Some((*reference_length, reference_id)), Some(current_record_start), None, Some(record_length),None);
		record.set_name(record_bytes.clone());

		if i == 0 {
			let mut first_flag = Flag(flag.0);
			first_flag.set_first_in_pair(true);

			record.set_flag(first_flag.0);
		}
		else {
			record.set_flag(flag.0);
		}

		let next_record_start = current_record_start + record_length + gap_length;

		record.set_mate_start(next_record_start as i32);
		record.set_mate_ref_id(reference_id as i32);
		record.set_template_len(template_length as i32);

		current_record_start = next_record_start;
		records.push(record);
	}

	current_record_start += gap_remainder;

	let mut last_record = generate_random_read(references, Some((*reference_length, reference_id)), Some(current_record_start), None, Some(last_record_length),None);

	let mut last_flag = Flag(flag.0);
	last_flag.set_last_in_pair(true);

	last_record.set_flag(last_flag.0);
	last_record.set_mate_start(first_record_start as i32);
	last_record.set_mate_ref_id(reference_id as i32);
	last_record.set_template_len(template_length as i32);
	last_record.set_name(record_bytes.clone());

	records.push(last_record);

	records
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
) -> Record
{
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

use std::cmp::min;
use bam::Header;
use bam::header::HeaderEntry;
use rand::{Rng, thread_rng};
use crate::ReferenceSequences;
use crate::files::Files;
use crate::record_generation::{generate_random_read, generate_random_unmapped_read};
use crate::util::split_random;

pub fn default_generation(sam_path: String, bam_path: String, count: usize, sequences: ReferenceSequences, header: Header) {
	let mut files = Files::from_path(sam_path,bam_path, header);

	for _ in 0..count {
		let record = crate::record_generation::generate_random_read(&sequences, None, None, None, None, None);
		files.write(&record);
	}

	files.flush();
}

pub fn ref_len_generation(sam_path: String, bam_path: String) {
	let mut cur_reference_length = 1000000;

	let mut reference_lengths = vec![Some(0u32); 10];

	for i in 0..10 {
		reference_lengths[i] = Some(cur_reference_length);
		cur_reference_length /= 2;
	}

	let references = ReferenceSequences::new_random_with_length(reference_lengths);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let sam_path = format!("{}_ref_len.sam", &sam_path[0..(sam_path.len() - 4)]);
	let bam_path = format!("{}_ref_len.bam", &bam_path[0..(bam_path.len() - 4)]);
	let mut files = Files::from_path(sam_path,bam_path, header.clone());
	files.flush();
}

pub fn per_reference_generation(sam_path: String, bam_path: String, count: usize, sequences: ReferenceSequences, header: Header) {
	for (id, (_, len)) in sequences.sequences.iter().enumerate() {
		let sam_path = format!("{}_reference_{}.sam", &sam_path[0..(sam_path.len() - 4)], id);
		let bam_path = format!("{}_reference_{}.bam", &bam_path[0..(bam_path.len() - 4)], id);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		for _ in 0..count {
			let record = crate::record_generation::generate_random_read(&sequences, Some((*len, id)), None, None, None, None);
			files.write(&record);
		}

		files.flush();
	}
}

pub fn per_bin_generation(sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![Some(1000000)]);
	let reference_length = references.get_sequence_length(0).unwrap();

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	for i in 0..10 {
		let sam_path = format!("{}_bin_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_bin_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let high = i % 2 == 0;
		let high_coverage = 2;
		let low_coverage = 1;

		let coverage = if high {
			high_coverage
		}
		else {
			low_coverage
		};

		let splits = i + 1;
		let split_length = reference_length / (splits * 2) as u32;

		let read_length = 100u32;
		let split_read_count = split_length / read_length;
		let last_read_length = split_length % read_length;

		for s in 0..splits {
			let mut cur_start = (s * split_length) * 2;

			for _ in 0..split_read_count {
				for _ in 0..coverage {
					let record = crate::record_generation::generate_random_read(&references, Some((reference_length, 0)), Some(cur_start), None, Some(read_length), None);
					files.write(&record);
				}
				cur_start += read_length;
			}
			if last_read_length != 0 {
				for _ in 0..coverage {
					let record = crate::record_generation::generate_random_read(&references, Some((reference_length, 0)), Some(cur_start), None, Some(last_read_length), None);
					files.write(&record);
				}
			}
		}
		files.flush();
	}
}

pub fn per_reference_length_generation(sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![Some(1000000)]);
	let reference_length = references.get_sequence_length(0).unwrap();

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	for i in 0..10 {
		let sam_path = format!("{}_reference_read_length_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_reference_read_length_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let total_length = 100000 / (i+1);
		let max_read_length = 100;

		let mut current_length = 0;

		while current_length < total_length {
			let read_length = thread_rng().gen_range(1..=min(total_length - current_length, max_read_length));

			let record = crate::record_generation::generate_random_read(&references, Some((reference_length, 0)), None, None, Some(read_length), None);
			files.write(&record);
			current_length += read_length;
		}

		files.flush();
	}
}

pub fn cigar_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length); 4]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	for i in 0..8 {
		let sam_path = format!("{}_cigar_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_cigar_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let high = i % 2 == 0;
		let high_coverage = 2;
		let low_coverage = 1;

		let (coverage, splits) = if high {
			(high_coverage, 1)
		}
		else {
			(low_coverage, 2)
		};

		let split_length = reference_length / (splits * 2) as u32;

		let read_length = 100u32;
		let split_read_count = split_length / read_length;
		let last_read_length = split_length % read_length;

		for s in 0..splits {
			if i == 0 || i == 1 {
				cigar_generation_matches(s, split_length, coverage, split_read_count, read_length, last_read_length, reference_length, &references, &mut files);
			}
			else if i == 2 || i == 3 {
				cigar_generation_insertions(s, split_length, coverage, split_read_count, read_length, last_read_length, reference_length, &references, &mut files);
			}
			else if i == 4 || i == 5 {
				cigar_generation_deletions(s, split_length, coverage, split_read_count, read_length, last_read_length, reference_length, &references, &mut files);
			}
			else {
				cigar_generation_skips(s, split_length, coverage, split_read_count, read_length, last_read_length, reference_length, &references, &mut files);
			}
		}
		files.flush();
	}
}

fn cigar_generation_matches(s: u32, split_length: u32, coverage: u32, split_read_count: u32, read_length: u32, last_read_length: u32, reference_length: u32, references: &ReferenceSequences, files: &mut Files) {
	let mut cur_start = (s * split_length) * 2;

	for _ in 0..split_read_count {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_matches(references, Some((reference_length, 0)), Some(cur_start), Some(read_length));
			files.write(&record);
		}
		cur_start += read_length;
	}
	if last_read_length != 0 {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_matches(references, Some((reference_length, 0)), Some(cur_start), Some(last_read_length));
			files.write(&record);
		}
	}
}

fn cigar_generation_insertions(s: u32, split_length: u32, coverage: u32, split_read_count: u32, read_length: u32, last_read_length: u32, reference_length: u32, references: &ReferenceSequences, files: &mut Files) {
	let mut cur_start = (s * split_length) * 2;

	for _ in 0..split_read_count {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_insertions(references, Some((reference_length, 0)), Some(cur_start), Some(read_length));
			files.write(&record);
		}
		cur_start += read_length;
	}
	if last_read_length != 0 {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_insertions(references, Some((reference_length, 0)), Some(cur_start), Some(last_read_length));
			files.write(&record);
		}
	}

}

fn cigar_generation_deletions(s: u32, split_length: u32, coverage: u32, split_read_count: u32, read_length: u32, last_read_length: u32, reference_length: u32, references: &ReferenceSequences, files: &mut Files) {
	let mut cur_start = (s * split_length) * 2;

	for _ in 0..split_read_count {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_deletions(references, Some((reference_length, 0)), Some(cur_start), Some(read_length));
			files.write(&record);
		}
		cur_start += read_length;
	}
	if last_read_length != 0 {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_deletions(references, Some((reference_length, 0)), Some(cur_start), Some(last_read_length));
			files.write(&record);
		}
	}
}

fn cigar_generation_skips(s: u32, split_length: u32, coverage: u32, split_read_count: u32, read_length: u32, last_read_length: u32, reference_length: u32, references: &ReferenceSequences, files: &mut Files) {
	let mut cur_start = (s * split_length) * 2;

	for _ in 0..split_read_count {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_skips(references, Some((reference_length, 0)), Some(cur_start), Some(read_length));
			files.write(&record);
		}
		cur_start += read_length;
	}
	if last_read_length != 0 {
		for _ in 0..coverage {
			let record = crate::record_generation::generate_random_only_skips(references, Some((reference_length, 0)), Some(cur_start), Some(last_read_length));
			files.write(&record);
		}
	}
}

pub fn read_quality_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length); 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	for i in 0..10 {
		let sam_path = format!("{}_mapq_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_mapq_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let splits = i + 1;
		let split_length = 255 as u8 / (splits * 2) as u8;

		let coverage = 2;

		for s in 0..splits {
			let mut cur_quality = (s * split_length) * 2;

			for _ in 0..split_length {
				for _ in 0..coverage {
					let record = crate::record_generation::generate_random_read(&references, Some((reference_length, i as usize)), None, None,None, Some(cur_quality));
					files.write(&record);
				}
				cur_quality += 1;
			}
		}
		files.flush();
	}
}

pub fn read_quality_per_ref_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length); 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let mut quality_range = 255 / 2;
	let middle_quality = 255u8 / 2;

	for i in 0..10 {
		let sam_path = format!("{}_mapq_per_ref_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_mapq_per_ref_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			None,
			None,
			Some(middle_quality.saturating_add(quality_range))
		);

		// TODO: Fix Boxplot generation
		// TODO: Generate Middle Reads

		files.write(&max_record);
		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			None,
			None,
			Some(middle_quality.saturating_sub(quality_range))
		);
		files.write(&min_record);

		files.flush();
		quality_range /= 2;
	}
}

pub fn read_length_sequence_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length); 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let max_length = 1000u32;

	let mut length_range = max_length / 2;
	let middle_length = max_length / 2;

	for i in 0..10 {
		let sam_path = format!("{}_read_len_seq_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_read_len_seq_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			Some(middle_length + length_range),
			None,
			None
		);

		// TODO: Fix Boxplot generation
		// TODO: Generate Middle Reads

		files.write(&max_record);
		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			Some(middle_length.saturating_sub(length_range)),
			None,
			None
		);
		files.write(&min_record);

		files.flush();
		length_range /= 2;
	}
}

pub fn read_length_reference_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length); 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let max_read_length = 1000u32;

	let mut length_range = max_read_length / 2;
	let middle_length = max_read_length / 2;

	for i in 0..10 {
		let sam_path = format!("{}_read_len_ref_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_read_len_ref_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			None,
			Some(middle_length + length_range),
			None
		);

		// TODO: Fix Boxplot generation
		// TODO: Generate Middle Reads

		files.write(&max_record);
		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			None,
			Some(middle_length.saturating_sub(length_range)),
			None
		);
		files.write(&min_record);

		files.flush();
		length_range /= 2;
	}
}

pub fn unmapped_generation(sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length)]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let read_count = 10000u32;

	let mut unmapped_read_percent = 100u32;

	for i in 0..10 {
		let sam_path = format!("{}_unmapped_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_unmapped_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let (unmapped_read_count, mapped_read_count) = split_random(read_count, Some(unmapped_read_percent), None);

		for _ in 0..unmapped_read_count {
			let record = generate_random_unmapped_read(None);
			files.write(&record);
		}

		for _ in 0..mapped_read_count {
			let record = generate_random_read(&references, Some((reference_length, 0)), None, None, None, None);
			files.write(&record);
		}

		files.flush();
		unmapped_read_percent /= 2;
	}
}

pub fn split_read_generation(_count: usize, sam_path: String, bam_path: String) {
	let reference_length = 1000000;
	let references = ReferenceSequences::new_random_with_length(vec![Some(reference_length)]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let read_count = 10000u32;

	let mut unmapped_read_percent = 100u32;

	for i in 0..10 {
		let sam_path = format!("{}_unmapped_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_unmapped_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let (unmapped_read_count, mapped_read_count) = split_random(read_count, Some(unmapped_read_percent), None);

		for _ in 0..unmapped_read_count {
			let record = generate_random_unmapped_read(None);
			files.write(&record);
		}

		for _ in 0..mapped_read_count {
			let record = generate_random_read(&references, Some((reference_length, 0)), None, None, None, None);
			files.write(&record);
		}

		files.flush();
		unmapped_read_percent /= 2;
	}
}

// TODO's:

// Benchmarks


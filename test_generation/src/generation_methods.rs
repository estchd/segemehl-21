use std::cmp::{max, min};
use bam::Header;
use bam::header::HeaderEntry;
use rand::{Rng, thread_rng};
use crate::ReferenceSequences;
use crate::files::Files;
use crate::record_generation::{generate_random_read, generate_random_split_reads, generate_random_unmapped_read};
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

pub fn per_reference_generation(sam_path: String, bam_path: String) {
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

	let mut read_count = 10000;

	for i in 0..10 {
		let sam_path = format!("{}_reference_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_reference_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		for _ in 0..read_count {
			let record = crate::record_generation::generate_random_read(&references, Some((reference_length, i)), None, None, Some(50), None);
			files.write(&record);
		}

		files.flush();
		read_count /= 2;
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

	let record_count = 1000;

	let middle_quality = 255u8 / 2;
	let mut quality_range = 255 / 2;

	for i in 0..10 {
		let sam_path = format!("{}_mapq_per_ref_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_mapq_per_ref_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let min_quality = middle_quality.saturating_sub(quality_range);
		let max_quality = middle_quality.saturating_add(quality_range);

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			None,
			None,
			Some(max_quality)
		);

		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			None,
			None,
			None,
			Some(min_quality)
		);

		files.write(&max_record);
		files.write(&min_record);

		for _ in 0..(record_count - 2) {
			let quality = thread_rng().gen_range(min_quality..=max_quality);

			let record = generate_random_read(
				&references,
				Some((reference_length, i)),
				None,
				None,
				None,
				Some(quality)
			);
			files.write(&record);
		}

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

	let record_count = 1000;

	let middle_length = 500u32;
	let mut length_range = 500u32;

	for i in 0..10 {
		let sam_path = format!("{}_read_len_seq_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_read_len_seq_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let min_length = middle_length.saturating_sub(length_range);
		let max_length = middle_length + length_range;

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			Some(max_length),
			None,
			None
		);

		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			Some(min_length),
			None,
			None
		);

		files.write(&max_record);
		files.write(&min_record);

		for _ in 0..(record_count - 2) {
			let length = thread_rng().gen_range(min_length..=max_length);

			let record = generate_random_read(
				&references,
				Some((reference_length, i)),
				Some(0),
				Some(length),
				None,
				None
			);
			files.write(&record);
		}

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

	let record_count = 1000;

	let middle_length = 500u32;
	let mut length_range = 500u32;

	for i in 0..10 {
		let sam_path = format!("{}_read_len_ref_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_read_len_ref_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let min_length = middle_length.saturating_sub(length_range);
		let max_length = middle_length + length_range;

		let max_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			None,
			Some(min_length),
			None
		);

		let min_record = generate_random_read(
			&references,
			Some((reference_length, i)),
			Some(0),
			None,
			Some(max_length),
			None
		);

		files.write(&max_record);
		files.write(&min_record);

		for _ in 0..(record_count - 2) {
			let length = thread_rng().gen_range(min_length..=max_length);

			let record = generate_random_read(
				&references,
				Some((reference_length, i)),
				Some(0),
				None,
				Some(length),
				None
			);
			files.write(&record);
		}

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

	let middle_unmapped_read_length = 500u32;
	let mut unmapped_read_length_spread = 500u32;

	for i in 0..10 {
		let sam_path = format!("{}_unmapped_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_unmapped_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let (unmapped_read_count, mapped_read_count) = split_random(read_count, Some(unmapped_read_percent), None);

		let min_unmapped_read_length = max(1, middle_unmapped_read_length - unmapped_read_length_spread);
		let max_unmapped_read_length = middle_unmapped_read_length + unmapped_read_length_spread;


		if unmapped_read_count >= 2 {
			let max_unmapped_read = generate_random_unmapped_read(Some(max_unmapped_read_length));
			let min_unmapped_read = generate_random_unmapped_read(Some(min_unmapped_read_length));

			files.write(&max_unmapped_read);
			files.write(&min_unmapped_read);
		}
		if unmapped_read_count > 2 {
			for _ in 0..(unmapped_read_count - 2) {
				let record_length = thread_rng().gen_range(min_unmapped_read_length..=max_unmapped_read_length);
				let record = generate_random_unmapped_read(Some(record_length));
				files.write(&record);
			}
		}

		for _ in 0..mapped_read_count {
			let record = generate_random_read(&references, Some((reference_length, 0)), None, None, None, None);
			files.write(&record);
		}

		files.flush();
		unmapped_read_percent /= 2;
		unmapped_read_length_spread /= 2;
	}
}

pub fn split_read_generation(count: usize, sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![None; 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let sam_path = format!("{}_split_{}.sam", &sam_path[0..(sam_path.len() - 4)], count);
	let bam_path = format!("{}_split_{}.bam", &bam_path[0..(bam_path.len() - 4)], count);
	let mut files = Files::from_path(sam_path,bam_path, header.clone());

	let mut remaining_count = count;

	loop {
		if remaining_count == 0 {
			break;
		}
		if remaining_count == 1 {
			let record = generate_random_read(&references, None, None, None, None, None);
			files.write(&record);
			break;
		}

		let records = generate_random_split_reads(&references, Some(remaining_count - 1), None, None, None);

		remaining_count -= records.len();

		for record in records {
			files.write(&record);
		}
	}
	files.flush();
}

pub fn mixed_read_generation(count: usize, sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![None; 10]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let sam_path = format!("{}_mixed_{}.sam", &sam_path[0..(sam_path.len() - 4)], count);
	let bam_path = format!("{}_mixed_{}.bam", &bam_path[0..(bam_path.len() - 4)], count);
	let mut files = Files::from_path(sam_path,bam_path, header.clone());

	let (split_record_count, single_record_count) = split_random(count as u32, Some(50), None);

	for _ in 0..single_record_count {
		let record = crate::record_generation::generate_random_read(&references, None, None, None, None, None);
		files.write(&record);
	}

	let mut remaining_count = split_record_count as usize;

	loop {
		if remaining_count == 0 {
			break;
		}
		if remaining_count == 1 {
			let record = generate_random_read(&references, None, None, None, None, None);
			files.write(&record);
			break;
		}

		let records = generate_random_split_reads(&references, Some(remaining_count - 1), None, None, None);

		remaining_count -= records.len();

		for record in records {
			files.write(&record);
		}
	}
	files.flush();
}

pub fn split_read_gap_generation(sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![Some(1000000)]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let read_count = 1000;

	let gap_length_middle = 5000;
	let mut gap_length_spread = 5000;

	for i in 0..10 {
		let sam_path = format!("{}_split_gap_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_split_gap_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let min_gap_length = gap_length_middle - gap_length_spread;
		let max_gap_length = gap_length_middle + gap_length_spread;

		let max_record = generate_random_split_reads(&references, Some(1), Some(max_gap_length), Some(1), None);
		let min_record = generate_random_split_reads(&references, Some(1), Some(min_gap_length), Some(1), None);

		for record in max_record {
			files.write(&record);
		}
		for record in min_record {
			files.write(&record);
		}

		for _ in 0..(read_count - 2) {
			let gap_length = thread_rng().gen_range(min_gap_length..=max_gap_length);

			let records = generate_random_split_reads(&references, Some(1), Some(gap_length), Some(1), None);

			for record in records {
				files.write(&record);
			}
		}

		files.flush();
		gap_length_spread /= 2;
	}
}

pub fn split_read_count_generation(sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![Some(1000000)]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let read_count = 1000;

	let split_count_middle = 10;

	for i in 0..10 {
		let sam_path = format!("{}_split_count_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_split_count_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let split_count_spread = 9 - i;

		let min_split_count = split_count_middle - split_count_spread;
		let max_split_count = split_count_middle + split_count_spread;

		let max_record = generate_random_split_reads(&references, None, None, Some(max_split_count), None);
		let min_record = generate_random_split_reads(&references, None, None, Some(min_split_count), None);

		for record in max_record {
			files.write(&record);
		}
		for record in min_record {
			files.write(&record);
		}

		for _ in 0..(read_count - 2) {
			let split_count = thread_rng().gen_range(min_split_count..=max_split_count);

			let records = generate_random_split_reads(&references, None, None, Some(split_count), None);

			for record in records {
				files.write(&record);
			}
		}

		files.flush();
	}
}

pub fn split_read_tlen_generation(sam_path: String, bam_path: String) {
	let references = ReferenceSequences::new_random_with_length(vec![Some(1000000)]);

	let mut header = Header::new();

	let program_vec: Vec<String> = std::env::args().collect();
	let program: String = program_vec.join(" ");

	header.push_entry(HeaderEntry::program(program)).unwrap();

	for sequence in &references.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	let read_count = 1000;

	let tlen_middle = 5000;
	let mut tlen_spread = 5000;

	for i in 0..10 {
		let sam_path = format!("{}_split_tlen_{}.sam", &sam_path[0..(sam_path.len() - 4)], i);
		let bam_path = format!("{}_split_tlen_{}.bam", &bam_path[0..(bam_path.len() - 4)], i);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		let min_tlen = tlen_middle - tlen_spread;
		let max_tlen = tlen_middle + tlen_spread;

		let max_record = generate_random_split_reads(&references, None, None, None, Some(max_tlen));
		let min_record = generate_random_split_reads(&references, None, None, None, Some(min_tlen));

		for record in max_record {
			files.write(&record);
		}
		for record in min_record {
			files.write(&record);
		}

		for _ in 0..(read_count - 2) {
			let tlen = thread_rng().gen_range(min_tlen..=max_tlen);

			let records = generate_random_split_reads(&references, None, None, None, Some(tlen));

			for record in records {
				files.write(&record);
			}
		}

		files.flush();
		tlen_spread /= 2;
	}
}

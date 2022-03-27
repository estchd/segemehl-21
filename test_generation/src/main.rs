use std::cmp::min;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::str::FromStr;
use ascii::AsciiString;
use bam::{BamWriter, Header, Record, RecordWriter, SamWriter};
use bam::header::HeaderEntry;
use bam::record::Flag;
use rand::distributions::{Alphanumeric};
use rand::{Rng, thread_rng};
use crate::command_line::{CommandLineParameters, TestGenerationMode};
use crate::sequence::{Sequence, SequenceBase};

mod command_line;
mod options;
mod sequence;
mod references;

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

	let ref_sequences = ReferenceSequences::new_random(10);

	for sequence in &ref_sequences.sequences {
		let entry = HeaderEntry::ref_sequence(sequence.0.clone(), sequence.1);
		header.push_entry(entry).unwrap();
	}

	match params.mode {
		TestGenerationMode::Reference => {
			per_reference_generation(sam, bam, count, ref_sequences, header);
		}
		TestGenerationMode::Default => {
			default_generation(sam, bam, count, ref_sequences, header);
		}
	}
}

fn default_generation(sam_path: String, bam_path: String, count: usize, sequences: ReferenceSequences, header: Header) {
	let mut files = Files::from_path(sam_path,bam_path, header);

	for _ in 0..count {
		let record = generate_random_read(&sequences, None);
		files.write(&record);
	}

	files.flush();
}

fn per_reference_generation(sam_path: String, bam_path: String, count: usize, sequences: ReferenceSequences, header: Header) {
	for (id, (_, len)) in sequences.sequences.iter().enumerate() {
		let sam_path = format!("{}_reference_{}.sam", &sam_path[0..(sam_path.len() - 4)], id);
		let bam_path = format!("{}_reference_{}.bam", &bam_path[0..(bam_path.len() - 4)], id);
		let mut files = Files::from_path(sam_path,bam_path, header.clone());

		for _ in 0..count {
			let record = generate_random_read(&sequences, Some((*len, id)));
			files.write(&record);
		}

		files.flush();
	}
}

pub struct Files {
	sam: SamWriter<BufWriter<File>>,
	bam: BamWriter<File>
}

impl Files {
	pub fn from_path<S: AsRef<Path>, B: AsRef<Path>>(sam_path: S, bam_path: B, header: Header) -> Self {
		let sam = SamWriter::from_path(sam_path, header.clone()).expect("Couldn't create output SAM file");
		let bam = BamWriter::from_path(bam_path, header.clone()).expect("Couldn't create output BAM file");

		Self {
			sam,
			bam
		}
	}

	pub fn write(&mut self, record: &Record) {
		self.sam.write(record).unwrap();
		self.bam.write(record).unwrap();
	}

	pub fn flush(&mut self) {
		self.sam.flush().expect("Couldn't write SAM file");
		self.bam.flush().expect("Couldn't write BAM file");
	}
}

pub struct ReferenceSequences {
	sequences: Vec<(String, u32)>
}

impl ReferenceSequences {
	fn new_random(num_references: usize) -> Self {
		let mut sequences = Vec::with_capacity(num_references);

		for _ in 0..num_references {
			let sequence_name = generate_random_name_string(20);
			let sequence_length = thread_rng().gen_range(1..=100000000u32);
			sequences.push((sequence_name, sequence_length));
		}

		Self{
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

fn generate_random_read(references: &ReferenceSequences, reference_id: Option<(u32,usize)>) -> Record {
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

	let pos = thread_rng().gen_range(1..=reference_length);
	let available_space = reference_length - pos;

	let mapq = thread_rng().gen_range(0..255u8);

	let seq_length_min = min(available_space, 10);
	let seq_length_max = min(available_space, 100);

	let seq_length = thread_rng().gen_range(seq_length_min..=seq_length_max) as usize;

	let sequence = Sequence::generate_random(seq_length);

	read_from_parts(&name, flags, Some(index as u32), Some(pos), mapq, None, None, None, sequence)
}

fn generate_random_name_string(length: usize) -> String {
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
	map_q: u8,
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
	read.set_mapq(map_q);
	read.set_seq_qual(seq, qualities).unwrap();
	read.set_cigar(cigar).unwrap();
	read.set_mate_ref_id(mate_ref_id);
	read.set_mate_start(mate_start);
	read.set_template_len(template_len);

	read
}

fn generate_random_quality() -> u8 {
	thread_rng().gen_range(0..=0x5D)
}

enum SequenceEntry {
	Base(SequenceBase),
	Skip(u32),
	Deletion(u32),
}

impl SequenceEntry {
	fn generate_random() -> Self {
		let mut rand = thread_rng();
		let entry_type = rand.gen_range(1..=100);
		if entry_type <= 70 {
			let base = Base::generate_random();
			let cigar = CIGAREntry::generate_random();
			let quality = generate_random_quality();

			let sequence_base = SequenceBase {
				base,
				cigar,
				quality
			};
			Self::Base(sequence_base)
		}
		else if entry_type <= 90 {
			let deletion_length = rand.gen_range(1..=10);
			Self::Deletion(deletion_length)
		}
		else {
			let skip_length = rand.gen_range(1..=10);
			Self::Skip(skip_length)
		}
	}
}

#[derive(Copy, Clone, Debug)]
enum Base {
	A,
	G,
	C,
	T
}

impl Base {
	fn generate_random() -> Self {
		let mut rand = thread_rng();
		let value = rand.gen_range(0..4);
		match value {
			0 => {
				Self::A
			},
			1 => {
				Self::G
			},
			2 => {
				Self::C
			},
			3 => {
				Self::T
			}
			_ => {
				panic!()
			}
		}
	}

	fn to_u8(self) -> u8{
		match self {
			Base::A => {
				0x41
			}
			Base::G => {
				0x47
			}
			Base::C => {
				0x43
			}
			Base::T => {
				0x54
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
enum CIGAREntry {
	M,
	I,
	S,
	Eq,
	X
}

impl CIGAREntry {
	fn generate_random() -> Self {
		let value = thread_rng().gen_range(0..=3);

		match value {
			0 => {
				Self::M
			},
			1 => {
				Self::I
			},
			2 => {
				Self::Eq
			},
			3 => {
				Self::X
			}
			_ => {
				panic!()
			}
		}
	}

	fn to_u8(self) -> u8 {
		match self {
			CIGAREntry::M => {
				0x4D
			}
			CIGAREntry::I => {
				0x49
			}
			CIGAREntry::S => {
				0x53
			}
			CIGAREntry::Eq => {
				0x3D
			}
			CIGAREntry::X => {
				0x58
			}
		}
	}
}
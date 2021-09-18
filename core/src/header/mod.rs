use std::convert::TryFrom;

use bam::header::{EntryType, HeaderLine};
use thiserror::Error;

use metadata_line::Metadata;
use program_line::Program;
use read_group_line::ReadGroup;
use reference_sequence_line::ReferenceSequence;
use crate::header::metadata_line::MetadataError;
use crate::header::program_line::ProgramError;
use crate::header::read_group_line::ReadGroupError;
use crate::header::reference_sequence_line::ReferenceSequenceError;

pub mod metadata_line;
pub mod program_line;
pub mod read_group_line;
pub mod reference_sequence_line;

#[derive(Error, Debug)]
pub enum HeaderError {
	#[error("more than one metadata line, line number: {line_number}")]
	DuplicateMetadataLine {
		line_number: usize
	},
	#[error("metadata line with invalid format, line number: {line_number}")]
	InvalidMetadataLine {
		line_number: usize,
		source: MetadataError
	},
	#[error("reference sequence line with invalid format, line number: {line_number}")]
	InvalidRefSequenceLine {
		line_number: usize,
		source: ReferenceSequenceError
	},
	#[error("read group line with invalid format, line number: {line_number}")]
	InvalidReadGroupLine {
		line_number: usize,
		source: ReadGroupError
	},
	#[error("program line with invalid format, line number: {line_number}")]
	InvalidProgramLine {
		line_number: usize,
		source: ProgramError
	}
}

pub struct Header {
	pub metadata: Option<Metadata>,
	pub reference_sequences: Vec<ReferenceSequence>,
	pub read_groups: Vec<ReadGroup>,
	pub programs: Vec<Program>
}

impl TryFrom<&bam::Header> for Header {
	type Error = HeaderError;

	fn try_from(header: &bam::Header) -> Result<Self,Self::Error> {
		let lines = header.lines();
		let entries = lines.enumerate().filter_map(|(line_number,line)| match line {
			HeaderLine::Entry(entry) => Some((line_number,entry)),
			HeaderLine::Comment(_) => None
		});

		let mut metadata: Option<Metadata> = None;
		let mut reference_sequences = Vec::<ReferenceSequence>::new();
		let mut read_groups = Vec::<ReadGroup>::new();
		let mut programs = Vec::<Program>::new();

		for (line_number, entry) in entries {
			match entry.entry_type() {
				EntryType::HeaderLine => {
					let header_line = Metadata::try_from_iter(entry.iter())
						.map_err(|err| HeaderError::InvalidMetadataLine{line_number, source: err})?;
					match metadata {
						None => {
							metadata = Some(header_line);
						}
						Some(_) => {
							return Err(HeaderError::DuplicateMetadataLine {line_number});
						}
					}
				},
				EntryType::RefSequence => {
					let reference_sequence = ReferenceSequence::try_from_iter(entry.iter())
						.map_err(|err| HeaderError::InvalidRefSequenceLine {
							line_number,
							source: err
						})?;
					reference_sequences.push(reference_sequence);
				},
				EntryType::ReadGroup => {
					let read_group = ReadGroup::try_from_iter(entry.iter())
						.map_err(|err| HeaderError::InvalidReadGroupLine {
							line_number,
							source: err
						})?;
					read_groups.push(read_group);
				},
				EntryType::Program => {
					let program = Program::try_from_iter(entry.iter())
						.map_err(|err| HeaderError::InvalidProgramLine {
							line_number,
							source: err
						})?;
					programs.push(program);
				}
			}
		}

		Ok(Header {
			metadata,
			reference_sequences,
			read_groups,
			programs
		})
	}
}

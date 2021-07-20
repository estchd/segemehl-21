use bam::header::{HeaderLine, EntryType};

use crate::header::metadata_line::format_version::FormatVersion;
use crate::header::metadata_line::grouping_order::GroupingOrder;
use crate::header::metadata_line::sorting_order::SortingOrder;
use crate::header::metadata_line::sub_sorting_order::SubSortingOrder;
use crate::header::reference_sequence_line::reference_sequence::ReferenceSequence;
use crate::header::metadata_line::metadata::Metadata;
use crate::header::read_group_line::read_group::ReadGroup;
use crate::header::program_line::program::Program;
use std::convert::TryFrom;

pub struct Header {
	pub version: FormatVersion,
	pub sorting: SortingOrder,
	pub grouping: GroupingOrder,
	pub sub_sorting: SubSortingOrder,
	pub reference_sequences: Vec<ReferenceSequence>,
	pub read_groups: Vec<ReadGroup>,
	pub programs: Vec<Program>
}

impl TryFrom<&bam::Header> for Header {
	type Error = ();

	fn try_from(header: &bam::Header) -> Result<Self,()> {
		let lines = header.lines();
		let entries = lines.filter_map(|line| match line {
			HeaderLine::Entry(entry) => Some(entry),
			HeaderLine::Comment(_) => None
		});

		let mut metadata: Option<Metadata> = None;
		let mut reference_sequences = Vec::<ReferenceSequence>::new();
		let mut read_groups = Vec::<ReadGroup>::new();
		let mut programs = Vec::<Program>::new();

		for entry in entries {
			match entry.entry_type() {
				EntryType::HeaderLine => {
					let header_line = Metadata::try_from_iter(entry.iter())?;
					match metadata {
						None => {
							metadata = Some(header_line);
						}
						Some(_) => {
							return Err(());
						}
					}
				},
				EntryType::RefSequence => {
					let reference_sequence = ReferenceSequence::try_from_iter(entry.iter())?;
					reference_sequences.push(reference_sequence);
				},
				EntryType::ReadGroup => {
					let read_group = ReadGroup::try_from_iter(entry.iter())?;
					read_groups.push(read_group);
				},
				EntryType::Program => {
					let program = Program::try_from_iter(entry.iter())?;
					programs.push(program);
				}
			}
		}

		let metadata = metadata.ok_or(())?;

		Ok(Header {
			version: metadata.format_version,
			sorting: metadata.sorting_order,
			grouping: metadata.grouping_order,
			sub_sorting: metadata.sub_sorting_order,
			reference_sequences,
			read_groups,
			programs
		})
	}
}
use std::convert::TryFrom;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum AlternateLocusError {
	#[error("alternate locus string contains no or more than one ':'")]
	InvalidRange,
	#[error("range string contains no or more than one '-', parsed: {parsed}")]
	InvalidRangeEnd {
		parsed: String
	},
	#[error("could not parse range start, parsed: {parsed}")]
	InvalidStart {
		parsed: String,
		source: ParseIntError
	},
	#[error("could not parse range end, parsed: {parsed}")]
	InvalidEnd {
		parsed: String,
		source: ParseIntError
	}
}

pub struct AlternateLocus {
	pub sequence: String,
	pub start: u32,
	pub end: u32
}

impl TryFrom<&str> for AlternateLocus {
	type Error = AlternateLocusError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let sequence_split: Vec<&str> = value.splitn(2, ":").collect();

		if sequence_split.len() != 2 {return Err(
			AlternateLocusError::InvalidRange
		)};

		let sequence = sequence_split[0].to_string();

		let range_split: Vec<&str> = sequence_split[1].splitn(2, "-").collect();

		if range_split.len() != 2 {return Err(
			AlternateLocusError::InvalidRangeEnd {
				parsed: sequence_split[1].to_string()
			}
		)};

		let start = range_split[0].parse::<u32>()
			.map_err(|err| AlternateLocusError::InvalidStart {
				parsed: range_split[0].to_string(),
				source: err
			})?;
		let end = range_split[1].parse::<u32>()
			.map_err(|err| AlternateLocusError::InvalidEnd {
				parsed: range_split[1].to_string(),
				source: err
			})?;
		
		Ok(AlternateLocus {
			sequence,
			start,
			end
		})
	}
}

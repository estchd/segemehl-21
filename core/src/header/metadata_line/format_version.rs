use thiserror::Error;

use std::convert::TryFrom;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum FormatVersionError {
	#[error("version number string contains no or more than one '.'")]
	InvalidFormat,
	#[error("could not parse major version number, parsed: {major_value}")]
	InvalidMajor {
		major_value: String,
		source: ParseIntError
	},
	#[error("could not parse minor version number, parsed: {minor_value}")]
	InvalidMinor {
		minor_value: String,
		source: ParseIntError
	}
}

pub struct FormatVersion {
	pub major: u64,
	pub minor: u64,
}

impl TryFrom<&str> for FormatVersion {
	type Error = FormatVersionError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let split_by_dot: Vec<&str> = value.split(".").collect();

		if split_by_dot.len() != 2 {return Err(FormatVersionError::InvalidFormat)};

		let major = split_by_dot[0].parse::<u64>().map_err(|err| FormatVersionError::InvalidMajor {
			major_value: split_by_dot[0].to_string(),
			source: err
		})?;
		let minor = split_by_dot[1].parse::<u64>().map_err(|err| FormatVersionError::InvalidMinor {
			minor_value: split_by_dot[1].to_string(),
			source: err
		})?;

		Ok(FormatVersion {
			major,
			minor
		})
	}
}

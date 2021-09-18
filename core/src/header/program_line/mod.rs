use std::convert::TryFrom;
use thiserror::Error;

use bam::header::Tag;
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum ProgramError {
	#[error("no program identifier tag")]
	NoProgramIdentifier,
	#[error("more than one program identifier tag, tag number: {tag_number}")]
	DuplicateProgramIdentifier {
		tag_number: usize
	},
	#[error("more than one program name tag, tag number: {tag_number}")]
	DuplicateProgramName {
		tag_number: usize
	},
	#[error("more than one command line tag, tag number: {tag_number}")]
	DuplicateCommandLine {
		tag_number: usize
	},
	#[error("more than one previous program tag, tag number: {tag_number}")]
	DuplicatePreviousProgram {
		tag_number: usize
	},
	#[error("more than one description tag, tag number: {tag_number}")]
	DuplicateDescription {
		tag_number: usize
	},
	#[error("more than one version tag, tag number: {tag_number}")]
	DuplicateVersion {
		tag_number: usize
	},
	#[error("tag with invalid tag name or content, tag number: {tag_number}")]
	InvalidTag {
		tag_number: usize,
		source: ProgramTagError
	}
}

pub struct Program {
	pub identifier: String,
	pub name: Option<String>,
	pub command_line: Option<String>,
	pub previous_program: Option<String>,
	pub description: Option<String>,
	pub version: Option<String>
}

impl Program {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<Program, ProgramError>{
		let mut program_identifier: Option<String> = None;
		let mut program_name: Option<String> = None;
		let mut program_command_line: Option<String> = None;
		let mut program_previous_program: Option<String> = None;
		let mut program_description: Option<String> = None;
		let mut program_version: Option<String> = None;

		for (tag_number, tag) in iter.enumerate() {
			let program_tag = ProgramTag::try_from(tag)
				.map_err(|err| ProgramError::InvalidTag {
					tag_number,
					source: err
				})?;

			match program_tag {
				ProgramTag::Identifier(identifier) => {
					match program_identifier {
						None => {
							program_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(ProgramError::DuplicateProgramIdentifier {
								tag_number
							});
						}
					}
				}
				ProgramTag::Name(name) => {
					match program_name {
						None => {
							program_name = Some(name);
						}
						Some(_) => {
							return Err(ProgramError::DuplicateProgramName {
								tag_number
							});
						}
					}
				}
				ProgramTag::CommandLine(command_line) => {
					match program_command_line {
						None => {
							program_command_line = Some(command_line);
						}
						Some(_) => {
							return Err(ProgramError::DuplicateCommandLine {
								tag_number
							});
						}
					}
				}
				ProgramTag::PreviousProgram(previous) => {
					match program_previous_program {
						None => {
							program_previous_program = Some(previous);
						}
						Some(_) => {
							return Err(ProgramError::DuplicatePreviousProgram {
								tag_number
							});
						}
					}
				}
				ProgramTag::Description(description) => {
					match program_description {
						None => {
							program_description = Some(description);
						}
						Some(_) => {
							return Err(ProgramError::DuplicateDescription {
								tag_number
							});
						}
					}
				}
				ProgramTag::Version(version) => {
					match program_version {
						None => {
							program_version = Some(version);
						}
						Some(_) => {
							return Err(ProgramError::DuplicateVersion {
								tag_number
							});
						}
					}
				}
			}
		}

		let identifier = program_identifier.ok_or(ProgramError::NoProgramIdentifier)?;

		Ok(Program {
			identifier,
			name: program_name,
			command_line: program_command_line,
			previous_program: program_previous_program,
			description: program_description,
			version: program_version
		})
	}
}

#[derive(Error, Debug)]
pub enum ProgramTagError {
	#[error("tag name is not valid UTF-8, name values: {name_values:?}")]
	InvalidTagNameFormat {
		name_values: Vec<u8>,
		source: FromUtf8Error
	},
	#[error("tag name is not valid for a program line, name: {name}")]
	InvalidTagName {
		name: String
	},
}

pub enum ProgramTag {
	Identifier(String),
	Name(String),
	CommandLine(String),
	PreviousProgram(String),
	Description(String),
	Version(String)
}

impl TryFrom<&Tag> for ProgramTag {
	type Error = ProgramTagError;

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec.clone()).map_err(|err| ProgramTagError::InvalidTagNameFormat {
			name_values: name_vec,
			source: err
		})?;

		match name.as_str() {
			"ID" => Ok(Self::Identifier(value.value().to_string())),
			"PN" => Ok(Self::Name(value.value().to_string())),
			"CL" => Ok(Self::CommandLine(value.value().to_string())),
			"PP" => Ok(Self::PreviousProgram(value.value().to_string())),
			"DS" => Ok(Self::Description(value.value().to_string())),
			"VN" => Ok(Self::Version(value.value().to_string())),
			_ => Err(ProgramTagError::InvalidTagName {
				name
			})
		}
	}
}

use std::convert::TryFrom;
use bam::header::Tag;

pub struct Program {
	pub identifier: String,
	pub name: Option<String>,
	pub command_line: Option<String>,
	pub previous_program: Option<String>,
	pub description: Option<String>,
	pub version: Option<String>
}

impl Program {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<Program, ()>{
		let mut program_identifier: Option<String> = None;
		let mut program_name: Option<String> = None;
		let mut program_command_line: Option<String> = None;
		let mut program_previous_program: Option<String> = None;
		let mut program_description: Option<String> = None;
		let mut program_version: Option<String> = None;

		for tag in iter {
			let program_tag = ProgramTag::try_from(tag)?;

			match program_tag {
				ProgramTag::Identifier(identifier) => {
					match program_identifier {
						None => {
							program_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ProgramTag::Name(name) => {
					match program_name {
						None => {
							program_name = Some(name);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ProgramTag::CommandLine(command_line) => {
					match program_command_line {
						None => {
							program_command_line = Some(command_line);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ProgramTag::PreviousProgram(previous) => {
					match program_previous_program {
						None => {
							program_previous_program = Some(previous);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ProgramTag::Description(description) => {
					match program_description {
						None => {
							program_description = Some(description);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ProgramTag::Version(version) => {
					match program_version {
						None => {
							program_version = Some(version);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
			}
		}

		let identifier = program_identifier.ok_or(())?;

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

pub enum ProgramTag {
	Identifier(String),
	Name(String),
	CommandLine(String),
	PreviousProgram(String),
	Description(String),
	Version(String)
}

impl TryFrom<&Tag> for ProgramTag {
	type Error = ();

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec).map_err(|_| ())?;

		match name.as_str() {
			"ID" => Ok(Self::Identifier(value.value().to_string())),
			"PN" => Ok(Self::Name(value.value().to_string())),
			"CL" => Ok(Self::CommandLine(value.value().to_string())),
			"PP" => Ok(Self::PreviousProgram(value.value().to_string())),
			"DS" => Ok(Self::Description(value.value().to_string())),
			"VN" => Ok(Self::Version(value.value().to_string())),
			_ => Err(())
		}
	}
}
use std::convert::TryFrom;
use thiserror::Error;

use bam::header::Tag;

use crate::header::read_group_line::platform::{Platform, PlatformError};
use std::string::FromUtf8Error;
use std::num::ParseIntError;

pub mod platform;

#[derive(Error, Debug)]
pub enum ReadGroupError {
	#[error("no read group identifier tag")]
	NoIdentifier,
	#[error("more than one read group identifier tag, tag number: {tag_number}")]
	DuplicateIdentifier {
		tag_number: usize
	},
	#[error("more than one read group barcode tag, tag number: {tag_number}")]
	DuplicateBarcode {
		tag_number: usize
	},
	#[error("more than one read group sequencing center tag, tag number: {tag_number}")]
	DuplicateSequencingCenter {
		tag_number: usize
	},
	#[error("more than one read group description tag, tag number: {tag_number}")]
	DuplicateDescription {
		tag_number: usize
	},
	#[error("more than one read group date tag, tag number: {tag_number}")]
	DuplicateDate {
		tag_number: usize
	},
	#[error("more than one read group flow order tag, tag number: {tag_number}")]
	DuplicateFlowOrder {
		tag_number: usize
	},
	#[error("more than one read group key sequence tag, tag number: {tag_number}")]
	DuplicateKeySequence {
		tag_number: usize
	},
	#[error("more than one read group library tag, tag number: {tag_number}")]
	DuplicateLibrary {
		tag_number: usize
	},
	#[error("more than one read group program tag, tag number: {tag_number}")]
	DuplicateProgram {
		tag_number: usize
	},
	#[error("more than one read group median insert size tag, tag number: {tag_number}")]
	DuplicateMedianInsertSize {
		tag_number: usize
	},
	#[error("more than one read group platform tag, tag number: {tag_number}")]
	DuplicatePlatform {
		tag_number: usize
	},
	#[error("more than one read group platform model tag, tag number: {tag_number}")]
	DuplicatePlatformModel {
		tag_number: usize
	},
	#[error("more than one read group platform unit tag, tag number: {tag_number}")]
	DuplicatePlatformUnit {
		tag_number: usize
	},
	#[error("more than one read group sample tag, tag number: {tag_number}")]
	DuplicateSample {
		tag_number: usize
	},
	#[error("tag with invalid tag name or content, tag number: {tag_number}")]
	InvalidTag {
		tag_number: usize,
		source: ReadGroupTagError
	}
}

pub struct ReadGroup {
	pub identifier: String,
	pub barcode: Option<String>,
	pub sequencing_center: Option<String>,
	pub description: Option<String>,
	pub date: Option<String>,
	pub flow_order: Option<String>,
	pub key_sequence: Option<String>,
	pub library: Option<String>,
	pub program: Option<String>,
	pub median_insert_size: Option<u128>,
	pub platform: Option<Platform>,
	pub platform_model: Option<String>,
	pub platform_unit: Option<String>,
	pub sample: Option<String>
}

impl ReadGroup {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<ReadGroup, ReadGroupError> {
		let mut read_group_identifier: Option<String> = None;
		let mut read_group_barcode: Option<String> = None;
		let mut read_group_sequencing_center: Option<String> = None;
		let mut read_group_description: Option<String> = None;
		let mut read_group_date: Option<String> = None;
		let mut read_group_flow_order: Option<String> = None;
		let mut read_group_key_sequence: Option<String> = None;
		let mut read_group_library: Option<String> = None;
		let mut read_group_program: Option<String> = None;
		let mut read_group_median_insert_size: Option<u128> = None;
		let mut read_group_platform: Option<Platform> = None;
		let mut read_group_platform_model: Option<String> = None;
		let mut read_group_platform_unit: Option<String> = None;
		let mut read_group_sample: Option<String> = None;

		for (tag_number, tag) in iter.enumerate() {
			let read_group_tag = ReadGroupTag::try_from(tag)
				.map_err(|err| ReadGroupError::InvalidTag {
					tag_number,
					source: err
				})?;

			match read_group_tag {
				ReadGroupTag::Identifier(identifier) => {
					match read_group_identifier {
						None => {
							read_group_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateIdentifier {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Barcode(barcode) => {
					match read_group_barcode {
						None => {
							read_group_barcode = Some(barcode);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateBarcode {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::SequencingCenter(center) => {
					match read_group_sequencing_center {
						None => {
							read_group_sequencing_center = Some(center);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateSequencingCenter {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Description(description) => {
					match read_group_description {
						None => {
							read_group_description = Some(description);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateDescription {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Date(date) => {
					match read_group_date {
						None => {
							read_group_date = Some(date);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateDate {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::FlowOrder(order) => {
					match read_group_flow_order {
						None => {
							read_group_flow_order = Some(order);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateFlowOrder {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::KeySequence(sequence) => {
					match read_group_key_sequence {
						None => {
							read_group_key_sequence = Some(sequence);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateKeySequence {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Library(library) => {
					match read_group_library {
						None => {
							read_group_library = Some(library);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateLibrary {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Program(program) => {
					match read_group_program {
						None => {
							read_group_program = Some(program);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateProgram {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::MedianInsertSize(size) => {
					match read_group_median_insert_size {
						None => {
							read_group_median_insert_size = Some(size);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateMedianInsertSize {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Platform(platform) => {
					match read_group_platform {
						None => {
							read_group_platform = Some(platform);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicatePlatform {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::PlatformModel(model) => {
					match read_group_platform_model {
						None => {
							read_group_platform_model = Some(model);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicatePlatformModel {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::PlatformUnit(unit) => {
					match read_group_platform_unit {
						None => {
							read_group_platform_unit = Some(unit);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicatePlatformUnit {
								tag_number
							});
						}
					}
				}
				ReadGroupTag::Sample(sample) => {
					match read_group_sample {
						None => {
							read_group_sample = Some(sample);
						}
						Some(_) => {
							return Err(ReadGroupError::DuplicateSample {
								tag_number
							});
						}
					}
				}
			}
		}

		let identifier = read_group_identifier.ok_or(ReadGroupError::NoIdentifier)?;

		Ok(ReadGroup {
			identifier,
			barcode: read_group_barcode,
			sequencing_center: read_group_sequencing_center,
			description: read_group_description,
			date: read_group_date,
			flow_order: read_group_flow_order,
			key_sequence: read_group_key_sequence,
			library: read_group_library,
			program: read_group_program,
			median_insert_size: read_group_median_insert_size,
			platform: read_group_platform,
			platform_model: read_group_platform_model,
			platform_unit: read_group_platform_unit,
			sample: read_group_sample
		})
	}
}

#[derive(Error, Debug)]
pub enum ReadGroupTagError {
	#[error("tag name is not valid UTF-8, name values: {name_values:?}")]
	InvalidTagNameFormat {
		name_values: Vec<u8>,
		source: FromUtf8Error
	},
	#[error("tag name is not valid for a read group line, name: {name}")]
	InvalidTagName {
		name: String
	},
	#[error("median insert size tag with invalid content, tag content: {content}")]
	InvalidMedianInsertSize {
		content: String,
		source: ParseIntError
	},
	#[error("platform tag with invalid content, tag content: {content}")]
	InvalidPlatform {
		content: String,
		source: PlatformError
	}
}

pub enum ReadGroupTag {
	Identifier(String),
	Barcode(String),
	SequencingCenter(String),
	Description(String),
	Date(String),
	FlowOrder(String),
	KeySequence(String),
	Library(String),
	Program(String),
	MedianInsertSize(u128),
	Platform(Platform),
	PlatformModel(String),
	PlatformUnit(String),
	Sample(String)
}

impl TryFrom<&Tag> for ReadGroupTag {
	type Error = ReadGroupTagError;

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec.clone()).map_err(|err| ReadGroupTagError::InvalidTagNameFormat {
			name_values: name_vec,
			source: err
		})?;

		match name.as_str() {
			"ID" => Ok(Self::Identifier(value.value().to_string())),
			"BC" => Ok(Self::Barcode(value.value().to_string())),
			"CN" => Ok(Self::SequencingCenter(value.value().to_string())),
			"DS" => Ok(Self::Description(value.value().to_string())),
			"DT" => Ok(Self::Date(value.value().to_string())),
			"FO" => Ok(Self::FlowOrder(value.value().to_string())),
			"KS" => Ok(Self::KeySequence(value.value().to_string())),
			"LB" => Ok(Self::Library(value.value().to_string())),
			"PG" => Ok(Self::Program(value.value().to_string())),
			"PI" => {
				let size = value.value().parse::<u128>().map_err(|err| ReadGroupTagError::InvalidMedianInsertSize {
					content: value.value().to_string(),
					source: err
				})?;
				Ok(Self::MedianInsertSize(size))
			},
			"PL" => {
				let platform = Platform::try_from(value.value())
					.map_err(|err| ReadGroupTagError::InvalidPlatform {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::Platform(platform))
			},
			"PM" => Ok(Self::PlatformModel(value.value().to_string())),
			"PU" => Ok(Self::PlatformUnit(value.value().to_string())),
			"SM" => Ok(Self::Sample(value.value().to_string())),
			_ => Err(ReadGroupTagError::InvalidTagName {
				name
			})
		}
	}
}

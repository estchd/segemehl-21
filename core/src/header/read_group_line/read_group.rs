

use crate::header::read_group_line::platform::Platform;
use std::convert::TryFrom;
use bam::header::Tag;

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
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<ReadGroup, ()> {
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

		for tag in iter {
			let read_group_tag = ReadGroupTag::try_from(tag)?;

			match read_group_tag {
				ReadGroupTag::Identifier(identifier) => {
					match read_group_identifier {
						None => {
							read_group_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Barcode(barcode) => {
					match read_group_barcode {
						None => {
							read_group_barcode = Some(barcode);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::SequencingCenter(center) => {
					match read_group_sequencing_center {
						None => {
							read_group_sequencing_center = Some(center);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Description(description) => {
					match read_group_description {
						None => {
							read_group_description = Some(description);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Date(date) => {
					match read_group_date {
						None => {
							read_group_date = Some(date);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::FlowOrder(order) => {
					match read_group_flow_order {
						None => {
							read_group_flow_order = Some(order);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::KeySequence(sequence) => {
					match read_group_key_sequence {
						None => {
							read_group_key_sequence = Some(sequence);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Library(library) => {
					match read_group_library {
						None => {
							read_group_library = Some(library);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Program(program) => {
					match read_group_program {
						None => {
							read_group_program = Some(program);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::MedianInsertSize(size) => {
					match read_group_median_insert_size {
						None => {
							read_group_median_insert_size = Some(size);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Platform(platform) => {
					match read_group_platform {
						None => {
							read_group_platform = Some(platform);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::PlatformModel(model) => {
					match read_group_platform_model {
						None => {
							read_group_platform_model = Some(model);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::PlatformUnit(unit) => {
					match read_group_platform_unit {
						None => {
							read_group_platform_unit = Some(unit);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReadGroupTag::Sample(sample) => {
					match read_group_sample {
						None => {
							read_group_sample = Some(sample);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
			}
		}

		let identifier = read_group_identifier.ok_or(())?;

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
	type Error = ();

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec).map_err(|_| ())?;

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
				let size = value.value().parse::<u128>().map_err(|_| ())?;
				Ok(Self::MedianInsertSize(size))
			},
			"PL" => {
				let platform = Platform::try_from(value.value())?;
				Ok(Self::Platform(platform))
			},
			"PM" => Ok(Self::PlatformModel(value.value().to_string())),
			"PU" => Ok(Self::PlatformUnit(value.value().to_string())),
			"SM" => Ok(Self::Sample(value.value().to_string())),
			_ => Err(())
		}
	}
}
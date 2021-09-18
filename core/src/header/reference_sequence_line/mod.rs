use std::convert::TryFrom;
use thiserror::Error;

use bam::header::Tag;

use crate::header::reference_sequence_line::alternate_locus::{AlternateLocus, AlternateLocusError};
use crate::header::reference_sequence_line::molecule_topology::{MoleculeTopology, MoleculeTopologyError};
use std::string::FromUtf8Error;
use std::num::ParseIntError;

pub mod alternate_locus;
pub mod molecule_topology;

#[derive(Error, Debug)]
pub enum ReferenceSequenceError {
	#[error("no reference sequence name tag")]
	NoName,
	#[error("no reference sequence length tag")]
	NoLength,
	#[error("more than one reference sequence name tag, tag number: {tag_number}")]
	DuplicateName {
		tag_number: usize
	},
	#[error("more than one reference sequence length tag, tag number: {tag_number}")]
	DuplicateLength {
		tag_number: usize
	},
	#[error("more than one reference sequence alternate locus tag, tag number: {tag_number}")]
	DuplicateAlternateLocus {
		tag_number: usize
	},
	#[error("more than one reference sequence alternate names tag, tag number: {tag_number}")]
	DuplicateAlternateNames {
		tag_number: usize
	},
	#[error("more than one reference sequence assembly tag, tag number: {tag_number}")]
	DuplicateAssembly {
		tag_number: usize
	},
	#[error("more than one reference sequence description tag, tag number: {tag_number}")]
	DuplicateDescription {
		tag_number: usize
	},
	#[error("more than one reference sequence checksum tag, tag number: {tag_number}")]
	DuplicateChecksum {
		tag_number: usize
	},
	#[error("more than one reference sequence species tag, tag number: {tag_number}")]
	DuplicateSpecies {
		tag_number: usize
	},
	#[error("more than one reference sequence topology tag, tag number: {tag_number}")]
	DuplicateTopology {
		tag_number: usize
	},
	#[error("more than one reference sequence uri tag, tag number: {tag_number}")]
	DuplicateUri {
		tag_number: usize
	},
	#[error("tag with invalid tag name or content, tag number: {tag_number}")]
	InvalidTag {
		tag_number: usize,
		source: ReferenceSequenceTagError
	}
}

pub struct ReferenceSequence {
	pub name: String,
	pub length: u32,
	pub alternate_locus: Option<AlternateLocus>,
	pub alternate_names: Option<Vec<String>>,
	pub assembly: Option<String>,
	pub description: Option<String>,
	pub checksum: Option<String>,
	pub species: Option<String>,
	pub topology: MoleculeTopology,
	pub uri: Option<String>
}

impl ReferenceSequence {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<ReferenceSequence, ReferenceSequenceError> {
		let mut sequence_name: Option<String> = None;
		let mut sequence_length: Option<u32> = None;
		let mut alternate_locus: Option<AlternateLocus> = None;
		let mut alternate_names: Option<Vec<String>> = None;
		let mut assembly_identifier: Option<String> = None;
		let mut sequence_description: Option<String> = None;
		let mut sequence_checksum: Option<String> = None;
		let mut sequence_species: Option<String> = None;
		let mut molecule_topology: Option<MoleculeTopology> = None;
		let mut sequence_uri: Option<String> = None;

		for (tag_number, tag) in iter.enumerate() {
			let tag = ReferenceSequenceTag::try_from(tag)
				.map_err(|err| ReferenceSequenceError::InvalidTag {
					tag_number,
					source: err
				})?;

			match tag {
				ReferenceSequenceTag::SequenceName(name) => {
					match sequence_name {
						None => {
							sequence_name = Some(name);
						},
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateName {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::SequenceLength(length) => {
					match sequence_length {
						None => {
							sequence_length = Some(length);
						},
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateLength {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::AlternateLocus(locus) => {
					match alternate_locus {
						None => {
							alternate_locus = Some(locus);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateAlternateLocus {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::AlternateNames(names) => {
					match alternate_names {
						None => {
							alternate_names = Some(names);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateAlternateNames {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::AssemblyIdentifier(identifier) => {
					match assembly_identifier {
						None => {
							assembly_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateAssembly {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::Description(description) => {
					match sequence_description {
						None => {
							sequence_description = Some(description);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateDescription {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::Checksum(checksum) => {
					match sequence_checksum {
						None => {
							sequence_checksum = Some(checksum);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateChecksum {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::Species(species) => {
					match sequence_species {
						None => {
							sequence_species = Some(species);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateSpecies {
								tag_number
							});
						}
					}
				}
				ReferenceSequenceTag::MoleculeTopology(topology) => {
					match molecule_topology {
						None => {
							molecule_topology = Some(topology);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateTopology {
								tag_number
							})
						}
					}
				}
				ReferenceSequenceTag::Uri(uri) => {
					match sequence_uri {
						None => {
							sequence_uri = Some(uri);
						}
						Some(_) => {
							return Err(ReferenceSequenceError::DuplicateUri {
								tag_number
							})
						}
					}
				}
			}
		}

		let name = sequence_name.ok_or(
			ReferenceSequenceError::NoName
		)?;
		let length = sequence_length.ok_or(
			ReferenceSequenceError::NoLength
		)?;
		let topology = molecule_topology.unwrap_or(MoleculeTopology::Linear);

		Ok(ReferenceSequence {
			name,
			length,
			alternate_locus,
			alternate_names,
			assembly: assembly_identifier,
			description: sequence_description,
			checksum: sequence_checksum,
			species: sequence_species,
			topology,
			uri: sequence_uri
		})
	}
}

#[derive(Error, Debug)]
pub enum ReferenceSequenceTagError {
	#[error("tag name is not valid UTF-8, name values: {name_values:?}")]
	InvalidTagNameFormat {
		name_values: Vec<u8>,
		source: FromUtf8Error
	},
	#[error("tag name is not valid for a reference sequence line, name: {name}")]
	InvalidTagName {
		name: String
	},
	#[error("length tag with invalid content, tag content: {content}")]
	InvalidLength {
		content: String,
		source: ParseIntError
	},
	#[error("alternate locus tag with invalid content, tag content: {content}")]
	InvalidAlternateLocus {
		content: String,
		source: AlternateLocusError
	},
	#[error("molecule topology tag with invalid content, tag content: {content}")]
	InvalidTopology {
		content: String,
		source: MoleculeTopologyError
	}
}

pub enum ReferenceSequenceTag {
	SequenceName(String),
	SequenceLength(u32),
	AlternateLocus(AlternateLocus),
	AlternateNames(Vec<String>),
	AssemblyIdentifier(String),
	Description(String),
	Checksum(String),
	Species(String),
	MoleculeTopology(MoleculeTopology),
	Uri(String)
}

impl TryFrom<&Tag> for ReferenceSequenceTag {
	type Error = ReferenceSequenceTagError;

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec.clone())
			.map_err(|err| ReferenceSequenceTagError::InvalidTagNameFormat {
				name_values: name_vec,
				source: err
			})?;
		match name.as_str() {
			"SN" => Ok(Self::SequenceName(value.value().to_string())),
			"LN" => {
				let length = value.value().parse::<u32>()
					.map_err(|err| ReferenceSequenceTagError::InvalidLength {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::SequenceLength(length))
			},
			"AH" => {
				let alternate_locus = AlternateLocus::try_from(value.value())
					.map_err(|err| ReferenceSequenceTagError::InvalidAlternateLocus {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::AlternateLocus(alternate_locus))
			},
			"AN" => {
				let alternate_names: Vec<String> = value.value().split(",").map(|str| str.to_string()).collect();
				Ok(Self::AlternateNames(alternate_names))
			},
			"AS" => Ok(Self::AssemblyIdentifier(value.value().to_string())),
			"DS" => Ok(Self::Description(value.value().to_string())),
			"M5" => Ok(Self::Checksum(value.value().to_string())),
			"SP" => Ok(Self::Species(value.value().to_string())),
			"TP" => {
				let topology = MoleculeTopology::try_from(value.value())
					.map_err(|err| ReferenceSequenceTagError::InvalidTopology {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::MoleculeTopology(topology))
			},
			"UR" => Ok(Self::Uri(value.value().to_string())),
			_ => Err(ReferenceSequenceTagError::InvalidTagName {
				name
			})
		}
	}
}

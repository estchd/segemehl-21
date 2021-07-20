use crate::header::reference_sequence_line::alternate_locus::AlternateLocus;
use crate::header::reference_sequence_line::molecule_topology::MoleculeTopology;
use bam::header::Tag;
use std::convert::TryFrom;

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
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<ReferenceSequence, ()> {
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

		for tag in iter {
			let tag = ReferenceSequenceTag::try_from(tag)?;

			match tag {
				ReferenceSequenceTag::SequenceName(name) => {
					match sequence_name {
						None => {
							sequence_name = Some(name);
						},
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::SequenceLength(length) => {
					match sequence_length {
						None => {
							sequence_length = Some(length);
						},
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::AlternateLocus(locus) => {
					match alternate_locus {
						None => {
							alternate_locus = Some(locus);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::AlternateNames(names) => {
					match alternate_names {
						None => {
							alternate_names = Some(names);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::AssemblyIdentifier(identifier) => {
					match assembly_identifier {
						None => {
							assembly_identifier = Some(identifier);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::Description(description) => {
					match sequence_description {
						None => {
							sequence_description = Some(description);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::Checksum(checksum) => {
					match sequence_checksum {
						None => {
							sequence_checksum = Some(checksum);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::Species(species) => {
					match sequence_species {
						None => {
							sequence_species = Some(species);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				ReferenceSequenceTag::MoleculeTopology(topology) => {
					match molecule_topology {
						None => {
							molecule_topology = Some(topology);
						}
						Some(_) => {
							return Err(())
						}
					}
				}
				ReferenceSequenceTag::Uri(uri) => {
					match sequence_uri {
						None => {
							sequence_uri = Some(uri);
						}
						Some(_) => {
							return Err(())
						}
					}
				}
			}
		}

		let name = sequence_name.ok_or(())?;
		let length = sequence_length.ok_or(())?;
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
	type Error = ();

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec).map_err(|_| ())?;
		match name.as_str() {
			"SN" => Ok(Self::SequenceName(value.value().to_string())),
			"LN" => {
				let length = value.value().parse::<u32>().map_err(|_| ())?;
				Ok(Self::SequenceLength(length))
			},
			"AH" => {
				let alternate_locus = AlternateLocus::try_from(value.value())?;
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
				let topology = MoleculeTopology::try_from(value.value())?;
				Ok(Self::MoleculeTopology(topology))
			},
			"UR" => Ok(Self::Uri(value.value().to_string())),
			_ => Err(())
		}
	}
}
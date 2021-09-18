use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoleculeTopologyError {
	#[error("the molecule topology provided is not a valid molecule topology")]
	InvalidTopology
}

pub enum MoleculeTopology {
	Linear,
	Circular
}

impl TryFrom<&str> for MoleculeTopology {
	type Error = MoleculeTopologyError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"linear" => Ok(Self::Linear),
			"circular" => Ok(Self::Circular),
			_ => Err(MoleculeTopologyError::InvalidTopology)
		}
	}
}
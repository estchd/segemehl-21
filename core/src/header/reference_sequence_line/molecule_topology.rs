use std::convert::TryFrom;

pub enum MoleculeTopology {
	Linear,
	Circular
}

impl TryFrom<&str> for MoleculeTopology {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"linear" => Ok(Self::Linear),
			"circular" => Ok(Self::Circular),
			_ => Err(())
		}
	}
}
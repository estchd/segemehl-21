use std::convert::TryFrom;

pub enum SortingOrder {
	Unknown,
	Unsorted,
	QueryName,
	Coordinate
}

impl TryFrom<&str> for SortingOrder {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"unknown" => Ok(Self::Unknown),
			"unsorted" => Ok(Self::Unsorted),
			"queryname" => Ok(Self::QueryName),
			"coordinate" => Ok(Self::Coordinate),
			_ => Err(())
		}
	}
}

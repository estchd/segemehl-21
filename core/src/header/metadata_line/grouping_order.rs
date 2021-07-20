use std::convert::TryFrom;

pub enum GroupingOrder {
	None,
	Query,
	Reference
}

impl TryFrom<&str> for GroupingOrder {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"none" => Ok(Self::None),
			"query" => Ok(Self::Query),
			"reference" => Ok(Self::Reference),
			_ => Err(())
		}
	}
}
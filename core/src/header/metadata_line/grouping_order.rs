use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GroupingOrderError {
	#[error("the grouping order provided is not a valid grouping order")]
	UnrecognizedGroupingOrder
}

pub enum GroupingOrder {
	None,
	Query,
	Reference
}

impl TryFrom<&str> for GroupingOrder {
	type Error = GroupingOrderError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"none" => Ok(Self::None),
			"query" => Ok(Self::Query),
			"reference" => Ok(Self::Reference),
			_ => Err(GroupingOrderError::UnrecognizedGroupingOrder)
		}
	}
}
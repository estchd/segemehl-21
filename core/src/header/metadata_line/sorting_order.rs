use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SortingOrderError {
	#[error("the sorting order provided is not a valid grouping order")]
	UnrecognizedSortingOrder
}

pub enum SortingOrder {
	Unknown,
	Unsorted,
	QueryName,
	Coordinate
}

impl TryFrom<&str> for SortingOrder {
	type Error = SortingOrderError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"unknown" => Ok(Self::Unknown),
			"unsorted" => Ok(Self::Unsorted),
			"queryname" => Ok(Self::QueryName),
			"coordinate" => Ok(Self::Coordinate),
			_ => Err(SortingOrderError::UnrecognizedSortingOrder)
		}
	}
}

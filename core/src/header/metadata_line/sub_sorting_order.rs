use std::convert::{TryFrom};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubSortingOrderError {
	#[error("sub sorting order string contains no ':")]
	InvalidFormat,
	#[error("the sorting order provided is not a valid sorting order, value: {value}")]
	InvalidSortingOrder {
		value: String
	}
}

pub struct SubSortingOrder {
	pub sorting_order: SortingOrder,
	pub sub_sort: Vec<String>
}


impl TryFrom<&str> for SubSortingOrder {
	type Error = SubSortingOrderError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let split: Vec<&str> = value.split(":").collect();

		if split.len() < 2 {return Err(SubSortingOrderError::InvalidFormat)};

		let sorting_order = SortingOrder::try_from(split[0])
			.map_err(|_| SubSortingOrderError::InvalidSortingOrder {value: split[0].to_string()})?;

		let sub_sort: Vec<String> = split[1..split.len()].iter().map(|str| str.to_string()).collect();

		Ok(SubSortingOrder {
			sorting_order,
			sub_sort
		})
	}
}

pub enum SortingOrder {
	Unsorted,
	QueryName,
	Coordinate
}

impl TryFrom<&str> for SortingOrder {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"unsorted" => Ok(Self::Unsorted),
			"queryname" => Ok(Self::QueryName),
			"coordinate" => Ok(Self::Coordinate),
			_ => Err(())
		}
	}
}

use std::convert::{TryFrom};

pub struct SubSortingOrder {
	pub sorting_order: SortingOrder,
	pub sub_sort: Vec<String>
}


impl TryFrom<&str> for SubSortingOrder {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let split: Vec<&str> = value.split(":").collect();

		if split.len() < 2 {return Err(())};

		let sorting_order = SortingOrder::try_from(split[0])?;

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

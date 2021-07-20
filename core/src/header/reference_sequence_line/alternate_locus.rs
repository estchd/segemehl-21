use std::convert::TryFrom;

pub struct AlternateLocus {
	pub sequence: String,
	pub start: u32,
	pub end: u32
}

impl TryFrom<&str> for AlternateLocus {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let sequence_split: Vec<&str> = value.splitn(2, ":").collect();

		if sequence_split.len() != 2 {return Err(())};

		let sequence = sequence_split[0].to_string();

		let range_split: Vec<&str> = value.splitn(2, "-").collect();

		if range_split.len() != 2 {return Err(())};

		let start = range_split[0].parse::<u32>().map_err(|_| ())?;
		let end = range_split[1].parse::<u32>().map_err(|_| ())?;
		
		Ok(AlternateLocus {
			sequence,
			start,
			end
		})
	}
}

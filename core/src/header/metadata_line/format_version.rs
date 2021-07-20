use std::convert::TryFrom;

pub struct FormatVersion {
	pub major: u64,
	pub minor: u64,
}

impl TryFrom<&str> for FormatVersion {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let split_by_dot: Vec<&str> = value.split(".").collect();

		if split_by_dot.len() != 2 {return Err(())};

		let major = split_by_dot[0].parse::<u64>().map_err(|_| ())?;
		let minor = split_by_dot[1].parse::<u64>().map_err(|_| ())?;

		Ok(FormatVersion {
			major,
			minor
		})
	}
}

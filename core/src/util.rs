use std::num::NonZeroU32;
use bam::Record;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;

pub fn length(start: u32, end: u32) -> u32 {
	if end < start {
		return 0;
	}

	(end - start) + 1
}

pub fn get_quality_frequency_map(map: &PresentationFrequencyMap<u8>) -> Vec<(u8, u64)> {
	let range = 0..=255u8;

	let mut vec = Vec::<(u8, u64)>::with_capacity(256);

	for i in range {
		let value = map.get(&i).unwrap_or(0);

		vec.push((i, value));
	}

	vec
}

pub struct BinCoordinates {
	pub bin_index: u32,
	pub position_in_bin: u32
}

pub fn calculate_bin(map_start: u32, bin_size: NonZeroU32, entry_index: u32) -> Result<BinCoordinates,()> {
	if entry_index < map_start {return Err(());}

	let shifted_entry_index = entry_index - map_start;

	let bin_index = shifted_entry_index / bin_size.get();
	let position_in_bin = shifted_entry_index % bin_size.get();

	Ok(BinCoordinates {
		bin_index,
		position_in_bin
	})
}
pub fn get_record_name_as_string(record: &Record) -> Result<String, ()> {
	let name = record.name();
	let vec = Vec::from(name);
	String::from_utf8(vec).map_err(|_| ())
}

pub fn get_record_length(record: &Record) -> u32 {
	record.query_len()
}

pub fn get_record_mapping_quality(record: &Record) -> u8 {
	record.mapq()
}

pub fn get_record_start(record: &Record) -> u32 {
	record.start() as u32
}

pub fn get_record_end(record: &Record) -> u32 {
	record.start() as u32 + get_record_length(record).saturating_sub(1)
}

#[cfg(test)]
mod tests {
	use crate::util::calculate_bin;
	use std::num::NonZeroU32;
	use rstest::rstest;

	#[rstest]
	#[should_panic]
	#[case::panic(100,100,0,0,0)]
	#[case(0,500,0,0,0)]
	#[case(0,500,250,0,250)]
	#[case(0,500,499,0,499)]
	#[case(0,500,500,1,0)]
	fn calculate_bin_test(
		#[case] map_start: u32,
		#[case] bin_size: u32,
		#[case] entry_index: u32,
		#[case] expected_bin_index: u32,
		#[case] expected_position_in_bin: u32
	) {
		let bin_size = NonZeroU32::new(bin_size).unwrap();
		let bin_coordinates = calculate_bin(
			map_start,
			bin_size,
			entry_index
		).unwrap();

		assert_eq!(
			expected_bin_index,
			bin_coordinates.bin_index,
			"Got Wrong Bin Index, Expected: {}, Actual: {}",
			expected_bin_index,
			bin_coordinates.bin_index
		);

		assert_eq!(
			expected_position_in_bin,
			bin_coordinates.position_in_bin,
			"Got Wrong Position in Bin, Expected: {}, Actual: {}",
			expected_position_in_bin,
			bin_coordinates.position_in_bin
		)
	}
}
use std::cmp::{max, min};
use std::num::NonZeroU32;
use std::sync::atomic::Ordering;

use crate::statistics::calculation::binned::BinConfig;
use crate::statistics::calculation::binned::data::BinStatisticsCalculationData;
use crate::util::{calculate_bin, length, get_record_start, get_record_end, CigarMaxLengthIter};
use bam::Record;
use thiserror::Error;
use bam::record::cigar::Operation;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Error, Debug)]
pub enum BinnedStatisticsCalculationMapNewError {
	#[error("the range end was smaller than the range start")]
	InvalidRange
}

#[derive(Debug)]
pub struct BinnedStatisticsCalculationMap {
	pub(crate) bin_size: NonZeroU32,
	pub(crate) bins: Vec<BinStatisticsCalculationData>,
	pub(crate) start: u32,
	pub(crate) end: u32
}

impl BinnedStatisticsCalculationMap {
	pub fn new(start: u32, end: u32, config: BinConfig) -> Result<Self, BinnedStatisticsCalculationMapNewError> {
		if end < start {return Err(BinnedStatisticsCalculationMapNewError::InvalidRange);}

		let length = length(start, end);

		let (length_of_bins, number_of_bins) = match config {
			BinConfig::NumberOfBins(num) => {
				if length <= num.get() {
					(NonZeroU32::new(1).unwrap(), NonZeroU32::new(length).unwrap())
				}
				else {
					let length_of_bin = NonZeroU32::new((length as f64 / num.get() as f64).ceil() as u32).unwrap();

					(length_of_bin, num)
				}
			}
			BinConfig::LengthOfBins(len) => {
				let num_of_bins = NonZeroU32::new((length as f64 / len.get() as f64).ceil() as u32).unwrap();

				(len, num_of_bins)
			}
		};

		let mut bins = Vec::<BinStatisticsCalculationData>::with_capacity(number_of_bins.get() as usize);

		let mut current_start = start;

		let mut current_end = start + length_of_bins.get() - 1;

		loop {
			if current_end >= end {
				bins.push(BinStatisticsCalculationData::new(current_start, end));
				break;
			}

			bins.push(BinStatisticsCalculationData::new(current_start, current_end));

			current_start += length_of_bins.get();
			current_end += length_of_bins.get();
		}

		Ok(BinnedStatisticsCalculationMap {
			bin_size: length_of_bins,
			bins,
			start,
			end
		})
	}

	pub fn add_record(&self, record: &Record) {
		let start = get_record_start(record);
		let end = get_record_end(record);

		if end < start {return;}

		if start > self.end && end > self.end {return;}
		if start < self.start && end < self.start {return;}

		let start = max(start, self.start);
		let end = min(end, self.end);

		let start_bin = calculate_bin(self.start, self.bin_size, start).unwrap();
		let end_bin = calculate_bin(self.start, self.bin_size, end).unwrap();

		let mut front_iterator = CigarMaxLengthIter::new(record);
		let mut back_iterator = CigarMaxLengthIter::new(record);

		if start_bin.bin_index == end_bin.bin_index {
			let length_in_bin = length(start, end);

			let bin = &self.bins[start_bin.bin_index as usize];

			let cigars = front_iterator.collect();

			Self::add_cigars_to_bin(bin, cigars);

			bin.read_count.fetch_add(1, Ordering::Relaxed);
			bin.total_read_length.fetch_add(length_in_bin as usize, Ordering::Relaxed);

			return;
		}

		let length_in_start_bin = self.bin_size.get() - start_bin.position_in_bin;
		let length_in_end_bin = end_bin.position_in_bin + 1;

		let bin = &self.bins[start_bin.bin_index as usize];

		bin.read_count.fetch_add(1, Ordering::Relaxed);
		bin.total_read_length.fetch_add(length_in_start_bin as usize, Ordering::Relaxed);

		let start_bin_cigars = front_iterator.next_for_ref_length(length_in_start_bin);

		Self::add_cigars_to_bin(bin, start_bin_cigars);

		let bin = &self.bins[end_bin.bin_index as usize];

		bin.read_count.fetch_add(1, Ordering::Relaxed);
		bin.total_read_length.fetch_add(length_in_end_bin as usize, Ordering::Relaxed);

		let end_bin_cigars = back_iterator.next_for_ref_length(length_in_end_bin);

		Self::add_cigars_to_bin(bin, end_bin_cigars);

		let first_full_bin = start_bin.bin_index + 1;

		if end_bin.bin_index <= first_full_bin {return;}

		for bin in first_full_bin..end_bin.bin_index {
			let bin = &self.bins[bin as usize];

			bin.read_count.fetch_add(1, Ordering::Relaxed);
			bin.total_read_length.fetch_add(self.bin_size.get() as usize, Ordering::Relaxed);

			let bin_cigars = front_iterator.next_for_ref_length(bin.get_length());

			Self::add_cigars_to_bin(bin, bin_cigars);
		}
	}

	#[inline(always)]
	pub fn get_bin_size(&self) -> NonZeroU32 {
		self.bin_size
	}

	#[inline(always)]
	pub fn get_bin_count(&self) -> NonZeroU32 {
		NonZeroU32::new(self.bins.len() as u32).unwrap()
	}

	#[inline(always)]
	pub fn get_start(&self) -> u32 {
		self.start
	}

	#[inline(always)]
	pub fn get_end(&self) -> u32 {
		self.end
	}

	#[inline(always)]
	pub fn get_length(&self) -> u32 {
		(self.end - self.start) + 1
	}

	#[inline(always)]
	pub fn get_bins(&self) -> impl Iterator<Item = &BinStatisticsCalculationData> {
		self.bins.iter()
	}

	fn add_cigars_to_bin(bin: &BinStatisticsCalculationData, cigars: Vec<(u32, Operation)>) {
		for (len, op) in cigars {
			match op {
				Operation::SeqMatch |
				Operation::SeqMismatch |
				Operation::AlnMatch => {
					bin.alignment_matches.fetch_add(len as usize, Relaxed);
				}
				Operation::Insertion => {
					bin.insertions.fetch_add(len as usize, Relaxed);
				}
				Operation::Deletion => {
					bin.deletions.fetch_add(len as usize, Relaxed);
				}
				Operation::Skip => {
					bin.skips.fetch_add(len as usize, Relaxed);
				}
				Operation::Soft |
				Operation::Hard |
				Operation::Padding => {
				}
			}
		}
	}
}

#[cfg(test)]
mod calculation_map_creation {
	use std::num::NonZeroU32;

	use rstest::rstest;

	use crate::statistics::calculation::binned::BinConfig;
	use crate::statistics::calculation::binned::map::BinnedStatisticsCalculationMap;

	#[rstest]
	#[case(0, 999, 1, 1000, 1000)]
	#[case(0, 1000, 1, 1001, 1001)]
	#[case(0, 999, 2, 500, 500)]
	#[case(0, 1000, 2, 501, 500)]
	#[case(0, 999, 3, 334, 332)]
	#[case(0, 1000, 3, 334, 333)]
	fn bin_count_creation_test(
		#[case] map_start: u32,
		#[case] map_end: u32,
		#[case] number_of_bins: u32,
		#[case] expected_bin_size: u32,
		#[case] expected_length_of_last_bin: u32)
	{
		let bin_config = BinConfig::NumberOfBins(NonZeroU32::new(number_of_bins).unwrap());
		creation_test(map_start, map_end, bin_config, map_start, map_end, expected_bin_size, number_of_bins, expected_length_of_last_bin);
	}

	#[rstest]
	#[case(0, 999, 1000, 1, 1000)]
	#[case(0, 1000, 1001, 1, 1001)]
	#[case(0, 999, 500, 2, 500)]
	#[case(0, 1000, 501, 2, 500)]
	#[case(0, 1000, 500, 3, 1)]
	fn bin_size_creation_test(
		#[case] map_start: u32,
		#[case] map_end: u32,
		#[case] length_of_bins: u32,
		#[case] expected_number_of_bins: u32,
		#[case] expected_length_of_last_bin: u32)
	{
		let bin_config = BinConfig::LengthOfBins(NonZeroU32::new(length_of_bins).unwrap());
		creation_test(map_start, map_end, bin_config, map_start, map_end, length_of_bins, expected_number_of_bins, expected_length_of_last_bin);
	}

	fn creation_test(
		map_start: u32,
		map_end: u32,
		bin_config: BinConfig,
		expected_start: u32,
		expected_end: u32,
		expected_bin_size: u32,
		expected_number_of_bins: u32,
		expected_length_of_last_bin: u32)
	{
		let map = BinnedStatisticsCalculationMap::new(map_start, map_end, bin_config).unwrap();

		assert_eq!(expected_start, map.start, "Expected Start at {}, got {}", expected_start, map.start);
		assert_eq!(expected_end, map.end, "Expected End at {}, got {}", expected_end, map.end);

		assert_eq!(expected_number_of_bins, map.bins.len() as u32, "Expected {} Bins, got {}", expected_number_of_bins, map.bins.len());

		assert_eq!(expected_bin_size, map.bin_size.get(), "Expected Bins with length {}, got {}", expected_bin_size, map.bin_size);

		let mut expected_bin_start = map.start;

		for i in 0..(map.bins.len() - 1) {
			let bin = &map.bins[i];
			assert_eq!(
				bin.start,
				expected_bin_start,
				"Got Bin that's not adjacent to last Bin, Index: {}, End of last Bin: {}, Start of new Bin: {}",
				i,
				expected_bin_start - 1,
				bin.start
			);

			assert!(
				bin.start <= bin.end,
				"Got Bin with End before Start, Index: {}, Start: {}, End: {}",
				i,
				bin.start,
				bin.end
			);

			assert_eq!(
				expected_bin_size,
				bin.get_length(),
				"Got Bin with differing Size, Index: {}, Expected: {}, Got: {}",
				i,
				expected_bin_size,
				bin.get_length()
			);

			assert_eq!(
				0,
				bin.get_read_count(),
				"Got Fresh Bin with Coverage, Index: {}, Coverage: {}",
				i,
				bin.get_read_count()
			);

			assert_eq!(
				0.0,
				bin.get_coverage(),
				"Got Fresh Bin with Average Coverage, Index: {}, Average Coverage: {}",
				i,
				bin.get_coverage()
			);

			expected_bin_start = bin.end + 1;
		}
		let i = map.bins.len() - 1;
		let bin = &map.bins[i];

		assert_eq!(
			bin.start,
			expected_bin_start,
			"Got Bin that's not adjacent to last Bin, Index: {}, End of last Bin: {}, Start of new Bin: {}",
			i,
			expected_bin_start - 1,
			bin.start
		);

		assert!(
			bin.start <= bin.end,
			"Got Bin with End before Start, Index: {}, Start: {}, End: {}",
			i,
			bin.start,
			bin.end
		);

		assert_eq!(
			expected_length_of_last_bin,
			bin.get_length(),
			"Got Last Bin with differing Size, Index: {}, Expected: {}, Got: {}",
			i,
			expected_length_of_last_bin,
			bin.get_length()
		);

		assert_eq!(
			0,
			bin.get_read_count(),
			"Got Fresh Bin with Coverage, Index: {}, Coverage: {}",
			i,
			bin.get_read_count()
		);

		assert_eq!(
			0.0,
			bin.get_coverage(),
			"Got Fresh Bin with Average Coverage, Index: {}, Average Coverage: {}",
			i,
			bin.get_coverage()
		);
	}
}
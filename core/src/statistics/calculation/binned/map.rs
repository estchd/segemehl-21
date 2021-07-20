use std::cmp::{max, min};
use std::num::NonZeroU32;
use std::sync::atomic::Ordering;

use crate::statistics::calculation::binned::BinConfig;
use crate::statistics::calculation::binned::data::BinStatisticsCalculationData;
use crate::util::{calculate_bin, length};

#[derive(Debug)]
pub struct BinnedStatisticsCalculationMap {
	pub(crate) bin_size: NonZeroU32,
	pub(crate) bins: Vec<BinStatisticsCalculationData>,
	pub(crate) start: u32,
	pub(crate) end: u32
}

impl BinnedStatisticsCalculationMap {
	pub fn new(start: u32, end: u32, config: BinConfig) -> Result<Self,()> {
		if end < start {return Err(());}

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

	pub fn add_coverage(&self, start: u32, end: u32) {
		if end < start {return;}

		if start > self.end && end > self.end {return;}
		if start < self.start && end < self.start {return;}

		let start = max(start, self.start);
		let end = min(end, self.end);

		let start_bin = calculate_bin(self.start, self.bin_size, start).unwrap();
		let end_bin = calculate_bin(self.start, self.bin_size, end).unwrap();

		if start_bin.bin_index == end_bin.bin_index {
			let length_in_bin = length(start, end);

			let bin = &self.bins[start_bin.bin_index as usize];

			bin.coverage.fetch_add(1, Ordering::Relaxed);
			bin.coverage_times_area.fetch_add(length_in_bin as usize, Ordering::Relaxed);

			return;
		}

		let length_in_start_bin = self.bin_size.get() - start_bin.position_in_bin;
		let length_in_end_bin = end_bin.position_in_bin + 1;

		let bin = &self.bins[start_bin.bin_index as usize];

		bin.coverage.fetch_add(1, Ordering::Relaxed);
		bin.coverage_times_area.fetch_add(length_in_start_bin as usize, Ordering::Relaxed);

		let bin = &self.bins[end_bin.bin_index as usize];

		bin.coverage.fetch_add(1, Ordering::Relaxed);
		bin.coverage_times_area.fetch_add(length_in_end_bin as usize, Ordering::Relaxed);

		let first_full_bin = start_bin.bin_index + 1;

		if end_bin.bin_index <= first_full_bin {return;}

		for bin in first_full_bin..end_bin.bin_index {
			let bin = &self.bins[bin as usize];

			bin.coverage.fetch_add(1, Ordering::Relaxed);
			bin.coverage_times_area.fetch_add(self.bin_size.get() as usize, Ordering::Relaxed);
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
				bin.get_coverage(),
				"Got Fresh Bin with Coverage, Index: {}, Coverage: {}",
				i,
				bin.get_coverage()
			);

			assert_eq!(
				0.0,
				bin.get_average_coverage(),
				"Got Fresh Bin with Average Coverage, Index: {}, Average Coverage: {}",
				i,
				bin.get_average_coverage()
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
			bin.get_coverage(),
			"Got Fresh Bin with Coverage, Index: {}, Coverage: {}",
			i,
			bin.get_coverage()
		);

		assert_eq!(
			0.0,
			bin.get_average_coverage(),
			"Got Fresh Bin with Average Coverage, Index: {}, Average Coverage: {}",
			i,
			bin.get_average_coverage()
		);
	}


}

#[cfg(test)]
mod calculation_map_insertion {
	use std::num::NonZeroU32;

	use rstest::{fixture, rstest};

	use crate::statistics::calculation::binned::BinConfig;
	use crate::statistics::calculation::binned::map::BinnedStatisticsCalculationMap;

	#[fixture]
	fn map() -> BinnedStatisticsCalculationMap {
		let config = BinConfig::NumberOfBins(NonZeroU32::new(3).unwrap());
		BinnedStatisticsCalculationMap::new(0,1499, config).unwrap()
	}

	#[rstest]
	#[case(1, vec![(0,499)], vec![(1,500),(0,0),(0,0)])]
	#[case(2, vec![(500,999)], vec![(0,0),(1,500),(0,0)])]
	#[case(3, vec![(1000,1499)], vec![(0,0),(0,0),(1,500)])]
	#[case(4, vec![(1000,1999)], vec![(0,0),(0,0),(1,500)])]
	#[case(5, vec![(497,502)], vec![(1,3),(1,3),(0,0)])]
	#[case(6, vec![(997,1002)], vec![(0,0),(1,3),(1,3)])]
	#[case(7, vec![(249,250)], vec![(1,2),(0,0),(0,0)])]
	#[case(8, vec![(749,750)], vec![(0,0),(1,2),(0,0)])]
	#[case(9, vec![(1249,1250)], vec![(0,0),(0,0),(1,2)])]
	#[case(10, vec![(250,1249)], vec![(1,250),(1,500),(1,250)])]
	#[case(11, vec![(250,1249), (0, 499)], vec![(2,750),(1,500),(1,250)])]
	#[case(12, vec![(250,1249), (500, 999)], vec![(1,250),(2,1000),(1,250)])]
	#[case(13, vec![(250,1249), (1000, 1499)], vec![(1,250),(1,500),(2,750)])]
	#[case(14, vec![(0, 499), (250,1249), (1000, 1499)], vec![(2,750),(1,500),(2,750)])]
	#[case(15, vec![(250,1249), (1000, 1499), (0, 499)], vec![(2,750),(1,500),(2,750)])]
	#[trace]
	fn insertion_test(
		map: BinnedStatisticsCalculationMap,
		#[allow(unused_variables)]
		#[case] test_index: u32,
		#[case] entries: Vec<(u32, u32)>,
		#[case] expected_bins: Vec<(u32, usize)>
	) {
		for (start, end) in entries {
			map.add_coverage(start, end);
		}

		assert_eq!(expected_bins.len(), map.bins.len(), "Expected {} Bins, got {}", expected_bins.len(), map.bins.len());

		let map_bin_iter = map.bins
			.into_iter()
			.map(|item| (item.coverage.into_inner(), item.coverage_times_area.into_inner()));
		let expected_bin_iter = expected_bins.into_iter();

		let comparison_iter = map_bin_iter
			.zip(expected_bin_iter)
			.enumerate();

		for (i, ((coverage, coverage_times_area), (expected_coverage,  expected_coverage_times_area))) in comparison_iter {
			assert_eq!(
				expected_coverage,
				coverage,
				"Got Bin with differing Coverage, Index: {}, Expected: {}, Actual: {}",
				i,
				expected_coverage,
				coverage
			);

			assert_eq!(
				expected_coverage_times_area,
				coverage_times_area,
				"Got Bin with differing Coverage times Area, Index: {}, Expected: {}, Actual: {}",
				i,
				expected_coverage_times_area,
				coverage_times_area
			)
		}
	}
}

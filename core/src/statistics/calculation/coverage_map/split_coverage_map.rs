use crate::statistics::calculation::coverage_map::coverage_map::CoverageMap;
use std::num::{NonZeroUsize, NonZeroU32};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
pub struct SplitCoverageMap {
	pub max_map_size: NonZeroUsize,
	pub start: u32,
	pub end: NonZeroU32,
	pub maps: Vec<CoverageMap>
}

impl SplitCoverageMap {
	pub fn new(start: u32, end: NonZeroU32, max_map_size: NonZeroUsize) -> Self {
		let first_map = CoverageMap::new(start,end);
		let mut maps = Vec::<CoverageMap>::new();
		maps.push(first_map);

		SplitCoverageMap {
			max_map_size,
			start,
			end,
			maps
		}
	}

	pub fn add_coverage(&mut self, start: u32, end: u32) {
		if self.maps.last().unwrap().entries.len() >= usize::from(self.max_map_size) {
			let new_map = CoverageMap::new(self.start,self.end);
			self.maps.push(new_map);
		}

		let map = self.maps.last_mut().unwrap();

		map.add_coverage(start,end);
	}

	pub fn combine(self) -> CoverageMap {
		let SplitCoverageMap {
			start,
			end,
			maps,
			..
		} = self;

		let iter = maps.into_par_iter();
		iter.reduce(
			|| CoverageMap::new(start, end),
			|lhs,rhs| CoverageMap::merge(lhs,rhs)
		)
	}
}

#[cfg(test)]
mod merge_tests {
	use std::num::{NonZeroU32, NonZeroUsize};
	use crate::statistics::calculation::coverage_map::split_coverage_map::SplitCoverageMap;

	#[test]
	fn disjointed_entries_smaller_first() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn disjointed_entries_smaller_first_non_total_coverage() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 50u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 150u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 101);
	}

	#[test]
	fn disjointed_entries_bigger_first() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn overlapping_entries_smaller_first() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn overlapping_entries_bigger_first() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_inclusion() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_coverage() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn coverage_for_separate_entries() {
		let mut coverage_map = SplitCoverageMap::new(0, NonZeroU32::new(200).unwrap(), NonZeroUsize::new(1).unwrap());

		let entry_a_start = 25u32;
		let entry_a_end = 75u32;
		let entry_b_start = 125u32;
		let entry_b_end = 175u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.maps.len(), 2);

		let coverage_map = coverage_map.combine();

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 102);
	}
}
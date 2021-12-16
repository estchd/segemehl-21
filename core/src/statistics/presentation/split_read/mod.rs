use std::cmp::{max, min, Ordering};
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::record::PresentationRecord;
use serde_derive::{Serialize,Deserialize};

pub mod collection;
pub mod statistics;
pub mod partial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRead {
	forward_strand_records: Vec<PresentationRecord>,
	reverse_strand_records: Vec<PresentationRecord>,
	unmapped_records: Vec<PresentationRecord>,
	min_unmapped_records: usize
}

impl SplitRead {
	pub fn get_gap_length_map(&self) -> PresentationFrequencyMap<i64> {
		let calculation_map = CalculationFrequencyMap::new();

		self.calculate_gap_lengths_into_map(&calculation_map);

		calculation_map.into()
	}

	pub fn calculate_gap_lengths_into_map(&self, calculation_map: &CalculationFrequencyMap<i64>) {
		let mut forward_index = 0usize;
		let mut reverse_index = 0usize;

		let mut last_end = None;

		while forward_index < self.forward_strand_records.len() && reverse_index < self.reverse_strand_records.len() {
			let forward = &self.forward_strand_records[forward_index];
			let reverse = &self.reverse_strand_records[reverse_index];

			let current: &PresentationRecord;

			if forward.get_start() <= reverse.get_start() {
				current = forward;
				forward_index += 1;
			}
			else {
				current = reverse;
				reverse_index += 1;
			}

			if let Some(end) = last_end {
				let gap_length = current.get_start() as i64 - end as i64;
				calculation_map.add_entry(gap_length);
			}

			last_end = Some(current.get_end());
		}

		while forward_index < self.forward_strand_records.len() {
			let current = &self.forward_strand_records[forward_index];

			if let Some(end) = last_end {
				let gap_length = current.get_start() as i64 - end as i64;
				calculation_map.add_entry(gap_length);
			}

			last_end = Some(current.get_end());
			forward_index += 1;
		}

		while reverse_index < self.reverse_strand_records.len() {
			let current = &self.reverse_strand_records[reverse_index];

			if let Some(end) = last_end {
				let gap_length = current.get_start() as i64 - end as i64;
				calculation_map.add_entry(gap_length);
			}

			last_end = Some(current.get_end());
			reverse_index += 1;
		}
	}

	pub fn get_split_count(&self, include_unmapped: bool) -> usize {
		let mapped_splits =
			self.forward_strand_records.len() +
				self.reverse_strand_records.len();

		return if include_unmapped {
			mapped_splits + self.unmapped_records.len() + self.min_unmapped_records
		}
		else {
			mapped_splits
		}
	}

	pub fn get_total_length(&self) -> u32 {
		let forward_min = self.forward_strand_records
		                      .first()
		                      .map(|record| record.get_start())
		                      .unwrap_or(0);

		let mut forward_max = 0u32;
		for record in &self.forward_strand_records {
			forward_max = max(forward_max, record.get_end());
		}

		let reverse_min = self.reverse_strand_records
		                      .first()
		                      .map(|record| record.get_start())
		                      .unwrap_or(0);

		let mut reverse_max = 0u32;
		for record in &self.reverse_strand_records {
			reverse_max = max(reverse_max, record.get_end());
		}

		let min = min(forward_min, reverse_min);
		let max = max(forward_max, reverse_max);

		max - min
	}

	pub fn get_statistics(&self) -> (PresentationFrequencyMap<i64>, u32, usize, usize) {
		let gap_length_map = self.get_gap_length_map();
		let split_count = self.get_split_count(false);
		let split_count_unmapped = self.get_split_count(true);
		let total_length = self.get_total_length();

		(gap_length_map, total_length, split_count, split_count_unmapped)
	}

	pub fn calculate_statistics_into(&self, gap_length_map: &CalculationFrequencyMap<i64>, total_length_map: &CalculationFrequencyMap<u32>, split_count_map: &CalculationFrequencyMap<usize>, split_count_unmapped_map: &CalculationFrequencyMap<usize>, unmapped_count_map: &CalculationFrequencyMap<usize>) {
		self.calculate_gap_lengths_into_map(gap_length_map);
		let total_length = self.get_total_length();
		let split_count = self.get_split_count(false);

		if split_count > 10 {
			println!("{}", split_count);
		}

		let split_count_unmapped = self.get_split_count(true);
		let unmapped_count = split_count_unmapped - split_count;

		total_length_map.add_entry(total_length);
		split_count_map.add_entry(split_count);
		split_count_unmapped_map.add_entry(split_count_unmapped);
		unmapped_count_map.add_entry(unmapped_count);
	}
}

impl From<(Vec<PresentationRecord>, usize)> for SplitRead {
	fn from(records: (Vec<PresentationRecord>, usize)) -> Self {
		let (associated_records, min_unmapped_records) = records;
		let mut forward_strand_records: Vec<PresentationRecord> = Vec::new();
		let mut reverse_strand_records: Vec<PresentationRecord> = Vec::new();
		let mut unmapped_records: Vec<PresentationRecord> = Vec::new();

		for record in associated_records {
			let mapped = record.get_flags().get_is_mapped();
			let reverse_strand = record.get_flags().get_is_reverse_strand();

			if !mapped {
				unmapped_records.push(record);
				continue;
			}

			if reverse_strand {
				reverse_strand_records.push(record)
			}
			else {
				forward_strand_records.push(record)
			}
		}

		let ordering_func: fn(&PresentationRecord, &PresentationRecord) -> Ordering =
			|a,b| {
				return if a.get_start() < b.get_start() {
					Ordering::Less
				} else if a.get_start() > b.get_start() {
					Ordering::Greater
				} else {
					if a.get_end() < b.get_end() {
						Ordering::Less
					} else if a.get_end() > b.get_end() {
						Ordering::Greater
					} else {
						Ordering::Equal
					}
				}
			};

		forward_strand_records.sort_by(ordering_func);
		reverse_strand_records.sort_by(ordering_func);

		Self {
			forward_strand_records,
			reverse_strand_records,
			unmapped_records,
			min_unmapped_records
		}
	}
}
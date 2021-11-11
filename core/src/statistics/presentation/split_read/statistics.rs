use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_derive::{Serialize, Deserialize};
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::split_read::collection::SplitReadCollection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitReadStatistics {
	gap_length_map: PresentationFrequencyMap<i64>,
	total_length_map: PresentationFrequencyMap<u32>,
	split_count_map: PresentationFrequencyMap<usize>,
	split_count_unmapped_map: PresentationFrequencyMap<usize>,
	unmapped_count_map: PresentationFrequencyMap<usize>
}

impl SplitReadStatistics {
	pub fn new() -> Self {
		Self {
			gap_length_map: PresentationFrequencyMap::new(),
			total_length_map: PresentationFrequencyMap::new(),
			split_count_map: PresentationFrequencyMap::new(),
			split_count_unmapped_map: PresentationFrequencyMap::new(),
			unmapped_count_map: PresentationFrequencyMap::new()
		}
	}

	pub fn get_gap_length_map(&self) -> &PresentationFrequencyMap<i64> {
		&self.gap_length_map
	}

	pub fn get_total_length_map(&self) -> &PresentationFrequencyMap<u32> {
		&self.total_length_map
	}

	pub fn get_split_count_map(&self) -> &PresentationFrequencyMap<usize> {
		&self.split_count_map
	}

	pub fn get_split_count_unmapped_map(&self) -> &PresentationFrequencyMap<usize> {
		&self.split_count_unmapped_map
	}

	pub fn get_unmapped_count_map(&self) -> &PresentationFrequencyMap<usize> {
		&self.unmapped_count_map
	}
}

impl From<SplitReadCollection> for SplitReadStatistics {
	fn from(value: SplitReadCollection) -> Self {
		println!("Calculating Statistics");
		let gap_length_map = CalculationFrequencyMap::<i64>::new();
		let total_length_map = CalculationFrequencyMap::<u32>::new();
		let split_count_map = CalculationFrequencyMap::<usize>::new();
		let split_count_unmapped_map = CalculationFrequencyMap::<usize>::new();
		let unmapped_count_map = CalculationFrequencyMap::<usize>::new();

		let vec = value.into_inner();

		vec.into_par_iter().for_each(|item| {
			item.calculate_statistics_into(&gap_length_map, &total_length_map, &split_count_map, &split_count_unmapped_map, &unmapped_count_map)
		});

		Self {
			gap_length_map: gap_length_map.into(),
			total_length_map: total_length_map.into(),
			split_count_map: split_count_map.into(),
			split_count_unmapped_map: split_count_unmapped_map.into(),
			unmapped_count_map: unmapped_count_map.into()
		}
	}
}
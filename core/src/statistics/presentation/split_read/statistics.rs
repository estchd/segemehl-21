use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
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
		let statistics: Vec<(PresentationFrequencyMap<i64>, u32, usize, usize)> = value.into_inner().par_iter().map(|item| {
			item.get_statistics()
		}).collect();

		let total_length_map = CalculationFrequencyMap::<u32>::new();
		let split_count_map = CalculationFrequencyMap::<usize>::new();
		let split_count_unmapped_map = CalculationFrequencyMap::<usize>::new();
		let unmapped_count_map = CalculationFrequencyMap::<usize>::new();

		let gap_length_maps: Vec<PresentationFrequencyMap<i64>> = statistics.into_iter().par_bridge().fold(
			|| {
				PresentationFrequencyMap::<i64>::new()
			},
			|a,b| {
				total_length_map.add_entry(b.1);
				split_count_map.add_entry(b.2);
				split_count_unmapped_map.add_entry(b.3);
				unmapped_count_map.add_entry(b.3 - b.2);
				PresentationFrequencyMap::merge(&a,&b.0)
			}
		).collect();

		let gap_length_map = gap_length_maps.into_iter().fold(
			PresentationFrequencyMap::<i64>::new(),
			|a,b| {
				PresentationFrequencyMap::merge(&a,&b)
			}
		);

		Self {
			gap_length_map,
			total_length_map: total_length_map.into(),
			split_count_map: split_count_map.into(),
			split_count_unmapped_map: split_count_unmapped_map.into(),
			unmapped_count_map: unmapped_count_map.into()
		}
	}
}
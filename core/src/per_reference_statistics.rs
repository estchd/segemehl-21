use serde_derive::{Deserialize, Serialize};

use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::calculation::coverage_map::split_coverage_map::SplitCoverageMap;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;

#[derive(Serialize, Deserialize)]
pub struct PerReferenceStatistics {
	#[serde(rename = "rn")]
	pub reference_name: String,
	#[serde(rename = "rl")]
	pub reference_length: u32,
	#[serde(rename = "mean")]
	pub mean_read_length: f64,
	#[serde(rename = "mode")]
	pub mode_read_length: u32,
	#[serde(rename = "medi")]
	pub median_read_length: f64,
	#[serde(rename = "srl")]
	pub shortest_read_length: u32,
	#[serde(rename = "lrl")]
	pub longest_read_length: u32,
	#[serde(rename = "nr")]
	pub number_of_reads: u64,
	#[serde(rename = "trl")]
	pub total_read_length: u32,
	#[serde(rename = "cm")]
	pub covered_length: u32,
	#[serde(rename = "rlm")]
	pub read_length_map: PresentationFrequencyMap<u32>
}

impl From<PerReferenceStatisticsCalculationData> for PerReferenceStatistics {
	fn from(data: PerReferenceStatisticsCalculationData) -> Self {
		let PerReferenceStatisticsCalculationData {
			reference_name,
			reference_length,
			number_of_reads,
			total_read_length,
			shortest_read_length,
			longest_read_length,
			coverage_map,
			read_length_map
		} = data;

		let shortest_read_length = shortest_read_length.unwrap_or(0);
		let longest_read_length = longest_read_length.unwrap_or(0);

		let mean_read_length = total_read_length as f64 / number_of_reads as f64;
		let median_read_length = (shortest_read_length + longest_read_length) as f64 / 2.0;
		let mode_read_length = read_length_map.get_max_frequency().map(|(i,_)| i).unwrap_or(0);

		println!("Combining {} split reference maps", coverage_map.maps.len());
		let coverage_map = coverage_map.combine();

		PerReferenceStatistics {
			reference_name,
			reference_length,
			mean_read_length,
			mode_read_length,
			median_read_length,
			shortest_read_length,
			longest_read_length,
			number_of_reads: number_of_reads as u64,
			total_read_length,
			covered_length: coverage_map.get_total_covered_length(),
			read_length_map: read_length_map.into()
		}
	}
}

pub struct PerReferenceStatisticsCalculationData {
	pub reference_name: String,
	pub reference_length: u32,
	pub number_of_reads: usize,
	pub total_read_length: u32,
	pub shortest_read_length: Option<u32>,
	pub longest_read_length: Option<u32>,
	pub coverage_map: SplitCoverageMap,
	pub read_length_map: CalculationFrequencyMap<u32>
}
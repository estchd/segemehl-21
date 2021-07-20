use crate::per_reference_statistics::PerReferenceStatistics;
use serde_derive::{Serialize, Deserialize};
use crate::statistics::presentation::binned::map::BinnedStatisticsPresentationMap;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;

#[derive(Serialize, Deserialize)]
pub struct BamStatistics {
	#[serde(rename = "rc")]
	pub record_count: u64,
	#[serde(rename = "trl")]
	pub total_record_length: u64,
	#[serde(rename = "medrl")]
	pub median_read_length: f64,
	#[serde(rename = "modrl")]
	pub mode_read_length: u32,
	#[serde(rename = "mearl")]
	pub mean_read_length: f64,
	#[serde(rename = "medrc")]
	pub median_read_count: f64,
	#[serde(rename = "modrc")]
	pub mode_read_count: u64,
	#[serde(rename = "mearc")]
	pub mean_read_count: f64,
	#[serde(rename = "srl")]
	pub smallest_read_length: u32,
	#[serde(rename = "brl")]
	pub biggest_read_length: u32,
	#[serde(rename = "lrc")]
	pub least_read_count: u64,
	#[serde(rename = "mrc")]
	pub most_read_count: u64,
	#[serde(rename = "prs")]
	pub per_reference_statistics: Vec<PerReferenceStatistics>,
	#[serde(rename = "prb")]
	pub per_reference_binned_statistics: Vec<BinnedStatisticsPresentationMap>,
	#[serde(rename = "prq")]
	pub per_reference_quality_frequency: Vec<PresentationFrequencyMap<usize>>,
	#[serde(rename = "q")]
	pub complete_quality_frequency: PresentationFrequencyMap<usize>
}
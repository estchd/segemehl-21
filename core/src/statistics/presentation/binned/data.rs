use crate::statistics::calculation::binned::data::BinStatisticsCalculationData;
use crate::util::length;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinStatisticsPresentationData {
	#[serde(rename = "st")]
	start: u32,
	#[serde(rename = "e")]
	end: u32,
	#[serde(rename = "c")]
	coverage: u32,
	#[serde(rename = "cta")]
	coverage_times_area: u64,
	#[serde(rename = "m")]
	alignment_matches: u64,
	#[serde(rename = "i")]
	insertions: u64,
	#[serde(rename = "d")]
	deletions: u64,
	#[serde(rename = "sk")]
	skips: u64
}

impl BinStatisticsPresentationData {
	pub fn merge(lhs: &BinStatisticsPresentationData, rhs: &BinStatisticsPresentationData) -> Result<BinStatisticsPresentationData,()> {
		if lhs.start != rhs.start { return Err(()); }
		if lhs.end != rhs.end { return Err(()); }

		Ok(BinStatisticsPresentationData {
			start: lhs.start,
			end: lhs.end,
			coverage: lhs.coverage + rhs.coverage,
			coverage_times_area: lhs.coverage_times_area + rhs.coverage_times_area,
			alignment_matches: lhs.alignment_matches + rhs.alignment_matches,
			insertions: lhs.insertions + rhs.insertions,
			deletions: lhs.deletions + rhs.deletions,
			skips: lhs.skips + rhs.skips
		})
	}

	#[inline(always)]
	pub fn get_length(&self) -> u32 {
		length(self.start, self.end)
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
	pub fn get_coverage(&self) -> u32 {
		self.coverage
	}

	#[inline(always)]
	pub fn get_average_coverage(&self) -> f64 {
		self.coverage_times_area as f64 / self.get_length() as f64
	}
}

impl Default for BinStatisticsPresentationData {
	fn default() -> Self {
		BinStatisticsPresentationData {
			start: 0,
			end: 0,
			coverage: Default::default(),
			coverage_times_area: Default::default(),
			alignment_matches: 0,
			insertions: 0,
			deletions: 0,
			skips: 0
		}
	}
}

impl From<BinStatisticsCalculationData> for BinStatisticsPresentationData {
	fn from(data: BinStatisticsCalculationData) -> Self {
		BinStatisticsPresentationData {
			start: data.start,
			end: data.end,
			coverage: data.coverage.into_inner(),
			coverage_times_area: data.coverage_times_area.into_inner() as u64,
			alignment_matches: data.alignment_matches.into_inner() as u64,
			insertions: data.insertions.into_inner() as u64,
			deletions: data.deletions.into_inner() as u64,
			skips: data.skips.into_inner() as u64
		}
	}
}

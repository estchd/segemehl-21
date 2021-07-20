use crate::statistics::calculation::binned::data::BinStatisticsCalculationData;
use crate::util::length;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinStatisticsPresentationData {
	#[serde(rename = "s")]
	start: u32,
	#[serde(rename = "e")]
	end: u32,
	#[serde(rename = "c")]
	coverage: u32,
	#[serde(rename = "cta")]
	coverage_times_area: u64
}

impl BinStatisticsPresentationData {
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
			coverage_times_area: Default::default()
		}
	}
}

impl From<BinStatisticsCalculationData> for BinStatisticsPresentationData {
	fn from(data: BinStatisticsCalculationData) -> Self {
		BinStatisticsPresentationData {
			start: data.start,
			end: data.end,
			coverage: data.coverage.into_inner(),
			coverage_times_area: data.coverage_times_area.into_inner() as u64
		}
	}
}

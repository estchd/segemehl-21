use std::num::NonZeroU32;

use crate::statistics::presentation::binned::data::BinStatisticsPresentationData;
use crate::statistics::calculation::binned::map::BinnedStatisticsCalculationMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinnedStatisticsPresentationMap {
	#[serde(rename = "bs")]
	bin_size: NonZeroU32,
	#[serde(rename = "b")]
	bins: Vec<BinStatisticsPresentationData>,
	#[serde(rename = "s")]
	start: u32,
	#[serde(rename = "e")]
	end: u32
}

impl BinnedStatisticsPresentationMap {
	pub fn merge(lhs: &BinnedStatisticsPresentationMap, rhs: &BinnedStatisticsPresentationMap) -> Result<BinnedStatisticsPresentationMap,()> {
		if lhs.start != rhs.start { return Err(()); }
		if lhs.end != rhs.end { return Err(()); }
		if lhs.bin_size != rhs.bin_size { return Err(()); }

		let mut new_bins = Vec::new();

		for i in 0..lhs.bins.len() {
			new_bins.push(BinStatisticsPresentationData::merge(&lhs.bins[i], &rhs.bins[i])?);
		}

		Ok(BinnedStatisticsPresentationMap {
			bin_size: lhs.bin_size,
			bins: new_bins,
			start: lhs.start,
			end: lhs.end
		})
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
	pub fn get_bins(&self) -> impl Iterator<Item = &BinStatisticsPresentationData> {
		self.bins.iter()
	}
}

impl From<BinnedStatisticsCalculationMap> for BinnedStatisticsPresentationMap {
	fn from(map: BinnedStatisticsCalculationMap) -> Self {
		BinnedStatisticsPresentationMap {
			bin_size: map.bin_size,
			bins: map.bins.into_iter().map(|data| data.into()).collect(),
			start: map.start,
			end: map.end
		}
	}
}

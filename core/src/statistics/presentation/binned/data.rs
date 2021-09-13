use crate::statistics::calculation::binned::data::BinStatisticsCalculationData;
use crate::util::length;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::cigar_operations::CigarOperations;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinStatisticsPresentationData {
	#[serde(rename = "st")]
	start: u32,
	#[serde(rename = "e")]
	end: u32,
	#[serde(rename = "c")]
	read_count: u64,
	#[serde(rename = "cta")]
	total_read_length: u64,
	#[serde(rename = "co")]
	cigar_operations: CigarOperations
}

impl BinStatisticsPresentationData {
	pub fn merge(lhs: &BinStatisticsPresentationData, rhs: &BinStatisticsPresentationData) -> Result<BinStatisticsPresentationData,()> {
		if lhs.start != rhs.start { return Err(()); }
		if lhs.end != rhs.end { return Err(()); }

		Ok(BinStatisticsPresentationData {
			start: lhs.start,
			end: lhs.end,
			read_count: lhs.read_count + rhs.read_count,
			total_read_length: lhs.total_read_length + rhs.total_read_length,
			cigar_operations: CigarOperations::merge(&lhs.cigar_operations, &rhs.cigar_operations)
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
	pub fn get_read_count(&self) -> u64 {
		self.read_count
	}

	#[inline(always)]
	pub fn get_total_read_length(&self) -> u64 {
		self.total_read_length
	}

	#[inline(always)]
	pub fn get_cigar_operations(&self) -> CigarOperations {
		self.cigar_operations
	}

	#[inline(always)]
	pub fn get_coverage(&self) -> f64 {
		self.total_read_length as f64 / self.get_length() as f64
	}
}

impl Default for BinStatisticsPresentationData {
	fn default() -> Self {
		BinStatisticsPresentationData {
			start: 0,
			end: 0,
			read_count: Default::default(),
			total_read_length: Default::default(),
			cigar_operations: Default::default(),
		}
	}
}

impl From<BinStatisticsCalculationData> for BinStatisticsPresentationData {
	fn from(data: BinStatisticsCalculationData) -> Self {
		BinStatisticsPresentationData {
			start: data.start,
			end: data.end,
			read_count: data.read_count.into_inner() as u64,
			total_read_length: data.total_read_length.into_inner() as u64,
			cigar_operations: CigarOperations {
				alignment_matches: data.alignment_matches.into_inner() as u64,
				insertions: data.insertions.into_inner() as u64,
				deletions: data.deletions.into_inner() as u64,
				skips: data.skips.into_inner() as u64,
			}
		}
	}
}

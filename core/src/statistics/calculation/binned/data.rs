use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use crate::util::length;

#[derive(Debug)]
pub struct BinStatisticsCalculationData {
	pub(crate) start: u32,
	pub(crate) end: u32,
	pub(crate) coverage: AtomicU32,
	pub(crate) coverage_times_area: AtomicUsize,
	pub(crate) alignment_matches: AtomicUsize,
	pub(crate) insertions: AtomicUsize,
	pub(crate) deletions: AtomicUsize,
	pub(crate) skips: AtomicUsize,
}

impl BinStatisticsCalculationData {
	pub fn new(start: u32, end: u32) -> Self {
		BinStatisticsCalculationData {
			start,
			end,
			coverage: Default::default(),
			coverage_times_area: Default::default(),
			alignment_matches: Default::default(),
			insertions: Default::default(),
			deletions: Default::default(),
			skips: Default::default()
		}
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
		self.coverage.load(Ordering::Relaxed)
	}

	#[inline(always)]
	pub fn get_average_coverage(&self) -> f64 {
		self.coverage_times_area.load(Ordering::Relaxed) as f64 / self.get_length() as f64
	}
}

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::util::length;

#[derive(Debug)]
pub struct BinStatisticsCalculationData {
	pub(crate) start: u32,
	pub(crate) end: u32,
	pub(crate) read_count: AtomicUsize,
	pub(crate) total_read_length: AtomicUsize,
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
			read_count: Default::default(),
			total_read_length: Default::default(),
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
	pub fn get_read_count(&self) -> usize {
		self.read_count.load(Ordering::Relaxed)
	}

	#[inline(always)]
	pub fn get_total_read_length(&self) -> usize {
		self.total_read_length.load(Ordering::Relaxed)
	}

	#[inline(always)]
	pub fn get_coverage(&self) -> f64 {
		self.total_read_length.load(Ordering::Relaxed) as f64 / self.get_length() as f64
	}
}

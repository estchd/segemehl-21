use std::num::NonZeroU32;

pub mod data;
pub mod map;

#[derive(Debug, Copy, Clone)]
pub enum BinConfig {
	NumberOfBins(NonZeroU32),
	LengthOfBins(NonZeroU32),
}

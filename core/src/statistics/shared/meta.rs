use std::num::{NonZeroU32};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Meta {
	pub bin_size: NonZeroU32
}
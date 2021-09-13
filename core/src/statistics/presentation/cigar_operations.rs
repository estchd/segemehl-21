use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct CigarOperations {
	#[serde(rename = "m")]
	pub alignment_matches: u64,
	#[serde(rename = "i")]
	pub insertions: u64,
	#[serde(rename = "d")]
	pub deletions: u64,
	#[serde(rename = "sk")]
	pub skips: u64
}

impl CigarOperations {
	pub fn merge(lhs: &Self, rhs: &Self) -> Self {
		Self {
			alignment_matches: lhs.alignment_matches + rhs.alignment_matches,
			insertions: lhs.insertions + rhs.insertions,
			deletions: lhs.deletions + rhs.deletions,
			skips: lhs.skips + rhs.skips
		}
	}
}
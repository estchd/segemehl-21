use crate::{Base, CIGAREntry};

pub struct SequenceBase {
	pub(crate) base: Base,
	pub(crate) cigar: CIGAREntry,
	pub(crate) quality: u8
}

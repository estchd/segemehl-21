use rand::{Rng, thread_rng};
use crate::{Base, CIGAREntry};
use crate::record_generation::generate_random_quality;
use crate::sequence_base::SequenceBase;

pub enum SequenceEntry {
	Base(SequenceBase),
	Skip,
	Deletion,
}

impl SequenceEntry {
	pub fn generate_random() -> Self {
		let mut rand = thread_rng();
		let entry_type = rand.gen_range(1..=100);
		if entry_type <= 60 {
			Self::generate_random_no_skips()
		}
		else if entry_type <= 80 {
			Self::generate_sequence_skip()
		}
		else {
			Self::generate_reference_skip()
		}
	}

	pub fn generate_random_skips_only() -> Self {
		let mut rand = thread_rng();
		let random = rand.gen_bool(0.5);

		if random{
			Self::generate_reference_skip()
		}
		else {
			Self::generate_sequence_skip()
		}
	}

	pub fn generate_random_no_reference_skips() -> Self {
		let random = thread_rng().gen_bool(0.5);
		if random {
			Self::generate_random_no_skips()
		}
		else {
			Self::generate_sequence_skip()
		}
	}

	pub fn generate_random_no_sequence_skips() -> Self {
		let random = thread_rng().gen_bool(0.5);
		if random {
			Self::generate_random_no_skips()
		}
		else {
			Self::generate_reference_skip()
		}
	}

	pub fn generate_random_no_skips() -> Self {
		let base = Base::generate_random();
		let cigar = CIGAREntry::generate_random();
		let quality = crate::record_generation::generate_random_quality();

		let sequence_base = SequenceBase {
			base,
			cigar,
			quality
		};
		Self::Base(sequence_base)
	}

	pub fn generate_random_soft_clip() -> Self {
		let base = Base::generate_random();
		let quality = generate_random_quality();
		let sequence_base = SequenceBase {
			base,
			cigar: CIGAREntry::S,
			quality
		};
		SequenceEntry::Base(sequence_base)
	}

	pub fn generate_sequence_skip() -> Self {
		Self::generate_insertion()
	}

	pub fn generate_insertion() -> Self {
		let base = Base::generate_random();
		let quality = generate_random_quality();
		let sequence_base = SequenceBase {
			base,
			cigar: CIGAREntry::I,
			quality
		};
		SequenceEntry::Base(sequence_base)
	}

	pub fn generate_reference_skip() -> Self {
		let random = thread_rng().gen_bool(0.5);
		if random {
			Self::generate_deletion()
		}
		else {
			Self::generate_skip()
		}
	}

	pub fn generate_deletion() -> Self {
		Self::Deletion
	}

	pub fn generate_skip() -> Self {
		Self::Skip
	}
}

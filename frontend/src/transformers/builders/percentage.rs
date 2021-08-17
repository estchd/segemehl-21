use crate::transformers::percentage::{PercentageTransformer, PerFilePercentageTransformer};

pub struct PercentageTransformerBuilder {}

impl PercentageTransformerBuilder {
	pub fn new() -> Self {
		PercentageTransformerBuilder {}
	}

	pub fn build(self, dataset_name: String) -> PercentageTransformer {
		PercentageTransformer {
			dataset_name
		}
	}
}

pub struct PerFilePercentageTransformerBuilder {}

impl PerFilePercentageTransformerBuilder {
	pub fn new() -> Self {
		Self {}
	}

	pub fn build(self, dataset_name: String) -> PerFilePercentageTransformer {
		PerFilePercentageTransformer {
			dataset_name
		}
	}
}

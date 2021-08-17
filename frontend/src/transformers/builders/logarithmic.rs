use crate::transformers::logarithmic::{LogarithmicTransformer, PerFileLogarithmicTransformer};

pub struct LogarithmicTransformerBuilder {
	toggle_element_id: Option<String>,
	base_element_id: Option<String>,
}

impl LogarithmicTransformerBuilder {
	pub fn new() -> Self {
		LogarithmicTransformerBuilder {
			toggle_element_id: None,
			base_element_id: None
		}
	}

	pub fn with_toggle(mut self, id: String) -> Self {
		self.toggle_element_id = Some(id);
		self
	}

	pub fn with_base(mut self, id: String) -> Self {
		self.base_element_id = Some(id);
		self
	}
	pub fn build(self, dataset_name: String) -> Result<LogarithmicTransformer,()> {
		let LogarithmicTransformerBuilder {
			toggle_element_id,base_element_id
		} = self;

		Ok(LogarithmicTransformer {
			toggle_element_id,
			dataset_name,
			base_element_id
		})
	}
}

pub struct PerFileLogarithmicTransformerBuilder {
	toggle_element_id: Option<String>,
	base_element_id: Option<String>,
}

impl PerFileLogarithmicTransformerBuilder {
	pub fn new() -> Self {
		Self {
			toggle_element_id: None,
			base_element_id: None
		}
	}

	pub fn with_toggle(mut self, id: String) -> Self {
		self.toggle_element_id = Some(id);
		self
	}

	pub fn with_base(mut self, id: String) -> Self {
		self.base_element_id = Some(id);
		self
	}
	pub fn build(self, dataset_name: String) -> Result<PerFileLogarithmicTransformer,()> {
		let PerFileLogarithmicTransformerBuilder {
			toggle_element_id,base_element_id
		} = self;

		Ok(PerFileLogarithmicTransformer {
			toggle_element_id,
			dataset_name,
			base_element_id
		})
	}
}
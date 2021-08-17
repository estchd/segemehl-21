use crate::transformers::multiple_select::{MultipleSelectTransformer, PerFileMultipleSelectTransformer};

pub struct MultipleSelectTransformerBuilder {
	transform_y_axis: Vec<bool>,
	select_element_ids: Vec<String>
}

impl MultipleSelectTransformerBuilder {
	pub fn new() -> Self {
		MultipleSelectTransformerBuilder {
			transform_y_axis: vec![],
			select_element_ids: vec![]
		}
	}

	pub fn add_select(mut self, element_id: String, transform_y_desc: bool) -> Self {
		self.transform_y_axis.push(transform_y_desc);
		self.select_element_ids.push(element_id);
		self
	}

	pub fn build(self, base_dataset_name: String) -> MultipleSelectTransformer {
		MultipleSelectTransformer {
			base_dataset_name,
			select_element_ids: self.select_element_ids,
			transform_y_axis: self.transform_y_axis
		}
	}
}

pub struct PerFileMultipleSelectTransformerBuilder {
	transform_y_axis: Vec<bool>,
	select_element_ids: Vec<String>
}

impl PerFileMultipleSelectTransformerBuilder {
	pub fn new() -> Self {
		Self {
			transform_y_axis: vec![],
			select_element_ids: vec![]
		}
	}

	pub fn add_select(mut self, element_id: String, transform_y_desc: bool) -> Self {
		self.transform_y_axis.push(transform_y_desc);
		self.select_element_ids.push(element_id);
		self
	}

	pub fn build(self, base_dataset_name: String) -> PerFileMultipleSelectTransformer {
		PerFileMultipleSelectTransformer {
			base_dataset_name,
			select_element_ids: self.select_element_ids,
			transform_y_axis: self.transform_y_axis
		}
	}
}

#![allow(dead_code)]

use crate::transformers::select::{SelectTransformer, PerFileSelectTransformer};

pub struct SelectTransformerBuilder {
	transform_y_axis: bool,
}

impl SelectTransformerBuilder {
	pub fn new() -> Self {
		SelectTransformerBuilder {
			transform_y_axis: false
		}
	}

	pub fn set_y_axis_transform(mut self, transform: bool) -> Self {
		self.transform_y_axis = transform;
		self
	}

	pub fn build(self, select_element_id: String, base_dataset_name: String) -> SelectTransformer {
		SelectTransformer {
			base_dataset_name,
			select_element_id,
			transform_y_axis: self.transform_y_axis
		}
	}
}

pub struct PerFileSelectTransformerBuilder {
	transform_y_axis: bool,
}

impl PerFileSelectTransformerBuilder {
	pub fn new() -> Self {
		Self {
			transform_y_axis: false
		}
	}

	pub fn set_y_axis_transform(mut self, transform: bool) -> Self {
		self.transform_y_axis = transform;
		self
	}

	pub fn build(self, select_element_id: String, base_dataset_name: String) -> PerFileSelectTransformer {
		PerFileSelectTransformer {
			base_dataset_name,
			select_element_id,
			transform_y_axis: self.transform_y_axis
		}
	}
}


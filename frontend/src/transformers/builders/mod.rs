#![allow(dead_code)]

pub mod logarithmic;
pub mod multiple_select;
pub mod percentage;
pub mod select;

use logarithmic::LogarithmicTransformerBuilder;
use crate::transformers::builders::select::{SelectTransformerBuilder, PerFileSelectTransformerBuilder};
use crate::transformers::builders::multiple_select::{MultipleSelectTransformerBuilder, PerFileMultipleSelectTransformerBuilder};
use crate::transformers::builders::percentage::{PercentageTransformerBuilder, PerFilePercentageTransformerBuilder};
use crate::transformers::builders::logarithmic::PerFileLogarithmicTransformerBuilder;

pub struct TransformerBuilder {

}

impl TransformerBuilder {
	pub fn new() -> Self {
		TransformerBuilder {}
	}

	pub fn logarithmic_transformer(self) -> LogarithmicTransformerBuilder {
		LogarithmicTransformerBuilder::new()
	}
	pub fn per_file_logarithmic_transformer(self) -> PerFileLogarithmicTransformerBuilder {
		PerFileLogarithmicTransformerBuilder::new()
	}

	pub fn select_transformer(self) -> SelectTransformerBuilder {
		SelectTransformerBuilder::new()
	}
	pub fn per_file_select_transformer(self) -> PerFileSelectTransformerBuilder {
		PerFileSelectTransformerBuilder::new()
	}

	pub fn multiple_select_transformer(self) -> MultipleSelectTransformerBuilder {
		MultipleSelectTransformerBuilder::new()
	}

	pub fn per_file_multiple_select_transformer(self) -> PerFileMultipleSelectTransformerBuilder {
		PerFileMultipleSelectTransformerBuilder::new()
	}

	pub fn percentage_transformer(self) -> PercentageTransformerBuilder {
		PercentageTransformerBuilder::new()
	}
	pub fn per_file_percentage_transformer(self) -> PerFilePercentageTransformerBuilder {
		PerFilePercentageTransformerBuilder::new()
	}
}

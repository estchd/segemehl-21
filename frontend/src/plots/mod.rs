use std::sync::Arc;

use crate::data_repository::DataRepository;
use crate::declarative_test::plot_config::PlotAxesConfig;
use crate::transformers::Transformer;

pub mod bar_plot;
pub mod builders;
pub mod stacked_bar_plot;
mod util;

pub trait Plot {
	fn draw(&self) -> Result<(),String>;
}

pub struct PlotBase {
	pub title: String,
	pub transformers: Vec<Arc<dyn Transformer + Send + Sync>>,
	pub data_repository: Arc<dyn DataRepository + Send + Sync>,
	pub axes_description: PlotAxesConfig,
	pub tick_mark_size:  i32
}

impl PlotBase {
	pub fn get_transformed_data(&self) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let current_repository = self.data_repository.clone();

		self.transformers
			.iter()
			.try_fold(
				current_repository,
				|data_repository, transformer| {
					transformer.transform_data(data_repository)
				})
	}

	pub fn get_transformed_axes_description(&self) -> Result<PlotAxesConfig, ()> {
		let current_axes_description = self.axes_description.clone();

		self.transformers
			.iter()
			.try_fold(
				current_axes_description,
				|axes_description, transformer| {
					let axes_description = transformer.transform_axes_config(axes_description)?;
					Ok(axes_description)
				})
	}
}

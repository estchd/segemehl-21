#![allow(dead_code)]

use std::sync::Arc;

use crate::data_repository::DataRepository;
use bar_plot::BarPlotBuilder;
use crate::declarative_test::plot_config::{AxisConfig, PlotAxesConfig};
use crate::plots::PlotBase;
use crate::transformers::Transformer;
use crate::plots::builders::stacked_bar_plot::StackedBarPlotBuilder;

pub mod bar_plot;
pub mod stacked_bar_plot;

pub struct PlotBuilder {
	transformers: Vec<Arc<dyn Transformer + Send + Sync>>,
	title: String,
	axes: PlotAxesConfig,
	tick_mark_size: i32,
}

impl PlotBuilder {
	pub fn new() -> Self {
		PlotBuilder {
			transformers: Vec::new(),
			title: String::new(),
			axes: Default::default(),
			tick_mark_size: 5
		}
	}

	pub fn with_title(mut self, title: String) -> Self {
		self.title = title;
		self
	}

	pub fn set_tick_mark_size(mut self, size: i32) -> Self {
		self.tick_mark_size = size;
		self
	}

	pub fn set_x_axis(mut self, config: Option<AxisConfig>) -> Self {
		self.axes.x_axis = config;
		self
	}

	pub fn set_y_axis(mut self, config: Option<AxisConfig>) -> Self {
		self.axes.y_axis = config;
		self
	}

	pub fn with_transformer(mut self, transformer: Arc<dyn Transformer + Send + Sync>) -> Self {
		self.transformers.push(transformer);
		self
	}

	pub fn bar_plot(self, data: Arc<dyn DataRepository + Send + Sync>) -> BarPlotBuilder {
		let plot_base = PlotBase {
			transformers: self.transformers,
			data_repository: data,
			title: self.title,
			axes_description: self.axes,
			tick_mark_size: self.tick_mark_size,
		};

		BarPlotBuilder::new(plot_base)
	}

	pub fn stacked_bar_plot(self, data: Arc<dyn DataRepository + Send + Sync>) -> StackedBarPlotBuilder {
		let plot_base = PlotBase {
			transformers: self.transformers,
			data_repository: data,
			title: self.title,
			axes_description: self.axes,
			tick_mark_size: self.tick_mark_size,
		};

		StackedBarPlotBuilder::new(plot_base)
	}
}

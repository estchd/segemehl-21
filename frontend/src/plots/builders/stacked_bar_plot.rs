use crate::plots::PlotBase;
use crate::declarative_test::plot_config::{Side, DatasetDescription};
use crate::plots::bar_plot::BarPlot;
use crate::accessories::Accessory;
use std::sync::Arc;
use crate::plots::stacked_bar_plot::StackedBarPlot;
use plotters::style::RGBColor;
use plotters::prelude::{BLUE, GREEN};

pub struct StackedBarPlotBuilder {
	pub(crate) plot_base: PlotBase,
	pub(crate) accessories: Vec<Arc<dyn Accessory>>,
	pub(crate) colors: Vec<RGBColor>
}

impl StackedBarPlotBuilder {
	pub fn new(base: PlotBase) -> Self {
		StackedBarPlotBuilder {
			plot_base: base,
			accessories: vec![],
			colors: vec![BLUE, GREEN]
		}
	}

	pub fn with_colors(mut self, colors: Vec<RGBColor>) -> Self {
		self.colors = colors;
		self
	}

	pub fn build(self, canvas_id: String, dataset_name: String) -> Result<StackedBarPlot,()> {
		Ok(StackedBarPlot {
			base: self.plot_base,
			canvas_id,
			dataset_name,
			colors: self.colors
		})
	}
}

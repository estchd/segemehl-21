use crate::declarative_test::plot_config::{DatasetDescription, Side};
use crate::plots::bar_plot::BarPlot;
use crate::plots::PlotBase;
use std::sync::Arc;
use crate::accessories::Accessory;

pub struct BarPlotBuilder {
	pub(crate) plot_base: PlotBase,
	pub(crate) base_side: Side,
	pub(crate) reversed: bool,
	pub(crate) accessories: Vec<Arc<dyn Accessory>>
}

impl BarPlotBuilder {
	pub fn new(base: PlotBase) -> Self {
		BarPlotBuilder {
			plot_base: base,
			base_side: Side::Bottom,
			reversed: false,
			accessories: vec![]
		}
	}

	pub fn with_base_side(mut self, base_side: Side) -> Self {
		self.base_side = base_side;
		self
	}

	pub fn with_reversed(mut self, reversed: bool) -> Self {
		self.reversed = reversed;
		self
	}

	pub fn build(self, canvas_id: String, dataset: DatasetDescription) -> Result<BarPlot,()> {
		Ok(BarPlot {
			base: self.plot_base,
			canvas_id,
			dataset,
			base_side: self.base_side,
			reversed: self.reversed
		})
	}
}

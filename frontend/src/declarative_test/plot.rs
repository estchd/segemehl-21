#![allow(dead_code)]

use std::sync::Arc;

use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::ranged1d::IntoSegmentedCoord;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters_canvas::CanvasBackend;

use crate::data_repository::DataRepository;
use crate::declarative_test::draw::Draw;
use crate::declarative_test::plot_config::PlotAxesConfig;
use crate::transformers::Transformer;

pub struct Plot {
	title: String,
	axes: PlotAxesConfig,
	transformers: Vec<Box<dyn Transformer>>,
	draws: Vec<Box<dyn Draw>>,
	drawing_area: DrawingArea<CanvasBackend, Shift>,
	data_repository: Arc<dyn DataRepository>
}

impl Plot {
	pub fn plot(&self) -> Result<(),()> {
		let (current_repository, _current_axes_config) = (self.data_repository.clone(),self.axes.clone());

		/*

		let (current_repository, current_axes_config) = &self.transformers
			.iter()
			.try_fold(
				(current_repository, current_axes_config),
				|(data_repository, axes_config), transformer| {
					transformer.transform(data_repository, axes_config)
		})?;

		*/

		let _ctx = ChartBuilder::on(&self.drawing_area)
			.set_label_area_size(LabelAreaPosition::Left, 10)
			.set_label_area_size(LabelAreaPosition::Bottom, 10)
			.caption("Hello World", ("sans-serif", 10))
			.build_cartesian_2d((0..100usize).into_segmented(), 0.0..200.0).map_err(|_| ())?
			.set_secondary_coord(0.0..100.0, 0.0..200.0);


		for draw in &self.draws {
			draw.draw(&self.drawing_area, current_repository.clone())?
		}


		unimplemented!()
	}
}
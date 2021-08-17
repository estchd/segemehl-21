use std::cmp::max;
use std::sync::Arc;

use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use crate::data_repository::DataRepository;
use crate::declarative_test::plot_config::{Axis, AxisConfig, DatasetDescription, PlotAxesConfig, Side};
use crate::plots::{Plot, PlotBase};
use crate::plots::util::{get_data_from_repository, set_label_area_sizes, Orientation, BarPlotAxesSize, calculate_area_size};

static CAPTION_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif", 20);
static DESCRIPTION_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif", 15);
static LABEL_TEXT_STYLE_INFO: (&'static str, u32) = ("sans-serif", 10);

pub struct BarPlot {
	pub(crate) base: PlotBase,
	pub(crate) canvas_id: String,
	pub(crate) dataset: DatasetDescription,
	pub(crate) base_side: Side,
	pub(crate) reversed: bool
}

impl BarPlot {
	fn draw_horizontal_plot(&self, mut chart_builder: ChartBuilder<CanvasBackend>, x_max: usize, y_max: f64, data: Vec<(usize, f64)>, axes_description: PlotAxesConfig) -> Result<(),()> {
		let x_max = max(x_max, 1);
		let y_max = if y_max == 0.0 || y_max.is_nan() {
			1.0
		}
		else {
			y_max
		};

		let mut ctx = match (self.base_side, self.reversed) {
			(Side::Top, false) => chart_builder.build_cartesian_2d((x_max..0).into_segmented(), y_max..0.0).map_err(|_| ())?,
			(Side::Top, true) => chart_builder.build_cartesian_2d((0..x_max).into_segmented(), y_max..0.0).map_err(|_| ())?,
			(Side::Bottom, false) => chart_builder.build_cartesian_2d((0..x_max).into_segmented(), 0.0..y_max).map_err(|_| ())?,
			(Side::Bottom, true) => chart_builder.build_cartesian_2d((x_max..0).into_segmented(), 0.0..y_max).map_err(|_| ())?,
			_ => return Err(())
		};

		let x_desc = axes_description.clone().x_axis.map(|item| item.description.clone()).unwrap_or("".to_string());

		let y_desc = axes_description.clone().y_axis.map(|item| item.description.clone()).unwrap_or("".to_string());

		ctx
			.configure_mesh()
			.disable_x_mesh()
			.bold_line_style(&WHITE.mix(0.3))
			.axis_desc_style(DESCRIPTION_TEXT_STYLE_INFO)
			.set_all_tick_mark_size(self.base.tick_mark_size)
			.label_style(LABEL_TEXT_STYLE_INFO)
			.x_desc(x_desc)
			.y_desc(y_desc)
			.draw()
			.map_err(|_| ())?;

		ctx.draw_series(data.iter().map(|(x, y)| {
			let x0 = SegmentValue::Exact(*x);
			let x1 = SegmentValue::Exact(x + 1);
			let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], BLUE.filled());
			bar.set_margin(0, 0, 0, 0);
			bar
		})).map_err(|_| ())?;

		Ok(())
	}

	fn draw_vertical_plot(&self, mut chart_builder: ChartBuilder<CanvasBackend>, x_max: usize, y_max: f64, data: Vec<(usize, f64)>, axes_description: PlotAxesConfig) -> Result<(),()> {
		let x_max = max(x_max, 1);
		let y_max = if y_max == 0.0 || y_max.is_nan() {
			1.0
		}
		else {
			y_max
		};

		let mut ctx = match (self.base_side, self.reversed) {
			(Side::Left, false) => chart_builder.build_cartesian_2d(0.0..y_max, (0..x_max).into_segmented()).map_err(|_| ())?,
			(Side::Left, true) => chart_builder.build_cartesian_2d(0.0..y_max, (x_max..0).into_segmented()).map_err(|_| ())?,
			(Side::Right, false) => chart_builder.build_cartesian_2d(y_max..0.0, (0..x_max).into_segmented()).map_err(|_| ())?,
			(Side::Right, true) => chart_builder.build_cartesian_2d(y_max..0.0, (x_max..0).into_segmented()).map_err(|_| ())?,
			_ => return Err(())
		};

		let x_desc = axes_description.clone().y_axis.map(|item| item.description.clone()).unwrap_or("".to_string());

		let y_desc = axes_description.clone().y_axis.map(|item| item.description.clone()).unwrap_or("".to_string());

		ctx
			.configure_mesh()
			.disable_x_mesh()
			.bold_line_style(&WHITE.mix(0.3))
			.axis_desc_style(DESCRIPTION_TEXT_STYLE_INFO)
			.set_all_tick_mark_size(self.base.tick_mark_size)
			.label_style(LABEL_TEXT_STYLE_INFO)
			.x_desc(x_desc)
			.y_desc(y_desc)
			.draw()
			.map_err(|_| ())?;

		ctx.draw_series(data.iter().map(|(x, y)| {
			let x0 = SegmentValue::Exact(*x);
			let x1 = SegmentValue::Exact(x + 1);
			let mut bar = Rectangle::new([(0.0, x0), (*y, x1)], BLUE.filled());
			bar.set_margin(0, 0, 0, 0);
			bar
		})).map_err(|_| ())?;

		Ok(())
	}

	fn calculate_plot_areas(&self, drawing_area: &DrawingArea<CanvasBackend, Shift>, axes_config: PlotAxesConfig, x_max: usize, y_max: f64, base_side: Side, reversed: bool) -> Result<BarPlotAxesSize,()> {
		let biggest_y_label = format!("{:.2}", y_max);
		let biggest_x_label = format!("{}", x_max);

		let plot_orientation = match self.base_side {
			Side::Top |
			Side::Bottom => Orientation::Horizontal,
			Side::Left |
			Side::Right => Orientation::Vertical,
		};

		let (biggest_horizontal_label, biggest_vertical_label) = match plot_orientation {
			Orientation::Horizontal => (biggest_x_label, biggest_y_label),
			Orientation::Vertical => (biggest_y_label, biggest_x_label)
		};

		let (horizontal_area, vertical_area) = match base_side {
			Side::Top |
			Side::Bottom => {
				let horizontal = self.calculate_optional_area_size(
					drawing_area,
					axes_config.x_axis,
					biggest_horizontal_label.clone(),
					Orientation::Horizontal
				)?.unwrap_or(0);
				let vertical = self.calculate_optional_area_size(
					drawing_area,
					axes_config.y_axis,
					biggest_vertical_label.clone(),
					Orientation::Vertical
				)?.unwrap_or(0);

				(horizontal, vertical)
			}
			Side::Left |
			Side::Right => {
				let horizontal = self.calculate_optional_area_size(
					drawing_area,
					axes_config.y_axis,
					biggest_horizontal_label.clone(),
					Orientation::Horizontal
				)?.unwrap_or(0);
				let vertical = self.calculate_optional_area_size(
					drawing_area,
					axes_config.x_axis,
					biggest_vertical_label.clone(),
					Orientation::Vertical
				)?.unwrap_or(0);

				(horizontal, vertical)
			}
		};

		let (left,right,top,bottom) = match (base_side, reversed) {
			(Side::Top, false) => {
				(0,vertical_area,horizontal_area,0)
			}
			(Side::Top, true) => {
				(vertical_area, 0, horizontal_area, 0)
			}
			(Side::Bottom, false) => {

				(vertical_area, 0, 0, horizontal_area)
			}
			(Side::Bottom, true) => {
				(0, vertical_area, 0, horizontal_area)
			}
			(Side::Left, false) => {
				(vertical_area, 0, horizontal_area, 0)
			}
			(Side::Left, true) => {
				(vertical_area, 0, 0, horizontal_area)
			}
			(Side::Right, false) => {
				(0, vertical_area, 0, horizontal_area)
			}
			(Side::Right, true) => {
				(0, vertical_area, horizontal_area, 0)
			}
		};

		Ok(BarPlotAxesSize {
			top,
			bottom,
			left,
			right
		})
	}

	fn calculate_optional_area_size(&self, drawing_area: &DrawingArea<CanvasBackend, Shift>, axis_config: Option<AxisConfig>, max_label: String, axis_orientation: Orientation) -> Result<Option<u32>,()> {
		let size = axis_config.map(
			|config| {
				let sizes = calculate_area_size(
					config.description.clone(),
					DESCRIPTION_TEXT_STYLE_INFO,
					max_label,
					LABEL_TEXT_STYLE_INFO,
					config.margin,
					self.base.tick_mark_size,
					drawing_area
				).map_err(|_| ())?;

				Ok(match axis_orientation {
					Orientation::Horizontal => sizes.1,
					Orientation::Vertical => sizes.0
				})
			}
		);

		size.transpose()
	}
}

impl Plot for BarPlot {
	fn draw(&self) -> Result<(), String> {
		let transformed_repository = self.base.get_transformed_data()
			.map_err(|err| format!("Error getting Transformed Data: {}", err))?;
		let transformed_axes = self.base.get_transformed_axes_description()
			.map_err(|_| "Error getting Transformed Axes")?;

		let data = get_data_from_repository(self.dataset.clone(), transformed_repository);
		let data = data.ok_or(format!("No Data for Dataset: {}", self.dataset.name.clone()))?;

		let x_max = data.len();
		let y_max = data.iter().reduce(
			|a,b| {
				if *a > *b {
					return a;
				}
				b
			}
		).map(|item| *item).unwrap_or(0.0) * 1.1;

		let indexed_data: Vec<(usize, f64)> = data
			.into_iter()
			.enumerate()
			.filter(|(_,item)|
				!item.is_nan() && *item != 0.0 && item.is_finite()
			).collect();

		let canvas = CanvasBackend::new(self.canvas_id.as_str())
			.ok_or("Error Getting Canvas")?;

		let drawing_area = canvas.into_drawing_area();

		let plot_areas = self.calculate_plot_areas(
			&drawing_area,
			transformed_axes.clone(),
			x_max,
			y_max,
			self.base_side,
			self.reversed
		).map_err(|_| "Error calculating Plot Areas")?;


		drawing_area.fill(&WHITE)
			.map_err(|err| format!("Error Filling Drawing Area: {}", err.to_string()))?;

		let mut ctx = ChartBuilder::on(&drawing_area);

		set_label_area_sizes(&mut ctx, plot_areas);

		ctx.caption(self.base.title.clone(), CAPTION_TEXT_STYLE_INFO);

		match self.base_side {
			Side::Top |
			Side::Bottom => self.draw_horizontal_plot(ctx, x_max, y_max, indexed_data, transformed_axes.clone())
				.map_err(|_| "Error Drawing Horizontal Plot")?,
			Side::Left |
			Side::Right => self.draw_vertical_plot(ctx, x_max, y_max, indexed_data, transformed_axes.clone())
				.map_err(|_| "Error Drawing Vertical Plot")?
		}

		drawing_area.present().map_err(|err| format!("Error Presenting: {}", err.to_string()))
	}
}

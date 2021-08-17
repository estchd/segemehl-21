use crate::plots::{PlotBase, Plot};
use crate::declarative_test::plot_config::{Side, DatasetDescription, Axis, AxisConfig, PlotAxesConfig};
use crate::plots::util::{get_data_from_repository, set_label_area_sizes, Orientation, BarPlotAxesSize, calculate_area_size};
use crate::file_list::FILE_LIST;
use plotters::style::RGBColor;
use std::cmp::{min, max};
use plotters_canvas::CanvasBackend;
use plotters::drawing::{IntoDrawingArea, DrawingArea};
use plotters::coord::Shift;
use crate::{DESCRIPTION_TEXT_STYLE_INFO, LABEL_TEXT_STYLE_INFO, CAPTION_TEXT_STYLE_INFO};
use plotters::prelude::{WHITE, Color, SegmentValue};
use plotters::chart::ChartBuilder;
use plotters::coord::ranged1d::IntoSegmentedCoord;
use plotters::element::Rectangle;
use crate::console_log;

pub struct StackedBarPlot {
	pub(crate) base: PlotBase,
	pub(crate) canvas_id: String,
	pub(crate) dataset_name: String,
	pub(crate) colors: Vec<RGBColor>,
}

impl StackedBarPlot {
	fn calculate_plot_areas(&self, drawing_area: &DrawingArea<CanvasBackend, Shift>, axes_config: PlotAxesConfig, x_max: usize, y_max: f64, base_side: Side, reversed: bool) -> Result<BarPlotAxesSize,()> {
		let biggest_y_label = format!("{:.2}", y_max);
		let biggest_x_label = format!("{}", x_max);

		let plot_orientation = Orientation::Horizontal;

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

impl Plot for StackedBarPlot {
	fn draw(&self) -> Result<(), String> {
		let transformed_repository = self.base.get_transformed_data()
			.map_err(|err| format!("Error getting Transformed Data: {}", err))?;
		let transformed_axes = self.base.get_transformed_axes_description()
			.map_err(|_| "Error getting Transformed Axes")?;

		let file_names = FILE_LIST.lock().unwrap();
		let file_names: Vec<String> = file_names.iter().filter(|(_, value)| {
			value.is_some()
		}).map(|(key,_)| key.clone()).collect();

		let data: Result<Vec<Vec<f64>>,&str> = file_names.iter().map(
			|name| {
				let dataset_name = format!("{}_{}", name, self.dataset_name);
				let data = get_data_from_repository(
					DatasetDescription {
						name: dataset_name,
						axis: Axis::X
					},
					transformed_repository.clone()
				);

				data.ok_or("Dataset not found")
			}
		).collect();

		let data = data.map_err(|err| err.to_string())?;

		let mut stacked_data = Vec::<Vec<f64>>::new();

		for data in data {
			for stacked_data in &mut stacked_data {
				accumulate_vector(stacked_data, &data);
			}
			stacked_data.push(data);
		}

		let x_max = stacked_data.iter().reduce(
			|a, b| {
				if a.len() < b.len() {
					return a;
				}
				b
			}
		).map(|vec| vec.len()).unwrap_or(0);
		let x_max = max(x_max, 1);


		let indexed_data: Vec<Vec<(usize, f64)>> = stacked_data
			.into_iter()
			.map(
				|vec| {
						vec.into_iter()
							.enumerate()
							.filter(|(_,item)| {
								!item.is_nan() && *item != 0.0 && item.is_finite()
							}
						).collect()
					}
			).collect();

		let y_max = indexed_data.first().map(
			|vec| {
				vec.iter().reduce(
					|a, b| {
						if a.1 > b.1 {
							return a;
						}
						b
					}
				)
			}
		).flatten().map(|(_, item)| *item).unwrap_or(0.0);

		let y_max = y_max * 1.1;
		let y_max = if y_max == 0.0 || y_max.is_nan() {
			1.0
		}
		else {
			y_max
		};

		let canvas = CanvasBackend::new(self.canvas_id.as_str())
			.ok_or("Error getting Canvas")?;

		let drawing_area = canvas.into_drawing_area();

		let plot_areas = self.calculate_plot_areas(
			&drawing_area,
			transformed_axes.clone(),
			x_max,
			y_max,
			Side::Bottom,
			false
		).map_err(|_| "Error calculating Plot Areas")?;

		drawing_area.fill(&WHITE)
			.map_err(|err| format!("Error filling Drawing Area: {}", err.to_string()))?;

		let mut chart_builder = ChartBuilder::on(&drawing_area);

		set_label_area_sizes(&mut chart_builder, plot_areas);

		chart_builder.caption(self.base.title.clone(), CAPTION_TEXT_STYLE_INFO);

		let mut chart_context = chart_builder
			.build_cartesian_2d((0..x_max).into_segmented(), 0.0..y_max)
			.map_err(|_| "Error Building Coordinates")?;

		let x_desc = transformed_axes.clone().x_axis.map(|item| item.description.clone()).unwrap_or("".to_string());
		let y_desc = transformed_axes.clone().y_axis.map(|item| item.description.clone()).unwrap_or("".to_string());

		chart_context
			.configure_mesh()
			.disable_x_mesh()
			.bold_line_style(&WHITE.mix(0.3))
			.axis_desc_style(DESCRIPTION_TEXT_STYLE_INFO)
			.set_all_tick_mark_size(self.base.tick_mark_size)
			.label_style(LABEL_TEXT_STYLE_INFO)
			.x_desc(x_desc)
			.y_desc(y_desc)
			.draw()
			.map_err(|_| "Error Drawing Grid")?;

		let mut color_iter = self.colors.iter().cycle();

		for i in 0..min(file_names.len(), indexed_data.len()) {
			let name = &file_names[i];
			let data = &indexed_data[i];

			let color = color_iter.next().unwrap();

			chart_context.draw_series(data.iter().map(|(x,y)| {
				let x0 = SegmentValue::Exact(*x);
				let x1 = SegmentValue::Exact(x+1);
				let mut bar = Rectangle::new([(x0,0.0), (x1, *y)], color.filled());
				bar.set_margin(0,0,0,0);
				bar
			}))
				.map_err(|_| "Error Drawing Series")?
				.label(name);
		}

		drawing_area.present().map_err(|err| format!("Error presenting: {}", err.to_string()))
	}
}

fn accumulate_vector(acc: &mut Vec<f64>, data: &Vec<f64>) {
	for i in 0..min(acc.len(),data.len()) {
		acc[i] = acc[i] + data[i];
	}
}
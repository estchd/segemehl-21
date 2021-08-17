use crate::declarative_test::plot_config::{DatasetDescription, Axis};
use std::sync::Arc;
use crate::data_repository::DataRepository;
use plotters::prelude::{IntoTextStyle, DrawingBackend};
use plotters::coord::CoordTranslate;
use plotters::drawing::{DrawingArea, DrawingAreaErrorKind};
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters_canvas::CanvasBackend;

pub fn get_data_from_repository(dataset: DatasetDescription, repository: Arc<dyn DataRepository>) -> Option<Vec<f64>> {
	let one_dimensional = repository.get_one_dimensional_data(dataset.name.clone());
	let two_dimensional = repository.get_two_dimensional_data(dataset.name.clone());

	match dataset.axis {
		Axis::X => {
			let one_dimensional = one_dimensional.map(|vec| vec.clone());
			let two_dimensional = optional_make_one_dimensional(two_dimensional, Axis::X);

			one_dimensional.or(two_dimensional)
		}
		Axis::Y => {
			optional_make_one_dimensional(two_dimensional, Axis::Y)
		}
	}
}

pub fn calculate_area_size<'a, 'b, DS: IntoTextStyle<'a>, LS: IntoTextStyle<'b>,DB: DrawingBackend, CT: CoordTranslate>(desc: String, desc_style: DS, biggest_label: String, label_style: LS, margin: u32, tick_mark_size: i32, drawing_area: &DrawingArea<DB,CT>) -> Result<(u32, u32), DrawingAreaErrorKind<DB::ErrorType>> {
	let desc_style = desc_style.into_text_style(drawing_area);

	let desc_size = drawing_area.estimate_text_size(desc.as_str(), &desc_style)?;

	if tick_mark_size < 0 {return Ok((desc_size.1, desc_size.1));}

	let tick_mark_size = tick_mark_size as u32;

	let label_style = label_style.into_text_style(drawing_area);

	let biggest_label_size = drawing_area.estimate_text_size(biggest_label.as_str(), &label_style)?;

	let left_size = desc_size.1 + biggest_label_size.0 + tick_mark_size + margin;

	let bottom_size = desc_size.1 + biggest_label_size.1 + tick_mark_size + margin;

	Ok((left_size, bottom_size))
}

pub fn optional_make_one_dimensional(data: Option<Vec<(f64, f64)>>, axis: Axis) -> Option<Vec<f64>> {
	data.map(|vec|
		make_one_dimensional(vec, axis)
	)
}

pub fn make_one_dimensional(data: Vec<(f64, f64)>, axis: Axis) -> Vec<f64> {
	match axis {
		Axis::X => {
			data.iter().map(|(a,_)| *a).clone().collect()
		}
		Axis::Y => {
			data.iter().map(|(_,b)| *b).clone().collect()
		}
	}
}

pub fn set_label_area_sizes(ctx: &mut ChartBuilder<CanvasBackend>, area_sizes: BarPlotAxesSize) {
	set_label_area_size(ctx, area_sizes.top, LabelAreaPosition::Top);
	set_label_area_size(ctx, area_sizes.bottom, LabelAreaPosition::Bottom);
	set_label_area_size(ctx, area_sizes.left, LabelAreaPosition::Left);
	set_label_area_size(ctx, area_sizes.right, LabelAreaPosition::Right);
}

pub fn set_label_area_size(ctx: &mut ChartBuilder<CanvasBackend>, area_size: u32, area_position: LabelAreaPosition) {
	ctx.set_label_area_size(area_position, area_size);
}

pub enum Orientation {
	Horizontal,
	Vertical,
}

pub struct BarPlotAxesSize {
	pub(crate) top: u32,
	pub(crate) bottom: u32,
	pub(crate) left: u32,
	pub(crate) right: u32
}
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlotConfig {
	#[serde(rename = "canvas_id")]
	pub canvas_id: String,

	#[serde(rename = "title")]
	pub title: String,

	#[serde(rename = "axes")]
	pub axes: PlotAxesConfig,

	#[serde(rename = "draws")]
	pub draws: Vec<Draw>,

	#[serde(rename = "transformers")]
	pub transformers: Vec<Transformer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Transformer {
	#[serde(rename = "logarithmic")]
	Logarithmic{ config: LogarithmicTransformerConfig }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogarithmicTransformerConfig {
	#[serde(rename = "name")]
	pub name: String,

	#[serde(rename = "input")]
	pub input: DatasetDescription,

	#[serde(rename = "transform_axes_description")]
	pub transform_axes_description: Vec<Side>,

	#[serde(rename = "do_log_id")]
	pub do_log_id: String,

	#[serde(rename = "log_base_id")]
	pub log_base_id: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Draw {
	#[serde(rename = "bar")]
	Bar { config: BarDrawConfig },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BarDrawConfig {
	#[serde(rename = "x_data")]
	pub x_data: DatasetDescription,

	#[serde(rename = "y_data")]
	pub y_data: DatasetDescription,

	#[serde(rename = "x_direction")]
	pub x_direction: PlotDirection,

	#[serde(rename = "y_direction")]
	pub y_direction: PlotDirection,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PlotAxesConfig {
	#[serde(rename = "left", default)]
	pub x_axis: Option<AxisConfig>,

	#[serde(rename = "right", default)]
	pub y_axis: Option<AxisConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AxisConfig {
	#[serde(rename = "description")]
	pub description: String,

	#[serde(rename = "margin", default)]
	pub margin: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatasetDescription {
	#[serde(rename = "name")]
	pub name: String,

	#[serde(rename = "axis")]
	pub axis: Axis,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Axis {
	#[serde(rename = "x")]
	X,

	#[serde(rename = "y")]
	Y,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Side {
	#[serde(rename = "top")]
	Top,

	#[serde(rename = "bottom")]
	Bottom,

	#[serde(rename = "left")]
	Left,

	#[serde(rename = "right")]
	Right
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum PlotDirection {
	#[serde(rename = "up")]
	Up,

	#[serde(rename = "down")]
	Down,

	#[serde(rename = "left")]
	Left,

	#[serde(rename = "right")]
	Right
}
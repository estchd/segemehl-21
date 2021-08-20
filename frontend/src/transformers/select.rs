use std::collections::HashMap;
use std::sync::Arc;

use web_sys::HtmlSelectElement;

use crate::declarative_test::plot_config::{AxisConfig, PlotAxesConfig};
use crate::transformers::Transformer;
use crate::util::get_cast_element;
use crate::data_repository::DataRepository;
use crate::data_repository::transformer::TransformRepository;
use crate::file_list::FILE_LIST;

pub struct SelectTransformer {
	pub(crate) base_dataset_name: String,
	pub(crate) select_element_id: String,
	pub(crate) transform_y_axis: bool
}

impl Transformer for SelectTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let select_value = self.get_select_value().map_err(|_| "Error getting Select Value")?;

		let transformed_dataset_name = format!("{}_{}", select_value, self.base_dataset_name.clone());

		let selected_dataset = data_repository.get_one_dimensional_data(transformed_dataset_name.clone())
			.ok_or(format!("Selected Dataset {} not found", transformed_dataset_name.clone()))?;

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		update_hashmap.insert(self.base_dataset_name.clone(), selected_dataset);

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		if !self.transform_y_axis {
			return Ok(axes_config);
		}

		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{} {}", self.get_select_value()?, config.description),
					margin: config.margin
				})
			}
		};

		let new_config = PlotAxesConfig {
			y_axis: y_config,
			x_axis: axes_config.x_axis
		};

		Ok(new_config)
	}
}

impl SelectTransformer {
	fn get_select_value(&self) -> Result<String, ()> {
		let element = get_cast_element::<HtmlSelectElement>(self.select_element_id.clone()).ok_or(())?;
		Ok(element.value())
	}
}

pub struct PerFileSelectTransformer {
	pub(crate) base_dataset_name: String,
	pub(crate) select_element_id: String,
	pub(crate) transform_y_axis: bool
}

impl Transformer for PerFileSelectTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let files = FILE_LIST.lock().unwrap();
		let files = files.iter().filter(|(_,(_,value))| value.is_some());

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		for (name, _) in files {
			let select_value = self.get_select_value().map_err(|_| "Error getting Select Value")?;

			let transformed_dataset_name = format!("{}_{}_{}", name, select_value, self.base_dataset_name.clone());

			let selected_dataset = data_repository.get_one_dimensional_data(transformed_dataset_name.clone())
			                                      .ok_or(format!("Selected Dataset {} not found", transformed_dataset_name.clone()))?;

			update_hashmap.insert(format!("{}_{}", name, self.base_dataset_name.clone()), selected_dataset);
		}

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		if !self.transform_y_axis {
			return Ok(axes_config);
		}

		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{} {}", self.get_select_value()?, config.description),
					margin: config.margin
				})
			}
		};

		let new_config = PlotAxesConfig {
			y_axis: y_config,
			x_axis: axes_config.x_axis
		};

		Ok(new_config)
	}
}

impl PerFileSelectTransformer {
	fn get_select_value(&self) -> Result<String, ()> {
		let element = get_cast_element::<HtmlSelectElement>(self.select_element_id.clone()).ok_or(())?;
		Ok(element.value())
	}
}

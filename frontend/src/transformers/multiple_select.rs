use std::collections::HashMap;
use std::sync::Arc;

use web_sys::HtmlSelectElement;

use crate::declarative_test::plot_config::{AxisConfig, PlotAxesConfig};
use crate::transformers::Transformer;
use crate::util::get_cast_element;
use crate::data_repository::DataRepository;
use crate::data_repository::transformer::TransformRepository;
use crate::file_list::FILE_LIST;
use crate::console_log;

pub struct MultipleSelectTransformer {
	pub(crate) base_dataset_name: String,
	pub(crate) select_element_ids: Vec<String>,
	pub(crate) transform_y_axis: Vec<bool>
}

impl Transformer for MultipleSelectTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let transformed_dataset_name = self.get_transformed_dataset_name().map_err(|_| "Error getting Transformed Dataset Name")?;

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
		let mut iter = self.select_element_ids.iter().zip(self.transform_y_axis.iter());

		let transformed_config = iter.try_fold(
			axes_config,
			|config, (element_id, transform)| {
				if !*transform {
					return Ok(config);
				}

				let y_config = match config.y_axis {
					None => None,
					Some(config) => {
						Some(AxisConfig {
							description: format!("{} {}", self.get_single_select_value(element_id.clone())?, config.description),
							margin: config.margin
						})
					}
				};

				let new_config = PlotAxesConfig {
					y_axis: y_config,
					x_axis: config.x_axis
				};

				Ok(new_config)
			}
		)?;

		Ok(transformed_config)
	}
}

impl MultipleSelectTransformer {
	fn get_single_select_value(&self, element_id: String) -> Result<String, ()> {
		let element = get_cast_element::<HtmlSelectElement>(element_id).ok_or(())?;
		Ok(element.value())
	}

	fn get_transformed_dataset_name(&self) -> Result<String, ()> {
		let mut value = self.base_dataset_name.clone();

		for element_id in &self.select_element_ids {
			let element_value = self.get_single_select_value(element_id.clone())?;
			value = format!("{}_{}", element_value, value);
		}

		Ok(value)
	}
}

pub struct PerFileMultipleSelectTransformer {
	pub(crate) base_dataset_name: String,
	pub(crate) select_element_ids: Vec<String>,
	pub(crate) transform_y_axis: Vec<bool>
}

impl Transformer for PerFileMultipleSelectTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let files = FILE_LIST.lock().unwrap();
		let files = files.iter().filter(|(_,value)| value.is_some());

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		let transformed_dataset_name = self.get_transformed_dataset_name().map_err(|err| format!("Error getting Transformed Dataset Name: {}", err))?;

		for (name, _) in files {
			let transformed_dataset_name = format!("{}_{}", name, &transformed_dataset_name);

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
		let mut iter = self.select_element_ids.iter().zip(self.transform_y_axis.iter());

		let transformed_config = iter.try_fold(
			axes_config,
			|config, (element_id, transform)| {
				if !*transform {
					return Ok(config);
				}

				let y_config = match config.y_axis {
					None => None,
					Some(config) => {
						Some(AxisConfig {
							description: format!("{} {}", self.get_single_select_value(element_id.clone())?, config.description),
							margin: config.margin
						})
					}
				};

				let new_config = PlotAxesConfig {
					y_axis: y_config,
					x_axis: config.x_axis
				};

				Ok(new_config)
			}
		)?;

		Ok(transformed_config)
	}
}

impl PerFileMultipleSelectTransformer {
	fn get_single_select_value(&self, element_id: String) -> Result<String, ()> {
		let element = get_cast_element::<HtmlSelectElement>(element_id).ok_or(())?;
		Ok(element.value())
	}

	fn get_transformed_dataset_name(&self) -> Result<String, String> {
		let mut value = self.base_dataset_name.clone();

		for element_id in &self.select_element_ids {
			let element_value = self.get_single_select_value(element_id.clone())
				.map_err(|_| format!("Error getting Select value for element id: {}", element_id.clone()))?;
			value = format!("{}_{}", element_value, value);
		}

		console_log!("{}", value);

		Ok(value)
	}
}


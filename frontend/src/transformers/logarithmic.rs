use std::collections::HashMap;
use std::sync::Arc;

use web_sys::HtmlInputElement;

use crate::declarative_test::plot_config::{AxisConfig, PlotAxesConfig};
use crate::transformers::Transformer;
use crate::util::get_cast_element;
use crate::data_repository::DataRepository;
use crate::data_repository::transformer::TransformRepository;
use crate::file_list::FILE_LIST;

pub struct LogarithmicTransformer {
	pub(crate) toggle_element_id: Option<String>,
	pub(crate) base_element_id: Option<String>,
	pub(crate) dataset_name: String
}

impl Transformer for LogarithmicTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let toggle = self.get_toggle_value().map_err(|err| format!("Error getting toggle value: {}", err))?;
		let base = self.get_base_value().map_err(|_| "Error getting base value")?;

		if toggle && base <= 1.0 {
			return Err(format!("Invalid Base {} and toggle enabled", base));
		}

		let data = data_repository.get_one_dimensional_data(self.dataset_name.clone())
			.ok_or(format!("No Data for Dataset: {}", self.dataset_name.clone()))?;

		let data: Vec<f64> = data.into_iter().map(|item|
			return if toggle {
				item.log(base)
			}
			else {
				item
			}
		).collect();

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		update_hashmap.insert(self.dataset_name.clone(), data);

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		let toggle = self.get_toggle_value().map_err(|_| ())?;
		let base = self.get_base_value()?;

		if !toggle {
			return Ok(axes_config);
		}
		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{}, scaled with logarithm base {}", config.description, base),
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

impl LogarithmicTransformer {
	fn get_toggle_value(&self) -> Result<bool, String> {
		Ok(match &self.toggle_element_id {
			None => false,
			Some(id) => {
				let element = get_cast_element::<HtmlInputElement>(id.clone());
				match element {
					None => return Err("No Element with ID".to_string()),
					Some(element) => element.checked()
				}

			}
		})
	}

	fn get_base_value(&self) -> Result<f64,()> {
		Ok(match &self.base_element_id {
			None => 2.0,
			Some(id) => {
				let element = get_cast_element::<HtmlInputElement>(id.clone());
				match element {
					None => return Err(()),
					Some(element) => element.value().as_str().parse().map_err(|_| ())?
				}
			}
		})
	}
}

pub struct PerFileLogarithmicTransformer {
	pub(crate) toggle_element_id: Option<String>,
	pub(crate) base_element_id: Option<String>,
	pub(crate) dataset_name: String
}

impl Transformer for PerFileLogarithmicTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let files = FILE_LIST.lock().unwrap();
		let files = files.iter().filter(|(_,(_,value))| value.is_some());

		let toggle = self.get_toggle_value().map_err(|err| format!("Error getting toggle value: {}", err))?;
		let base = self.get_base_value().map_err(|_| "Error getting base value")?;

		if toggle && base <= 1.0 {
			return Err(format!("Invalid Base {} and toggle enabled", base));
		}

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		for (file,_) in files {
			let hashmap_name = format!("{}_{}", file, self.dataset_name.clone());

			let data = data_repository.get_one_dimensional_data(hashmap_name.clone())
			                          .ok_or(format!("No Data for Dataset: {}", self.dataset_name.clone()))?;

			let data: Vec<f64> = data.into_iter().map(|item|
				return if toggle {
					item.log(base)
				} else {
					item
				}
			).collect();

			update_hashmap.insert(hashmap_name.clone(), data);
		}

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		let toggle = self.get_toggle_value().map_err(|_| ())?;
		let base = self.get_base_value()?;

		if !toggle {
			return Ok(axes_config);
		}
		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{}, scaled with logarithm base {}", config.description, base),
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

impl PerFileLogarithmicTransformer {
	fn get_toggle_value(&self) -> Result<bool, String> {
		Ok(match &self.toggle_element_id {
			None => false,
			Some(id) => {
				let element = get_cast_element::<HtmlInputElement>(id.clone());
				match element {
					None => return Err("No Element with ID".to_string()),
					Some(element) => element.checked()
				}

			}
		})
	}

	fn get_base_value(&self) -> Result<f64,()> {
		Ok(match &self.base_element_id {
			None => 2.0,
			Some(id) => {
				let element = get_cast_element::<HtmlInputElement>(id.clone());
				match element {
					None => return Err(()),
					Some(element) => element.value().as_str().parse().map_err(|_| ())?
				}
			}
		})
	}
}



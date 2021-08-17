use std::collections::HashMap;
use std::sync::Arc;

use crate::declarative_test::plot_config::{AxisConfig, PlotAxesConfig};
use crate::transformers::Transformer;
use crate::data_repository::DataRepository;
use crate::data_repository::transformer::TransformRepository;
use crate::file_list::FILE_LIST;

pub struct PercentageTransformer {
	pub(crate) dataset_name: String,
}

impl Transformer for PercentageTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let selected_dataset = data_repository.get_one_dimensional_data(self.dataset_name.clone())
			.ok_or(format!("Selected Dataset {} not found", self.dataset_name.clone()))?;

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		let transformed_dataset: Vec<f64> = selected_dataset.into_iter()
			.map(|item| item * 100.0).collect();

		update_hashmap.insert(self.dataset_name.clone(), transformed_dataset);

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{} in %", config.description),
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

pub struct PerFilePercentageTransformer {
	pub(crate) dataset_name: String,
}

impl Transformer for PerFilePercentageTransformer {
	fn transform_data(&self, data_repository: Arc<dyn DataRepository + Send + Sync>) -> Result<Arc<dyn DataRepository + Send + Sync>, String> {
		let files = FILE_LIST.lock().unwrap();
		let files = files.iter().filter(|(_,value)| value.is_some());

		let mut update_hashmap = HashMap::<String, Vec<f64>>::new();

		for (name, _) in files {
			let selected_dataset = data_repository.get_one_dimensional_data(format!("{}_{}", name, self.dataset_name.clone()))
			                                      .ok_or(format!("Selected Dataset {} not found", self.dataset_name.clone()))?;


			let transformed_dataset: Vec<f64> = selected_dataset.into_iter()
			                                                    .map(|item| item * 100.0).collect();

			update_hashmap.insert(format!("{}_{}", name, self.dataset_name.clone()), transformed_dataset);
		}

		let repository = TransformRepository {
			base: data_repository,
			one_dimensional_transforms: update_hashmap,
			two_dimensional_transforms: Default::default()
		};

		Ok(Arc::from(repository))
	}

	fn transform_axes_config(&self, axes_config: PlotAxesConfig) -> Result<PlotAxesConfig, ()> {
		let y_config = match axes_config.y_axis {
			None => None,
			Some(config) => {
				Some(AxisConfig {
					description: format!("{} in %", config.description),
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

use std::sync::Arc;
use std::collections::HashMap;
use crate::data_repository::DataRepository;

pub struct TransformRepository<BR: DataRepository + ?Sized> {
	pub(crate) base: Arc<BR>,
	pub(crate) one_dimensional_transforms: HashMap<String, Vec<f64>>,
	pub(crate) two_dimensional_transforms: HashMap<String, Vec<(f64,f64)>>
}

impl<BR: DataRepository + ?Sized> DataRepository for TransformRepository<BR> {
	fn get_one_dimensional_data(&self, name: String) -> Option<Vec<f64>> {
		if self.one_dimensional_transforms.contains_key(&name) {
			return self.one_dimensional_transforms.get(&name)
				.map(|item| item.clone());
		}

		self.base.get_one_dimensional_data(name)
	}

	fn get_two_dimensional_data(&self, name: String) -> Option<Vec<(f64,f64)>> {
		if self.two_dimensional_transforms.contains_key(&name) {
			return self.two_dimensional_transforms.get(&name)
				.map(|item| item.clone());
		}

		self.base.get_two_dimensional_data(name)
	}
}

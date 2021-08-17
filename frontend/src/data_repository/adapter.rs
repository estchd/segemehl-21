use std::sync::{Arc, Mutex};
use crate::data_repository::hash_map::HashMapRepository;
use crate::data_repository::DataRepository;

pub struct AdapterRepository {
	base: Arc<Mutex<Option<HashMapRepository>>>
}

impl AdapterRepository {
	pub fn from(base: Arc<Mutex<Option<HashMapRepository>>>) -> Self {
		AdapterRepository {
			base
		}
	}
}

impl DataRepository for AdapterRepository {
	fn get_one_dimensional_data(&self, name: String) -> Option<Vec<f64>> {
		let base = self.base.lock().ok();

		let base = match base {
			None => return None,
			Some(base) => base
		};

		let repository = match &*base {
			None => return None,
			Some(repository) => repository
		};

		repository.get_one_dimensional_data(name)
	}

	fn get_two_dimensional_data(&self, name: String) -> Option<Vec<(f64, f64)>> {
		let base = self.base.lock().ok();

		let base = match base {
			None => return None,
			Some(base) => base
		};

		let repository = match &*base {
			None => return None,
			Some(repository) => repository
		};

		repository.get_two_dimensional_data(name)
	}
}

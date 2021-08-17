#![allow(dead_code)]

use std::collections::HashMap;
use crate::data_repository::DataRepository;

pub struct HashMapRepository {
	one_dimensional_data: HashMap<String, Vec<f64>>,
	two_dimensional_data: HashMap<String, Vec<(f64,f64)>>
}

impl HashMapRepository {
	pub fn new() -> Self {
		HashMapRepository {
			one_dimensional_data: Default::default(),
			two_dimensional_data: Default::default()
		}
	}

	pub fn add_one_dimensional_data(&mut self, name: String, data: Vec<f64>) {
		self.one_dimensional_data.insert(name, data);
	}

	pub fn add_two_dimensional_data(&mut self, name: String, data: Vec<(f64,f64)>) {
		self.two_dimensional_data.insert(name, data);
	}
}

impl DataRepository for HashMapRepository {
	fn get_one_dimensional_data(&self, name: String) -> Option<Vec<f64>> {
		self.one_dimensional_data.get(&name).map(|item| item.clone())
	}

	fn get_two_dimensional_data(&self, name: String) -> Option<Vec<(f64, f64)>> {
		self.two_dimensional_data.get(&name).map(|item| item.clone())
	}
}

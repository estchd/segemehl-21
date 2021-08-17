pub mod adapter;
pub mod hash_map;
pub mod transformer;

use core::option::Option;

pub trait DataRepository {
	fn get_one_dimensional_data(&self, name: String) -> Option<Vec<f64>>;
	fn get_two_dimensional_data(&self, name: String) -> Option<Vec<(f64,f64)>>;
}

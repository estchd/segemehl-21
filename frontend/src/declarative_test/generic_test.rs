#![allow(dead_code)]

use std::str::FromStr;

trait DataPoint {
	fn get_axis<T: FromStr>(&self, axis: usize) -> Option<T>;
}

trait DataSource {
	type DataPoint: DataPoint;

	fn get_data(&self) -> dyn Iterator<Item=Self::DataPoint>;
}

trait DataRepository {
	type DataSource: DataSource;

	fn get_data_source(&self, name: &str) -> Option<Self::DataSource>;
}

struct TestBaseRepository {

}
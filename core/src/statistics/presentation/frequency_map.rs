use std::collections::HashMap;
use std::hash::Hash;
use serde_derive::{Deserialize, Serialize};

use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use std::collections::hash_map::IntoIter;
use std::iter::Sum;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PresentationFrequencyMap<T>
	where T: Eq + Hash
{
	#[serde(rename = "m")]
	map: HashMap<T, u64>
}

impl<T: Eq + Hash> PresentationFrequencyMap<T> {
	pub fn new() -> Self {
		PresentationFrequencyMap {
			map: Default::default()
		}
	}

	pub fn add_entry(&mut self, entry: T) {
		if self.map.contains_key(&entry) {
			let entry = self.map.get_mut(&entry).unwrap();
			*entry = *entry + 1;
		}
		else {
			self.map.insert(entry, 1);
		}
	}

	pub fn get_frequencies(&self) -> impl Iterator<Item = (&T, u64)> {
		self.map.iter().map(|(t, v)| (t,*v))
	}

	pub fn get_frequency_sum(&self) -> u64 {
		self.map.iter().map(|(_, frequency)| *frequency as u64).sum()
	}

	pub fn get(&self, key: &T) -> Option<u64> {
		self.map.get(key).map(|item| *item)
	}
}

impl<T: Eq + Hash + Clone> PresentationFrequencyMap<T> {
	pub fn merge(lhs: &Self, rhs: &Self) -> Self {
		let mut map = lhs.map.clone();

		for (key, value) in &rhs.map {
			if map.contains_key(&key) {
				let entry = map.get_mut(&key).unwrap();
				*entry = *entry + value;
			}
			else {
				map.insert(key.clone(), *value);
			}
		}

		Self {
			map
		}
	}
}

impl<T: Eq + Hash + Copy + Into<u64>> PresentationFrequencyMap<T> {
	pub fn get_weighted_frequency_sum(&self) -> u64 {
		self.map
			.iter()
			.map(|(t, frequency)| (*t, *frequency))
			.map(|(t, frequency)| t.into() * frequency)
			.sum()
	}
}

impl<T: Eq + Hash + Copy + Ord + Into<u64>> PresentationFrequencyMap<T> {
	pub fn get_median_entry(&self) -> Option<f64> {
		let min = self.get_min_entry();
		let max = self.get_max_entry();

		let min_max = min.zip(max)?;

		let min = min_max.0.0.into();
		let max = min_max.1.0.into();

		let median = (min + max) as f64 / 2.0;
		Some(median)
	}
}

impl<T: Eq + Hash + Copy + Into<u64> + Sum> PresentationFrequencyMap<T> {
	pub fn get_mean_entry(&self) -> f64 {
		let sum: T = self.map.keys()
			.map(|item| *item)
			.sum();

		let sum: u64 = sum.into();
		let count = self.map.len();

		sum as f64 / count as f64
	}
}

impl<T: Eq + Hash> IntoIterator for PresentationFrequencyMap<T> {
	type Item = (T, u64);
	type IntoIter = IntoIter<T, u64>;

	fn into_iter(self) -> Self::IntoIter {
		self.map.into_iter()
	}
}

impl<T: Eq + Hash + Copy + Ord> PresentationFrequencyMap<T> {
	pub fn get_min_frequency(&self) -> Option<(T, u64)> {
		self.map.iter().reduce(|a, b| {
			if a.1 >= b.1 {
				a
			}
			else {
				b
			}
		}).map(|(entry, count)| (*entry, *count))
	}

	pub fn get_max_frequency(&self) -> Option<(T, u64)> {
		self.map.iter().reduce(|a, b| {
			if a.1 <= b.1 {
				a
			}
			else {
				b
			}
		}).map(|(entry, count)| (*entry, *count))
	}

	pub fn get_median_frequency(&self) -> Option<f64> {
		let min = self.get_min_frequency();
		let max = self.get_max_frequency();

		let min_max = min.zip(max);

		min_max.map(|(min, max)| (min.1 + max.1) as f64 / 2.0)
	}

	pub fn get_mean_frequency(&self) -> Option<f64> {
		let count = self.map.len();

		if count == 0 {
			return None;
		}

		let sum: u64 = self.map.values().sum();

		let mean = sum as f64 / count as f64;
		Some(mean)
	}

	pub fn get_min_entry(&self) -> Option<(T, u64)> {
		self.map.iter()
			.reduce(|a, b|
				if *a.0 <= *b.0 {
					a
				}
				else {
					b
				}
			)
			.map(|item| (*item.0, *item.1))
	}

	pub fn get_max_entry(&self) -> Option<(T, u64)> {
	self.map.iter()
		.reduce(|a, b|
			if *a.0 >= *b.0 {
				a
			}
			else {
				b
			}
		)
		.map(|item| (*item.0, *item.1))
	}
}

impl<T: Eq + Hash> From<CalculationFrequencyMap<T>> for PresentationFrequencyMap<T> {
	fn from(map: CalculationFrequencyMap<T>) -> Self {
		let map = map;

		let new_map = HashMap::<T, u64>::new();

		let map = map.into_iter().map(|(key, value)|
			(key, value.into_inner())
		).fold(new_map, |mut map, item| {
			map.insert(item.0, item.1);
			map
		});

		PresentationFrequencyMap {
			map
		}
	}
}

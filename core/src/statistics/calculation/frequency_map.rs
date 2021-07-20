use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::collections::hash_map::IntoIter;

#[derive(Debug)]
pub struct CalculationFrequencyMap<T>
	where T: Eq + Hash
{
	map: RwLock<HashMap<T, AtomicU64>>
}

impl<T: Eq + Hash> CalculationFrequencyMap<T> {
	pub fn new() -> Self {
		CalculationFrequencyMap {
			map: Default::default()
		}
	}

	pub fn add_entry(&self, entry: T) {
		let read = self.map.read().unwrap();

		if read.contains_key(&entry) {
			let entry = read.get(&entry).unwrap();
			entry.fetch_add(1, Ordering::Relaxed);
		}
		else {
			drop(read);
			let mut write = self.map.write().unwrap();
			if write.contains_key(&entry) {
				let entry = write.get(&entry).unwrap();
				entry.fetch_add(1, Ordering::Relaxed);
			}
			else {
				write.insert(entry, AtomicU64::new(1));
			}
		}
	}
}

impl<T: Eq + Hash> IntoIterator for CalculationFrequencyMap<T> {
	type Item = (T, AtomicU64);
	type IntoIter = IntoIter<T, AtomicU64>;

	fn into_iter(self) -> Self::IntoIter {
		self.map.into_inner().unwrap().into_iter()
	}
}

impl<T: Eq + Hash + Copy + Ord> CalculationFrequencyMap<T> {
	pub fn get_min_frequency(&self) -> Option<(T, u64)> {
		let map = self.map.read().unwrap();

		map.iter().reduce(|a, b| {
			let a_count = a.1.load(Ordering::Relaxed);
			let b_count = b.1.load(Ordering::Relaxed);

			if a_count >= b_count {
				a
			}
			else {
				b
			}
		}).map(|(entry, count)| (*entry, count.load(Ordering::Relaxed)))
	}

	pub fn get_max_frequency(&self) -> Option<(T, u64)> {
		let map = self.map.read().unwrap();

		map.iter().reduce(|a, b| {
			let a_count = a.1.load(Ordering::Relaxed);
			let b_count = b.1.load(Ordering::Relaxed);

			if a_count <= b_count {
				a
			}
			else {
				b
			}
		}).map(|(entry, count)| (*entry, count.load(Ordering::Relaxed)))
	}

	pub fn get_median_frequency(&self) -> Option<f64> {
		let min = self.get_min_frequency();
		let max = self.get_max_frequency();

		let min_max = min.zip(max);

		min_max.map(|(min, max)| (min.1 + max.1) as f64 / 2.0)
	}

	pub fn get_mean_frequency(&self) -> Option<f64> {
		let map = self.map.read().unwrap();

		let count = map.len();

		if count == 0 {
			return None;
		}

		let sum: u64 = map.values().map(|atomic| atomic.load(Ordering::Relaxed)).sum();

		let mean = sum as f64 / count as f64;
		Some(mean)
	}
}

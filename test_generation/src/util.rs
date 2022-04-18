use std::ops::RangeInclusive;
use rand::{Rng, thread_rng};

pub fn split_random(value: u32, split_exact: Option<u32>, split_range: Option<RangeInclusive<u32>>) -> (u32, u32) {
	let split_value = match split_exact {
		Some(split) => {
			split
		},
		None => {
			let range = match split_range {
				None => {
					0..=100u32
				}
				Some(range) => {
					range
				}
			};
			thread_rng().gen_range(range)
		}
	};

	let first_split = (value * split_value) / 100;
	let second_split = value - first_split;

	(first_split, second_split)
}

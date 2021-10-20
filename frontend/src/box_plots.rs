#![allow(dead_code)]

use std::hash::Hash;
use std::mem::MaybeUninit;
use segemehl_21_core::statistics::presentation::frequency_map::PresentationFrequencyMap;
use num_traits::AsPrimitive;

#[derive(Clone, Debug)]
pub struct BoxPlot {
	pub entries: Vec<BoxPlotEntry>
}

pub fn box_plot_from_frequency_maps<T: Eq + Hash + Copy + AsPrimitive<f64>>(maps: Vec<PresentationFrequencyMap<T>>) -> BoxPlot {
	let mut entries = Vec::<BoxPlotEntry>::new();

	for map in maps {
		let histogram = histogram_from_frequency_map(map);

		let entry = calculate_boxplot_from_histogram(histogram);

		entries.push(entry);
	}

	return BoxPlot {
		entries
	};
}

pub fn split_box_plot(plot: BoxPlot) -> (
	Vec<f64>,
	Vec<f64>,
	Vec<f64>,
	Vec<f64>,
	Vec<f64>,
	Vec<f64>,
	Vec<f64>
) {
	let mut min: Vec<f64> = Vec::new();
	let mut q1: Vec<f64> = Vec::new();
	let mut median: Vec<f64> = Vec::new();
	let mut mean: Vec<f64> = Vec::new();
	let mut mode: Vec<f64> = Vec::new();
	let mut q3: Vec<f64> = Vec::new();
	let mut max: Vec<f64> = Vec::new();

	for entry in plot.entries {
		min.push(entry.min);
		q1.push(entry.q1);
		median.push(entry.median);
		mean.push(entry.mean);
		mode.push(entry.mode);
		q3.push(entry.q3);
		max.push(entry.max);
	}

	(min, q1, median, mean, mode, q3, max)
}

pub fn boxplot_entry_from_frequency_map<T: Eq + Hash + Copy + AsPrimitive<f64>>(map: PresentationFrequencyMap<T>) -> BoxPlotEntry {
	let histogram = histogram_from_frequency_map(map);
	calculate_boxplot_from_histogram(histogram)
}

#[derive(Clone, Debug)]
pub struct Histogram {
	entries: Vec<HistogramEntry>
}

#[derive(Clone, Debug)]
pub struct HistogramEntry {
	value: f64,
	amount: f64
}

pub fn histogram_from_frequency_map<T: Eq + Hash + Copy + AsPrimitive<f64>>(map: PresentationFrequencyMap<T>) -> Histogram {
	let mut entries = Vec::<HistogramEntry>::new();

	for (value, amount) in map {
		let value: f64 = value.as_();
		let amount = amount as f64;

		entries.push(HistogramEntry {
			value,
			amount
		});
	}

	return Histogram {
		entries
	}
}

#[derive(Clone, Debug)]
pub struct BoxPlotEntry {
	min: f64,
	q1: f64,
	median: f64,
	mean: f64,
	mode: f64,
	q3: f64,
	max: f64
}

pub fn calculate_boxplot_from_histogram(mut histogram: Histogram) -> BoxPlotEntry {
	if histogram.entries.is_empty() {
		return BoxPlotEntry {
			min: 0.0,
			q1: 0.0,
			median: 0.0,
			mean: 0.0,
			mode: 0.0,
			q3: 0.0,
			max: 0.0
		}
	}
	
	histogram.entries.sort_by(|a,b| {
		a.value.partial_cmp(&b.value).unwrap()
	});

	let min = histogram.entries.first()
		.map(|item| item.value)
		.unwrap_or(0.0);

	let max = histogram.entries.last()
		.map(|item| item.value)
		.unwrap_or(0.0);

	let sum_histogram_amounts = histogram.entries.iter()
		.fold(0.0, |prev, curr| {
			prev + curr.amount
		});

	let multiplied_sum = histogram.entries.iter()
		.fold(0.0, |prev, curr| {
			prev + (curr.amount * curr.value)
		});

	let mode = histogram.entries.iter().reduce(
		|prev, curr| {
			if prev.amount > curr.amount {
				prev
			}
			else {
				curr
			}
		}
	).map(|item| item.value)
		.unwrap_or(0.0);

	let mean = multiplied_sum / sum_histogram_amounts;

	let mut quartile_positions = calculate_quartile_positions(sum_histogram_amounts as u64);

	let q1_calculated = calculate_quartile(&mut histogram, 0, None, quartile_positions.q1);

	if let None = q1_calculated {
		return BoxPlotEntry {
			min,
			q1: max,
			median: max,
			mean,
			mode,
			q3: max,
			max
		}
	}

	let q1_calculated = q1_calculated.unwrap();

	quartile_positions.q2 -= q1_calculated.covered_length;
	quartile_positions.q3 -= q1_calculated.covered_length;

	let q2_calculated = calculate_quartile(&mut histogram, q1_calculated.new_index, q1_calculated.new_last, quartile_positions.q2);

	if let None = q2_calculated {
		return BoxPlotEntry {
			min,
			q1: q1_calculated.quartile,
			median: max,
			mean,
			mode,
			q3: max,
			max
		}
	}

	let q2_calculated = q2_calculated.unwrap();

	quartile_positions.q3 -= q2_calculated.covered_length;

	let q3_calculated = calculate_quartile(&mut histogram, q2_calculated.new_index, q2_calculated.new_last, quartile_positions.q3);

	return BoxPlotEntry {
		min,
		q1: q1_calculated.quartile,
		median: q2_calculated.quartile,
		mean,
		mode,
		q3: q3_calculated.map(|item| item.quartile).unwrap_or(max),
		max
	}
}

#[derive(Clone, Debug)]
struct QuartileResult {
	quartile: f64,
	new_index: usize,
	new_last: Option<f64>,
	covered_length: f64
}

fn calculate_quartile(histogram: &mut Histogram, mut index: usize, mut last: Option<f64>, mut quartile_position: f64) -> Option<QuartileResult> {
	let mut quartile: MaybeUninit<f64> = MaybeUninit::uninit();
	let mut covered_length: f64 = 0.0;

	while index < histogram.entries.len() {
		let histogram_value = histogram.entries.get_mut(index)?;

		if quartile_position < 1.0 {
			if let Some(last) = last {
				quartile.write((histogram_value.value + last) / 2.0);
			}
			else{
				quartile.write(histogram_value.value);
			}
			break;
		}

		if histogram_value.amount >= quartile_position {
			histogram_value.amount = histogram_value.amount - quartile_position;

			quartile.write(histogram_value.value);
			last = Some(histogram_value.value);
			covered_length += quartile_position;
			break;
		}

		quartile_position -= histogram_value.amount;
		covered_length += histogram_value.amount;
		last = Some(histogram_value.value);

		index += 1;
	}

	let quartile = unsafe {
		quartile.assume_init()
	};

	return Some(QuartileResult {
		quartile,
		new_index: index,
		new_last: last,
		covered_length
	});
}
#[derive(Clone, Debug)]
struct QuartilePositions {
	q1: f64,
	q2: f64,
	q3: f64
}

fn calculate_quartile_positions(count: u64) -> QuartilePositions {
	let count = count as f64;

	let half_length = (count / 2.0).floor();

	let q1 = (half_length / 2.0) + 0.5;
	let q2 = (count / 2.0) + 0.5;
	let q3 = count - (q1 - 1.0);

	QuartilePositions {
		q1,
		q2,
		q3
	}
}
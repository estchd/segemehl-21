use js_sys::{Uint8Array, Array};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::chromosome_list::{clear_chromosomes, has_chromosomes, set_chromosomes, check_chromosomes};
use segemehl_21_core::statistics::presentation::PresentationData;
use segemehl_21_core::statistics::presentation::cigar_operations::CigarOperations;
use crate::box_plots::{box_plot_from_frequency_maps, BoxPlot, boxplot_entry_from_frequency_map, split_box_plot};

lazy_static! {
	pub static ref FILE_LIST: Mutex<HashMap<String, (Colors, Option<(PresentationData, HashMap<String, Vec<f64>>)>)>> = {
        Mutex::new(HashMap::new())
    };
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Colors {
	main_color: String,
	match_color: String,
	insertion_color: String,
	deletion_color: String,
	skip_color: String
}

impl Colors {
	pub fn from_array(array: Array) -> Self {
		let colors: Vec<String> = array.into_serde().unwrap();
		
		Self {
			main_color: colors[0].clone(),
			match_color: colors[1].clone(),
			insertion_color: colors[2].clone(),
			deletion_color: colors[3].clone(),
			skip_color: colors[4].clone()
		}
	}

	pub fn to_array(self) -> Array {
		let vec = vec![self.main_color, self.match_color, self.insertion_color, self.deletion_color, self.skip_color];
		let js_value = JsValue::from_serde(&vec).unwrap();
		Array::from(&js_value)
	}
}

#[wasm_bindgen]
pub fn setup_file_list() {
	let _ = FILE_LIST.lock().unwrap();
}

#[wasm_bindgen]
pub fn add_file(file: web_sys::File, colors: Array) {
	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.insert(file.name(), (Colors::from_array(colors), None));
}

#[wasm_bindgen]
pub async fn process_file(file: web_sys::File) -> Result<(),JsValue>{
	let promise = file.array_buffer();
	let future = JsFuture::from(promise);

	let result = future.await
		.map_err(|_| JsValue::from_str("Error getting Array Buffer From File"))?;

	let array: Uint8Array = Uint8Array::new(&result);
	let data: Vec<u8> = array.to_vec();

	let data: String = String::from_utf8(data)
		.map_err(|_| JsValue::from_str("Error Converting File Data to String"))?;

	let deserialized_data: PresentationData = serde_json::from_str(&data)
		.map_err(|_| JsValue::from_str("Error deserializing File Content"))?;

	//let deserialized_data: PresentationData = bincode::deserialize(&data)
	//	.map_err(|err| JsValue::from_str(format!("Error deserializing File Content: {}", err).as_str()))?;

	let chromosomes: Vec<String> = deserialized_data
		.get_per_reference_data()
		.map(|item| item.get_reference_name())
		.collect();

	if !has_chromosomes() {
		set_chromosomes(chromosomes);
	}
	else {
		if !check_chromosomes(chromosomes) {
			remove_file(file.name());
			return Err(JsValue::from("Chromosome Names do not Match!"))
		}
	}

	let repository = generate_data_repository(&deserialized_data);

	let mut file_list = FILE_LIST.lock().unwrap();

	let existing = file_list.get(&file.name());
	let color = match existing {
		None => Err(JsValue::from_str("File Removed before Completion")),
		Some((color, _)) => Ok(color.clone())
	}?;

	file_list.insert(file.name(), (color, Some((deserialized_data, repository))));
	drop(file_list);

	Ok(())
}

#[wasm_bindgen]
pub fn update_file_color(name: String, color: String, index: u8) {
	let mut file_list = FILE_LIST.lock().unwrap();
	if file_list.contains_key(&name) {
		let (mut old_colors, old_data) = file_list.remove(&name).unwrap();

		match index {
			0 => {
				old_colors.main_color = color;
			},
			1 => {
				old_colors.match_color = color;
			},
			2 => {
				old_colors.insertion_color = color;
			},
			3 => {
				old_colors.deletion_color = color;
			},
			4 => {
				old_colors.skip_color = color;
			},
			_ => {}
		}

		file_list.insert(name, (old_colors, old_data));
	}
}

#[wasm_bindgen]
pub fn remove_file(name: String) {
	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.remove(&name);

	if file_list.is_empty() {
		clear_chromosomes();
	}

	drop(file_list);
}

#[wasm_bindgen]
pub fn get_file_list() -> Array {
	let file_list = FILE_LIST.lock().unwrap();
	let items = file_list.iter()
		.map(
			|(name,(color, content))|
				(
					JsValue::from_str(name.as_str()),
					JsValue::from(color.clone().to_array()),
					JsValue::from_bool(content.is_some())
				)
		);
	let items = items.map(|item| {
		Array::of3(&item.0, &item.1, &item.2)
	});

	Array::from_iter(items)
}

#[wasm_bindgen]
pub fn get_file_colors(file_name: String) -> Option<Array> {
	let file_list = FILE_LIST.lock().unwrap();

	let (color, _) = file_list.get(&file_name)?;

	Some(color.clone().to_array())
}

#[wasm_bindgen]
pub fn get_dataset(file_name: String, dataset_name: String) -> Option<Array> {
	let file_list = FILE_LIST.lock().unwrap();

	let (_, file_content) = file_list.get(&file_name)?;

	let (_,repository) = file_content.as_ref()?;

	let dataset = repository.get(&dataset_name)?;

	let dataset = dataset.iter().map(|item| {
		JsValue::from(*item)
	});

	Some(dataset.collect())
}


fn generate_data_repository(data: &PresentationData) -> HashMap<String, Vec<f64>> {
	let mut repository = HashMap::<String, Vec<f64>>::new();

	////////////////////////////////////////////////////////////////////////////
	// --------------------------- Reference -------------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let reference_length_data: Vec<f64> = data.get_per_reference_data()
	                                               .map(|item| item.get_reference_length())
	                                               .map(|item| item as f64).collect();

	repository.insert("reference_length".to_string(), reference_length_data.clone());

	let bin_size_data: Vec<f64> = data.get_per_reference_data().map(
		|_| data.get_metadata().bin_size.get() as f64
	).collect();

	repository.insert("bin_size".to_string(), bin_size_data);

	////////////////////////////////////////////////////////////////////////////
	// --------------------------- Split Read ------------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let gap_lengths_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

	let (
		gap_lengths_file_box_min,
		gap_lengths_file_box_q1,
		gap_lengths_file_box_median,
		gap_lengths_file_box_mean,
		gap_lengths_file_box_mode,
		gap_lengths_file_box_q3,
		gap_lengths_file_box_max
	) = split_box_plot(BoxPlot{ entries: vec![gap_lengths_file_box_plot_entry]});

	repository.insert("gap_lengths_file_min".to_string(), gap_lengths_file_box_min);
	repository.insert("gap_lengths_file_q1".to_string(), gap_lengths_file_box_q1);
	repository.insert("gap_lengths_file_median".to_string(), gap_lengths_file_box_median);
	repository.insert("gap_lengths_file_mean".to_string(), gap_lengths_file_box_mean);
	repository.insert("gap_lengths_file_mode".to_string(), gap_lengths_file_box_mode);
	repository.insert("gap_lengths_file_q3".to_string(), gap_lengths_file_box_q3);
	repository.insert("gap_lengths_file_max".to_string(), gap_lengths_file_box_max);

	let gap_lengths_file_mean: f64 = data
		.get_gap_length_map()
		.get_mean_entry().unwrap();
	let gap_lengths_file_mode: f64 = data
		.get_gap_length_map()
		.get_max_frequency().unwrap_or((0,0)).0 as f64;
	let gap_lengths_file_median: f64 = data
		.get_gap_length_map()
		.get_median_entry().unwrap_or(0.0);
	let gap_lengths_file_shortest: f64 = data
		.get_gap_length_map()
		.get_min_entry()
		.unwrap_or((0,0)).0 as f64;
	let gap_lengths_file_longest: f64 = data
		.get_gap_length_map()
		.get_max_entry()
		.unwrap_or((0,0)).0 as f64;

	let complete_lengths_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

	let (
		complete_lengths_file_box_min,
		complete_lengths_file_box_q1,
		complete_lengths_file_box_median,
		complete_lengths_file_box_mean,
		complete_lengths_file_box_mode,
		complete_lengths_file_box_q3,
		complete_lengths_file_box_max
	) = split_box_plot(BoxPlot{ entries: vec![complete_lengths_file_box_plot_entry]});

	repository.insert("complete_lengths_file_min".to_string(), complete_lengths_file_box_min);
	repository.insert("complete_lengths_file_q1".to_string(), complete_lengths_file_box_q1);
	repository.insert("complete_lengths_file_median".to_string(), complete_lengths_file_box_median);
	repository.insert("complete_lengths_file_mean".to_string(), complete_lengths_file_box_mean);
	repository.insert("complete_lengths_file_mode".to_string(), complete_lengths_file_box_mode);
	repository.insert("complete_lengths_file_q3".to_string(), complete_lengths_file_box_q3);
	repository.insert("complete_lengths_file_max".to_string(), complete_lengths_file_box_max);

	let complete_lengths_file_mean: f64 = data
		.get_assembler_length_map()
		.get_mean_entry().unwrap();
	let complete_lengths_file_mode: f64 = data
		.get_assembler_length_map()
		.get_max_frequency().unwrap_or((0,0)).0 as f64;
	let complete_lengths_file_median: f64 = data
		.get_assembler_length_map()
		.get_median_entry().unwrap_or(0.0);
	let complete_lengths_file_shortest: f64 = data
		.get_assembler_length_map()
		.get_min_entry()
		.unwrap_or((0,0)).0 as f64;
	let complete_lengths_file_longest: f64 = data
		.get_assembler_length_map()
		.get_max_entry()
		.unwrap_or((0,0)).0 as f64;

	let split_counts_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

	let (
		split_counts_file_box_min,
		split_counts_file_box_q1,
		split_counts_file_box_median,
		split_counts_file_box_mean,
		split_counts_file_box_mode,
		split_counts_file_box_q3,
		split_counts_file_box_max
	) = split_box_plot(BoxPlot{ entries: vec![split_counts_file_box_plot_entry]});

	repository.insert("split_counts_file_min".to_string(), split_counts_file_box_min);
	repository.insert("split_counts_file_q1".to_string(), split_counts_file_box_q1);
	repository.insert("split_counts_file_median".to_string(), split_counts_file_box_median);
	repository.insert("split_counts_file_mean".to_string(), split_counts_file_box_mean);
	repository.insert("split_counts_file_mode".to_string(), split_counts_file_box_mode);
	repository.insert("split_counts_file_q3".to_string(), split_counts_file_box_q3);
	repository.insert("split_counts_file_max".to_string(), split_counts_file_box_max);

	let split_counts_file_mean: f64 = data
		.get_split_count_map()
		.get_mean_entry().unwrap();
	let split_counts_file_mode: f64 = data
		.get_split_count_map()
		.get_max_frequency().unwrap_or((0,0)).0 as f64;
	let split_counts_file_median: f64 = data
		.get_split_count_map()
		.get_median_entry().unwrap_or(0.0);
	let split_counts_file_shortest: f64 = data
		.get_split_count_map()
		.get_min_entry()
		.unwrap_or((0,0)).0 as f64;
	let split_counts_file_longest: f64 = data
		.get_split_count_map()
		.get_max_entry()
		.unwrap_or((0,0)).0 as f64;

	let gap_lengths_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
			.map(|item| item.get_split_read_data().get_gap_length_map().clone())
			.collect()
	);

	let (
		gap_lengths_per_reference_box_min,
		gap_lengths_per_reference_box_q1,
		gap_lengths_per_reference_box_median,
		gap_lengths_per_reference_box_mean,
		gap_lengths_per_reference_box_mode,
		gap_lengths_per_reference_box_q3,
		gap_lengths_per_reference_box_max
	) = split_box_plot(gap_lengths_per_reference_box_plot);

	repository.insert("gap_lengths_per_reference_min".to_string(), gap_lengths_per_reference_box_min);
	repository.insert("gap_lengths_per_reference_q1".to_string(), gap_lengths_per_reference_box_q1);
	repository.insert("gap_lengths_per_reference_median".to_string(), gap_lengths_per_reference_box_median);
	repository.insert("gap_lengths_per_reference_mean".to_string(), gap_lengths_per_reference_box_mean);
	repository.insert("gap_lengths_per_reference_mode".to_string(), gap_lengths_per_reference_box_mode);
	repository.insert("gap_lengths_per_reference_q3".to_string(), gap_lengths_per_reference_box_q3);
	repository.insert("gap_lengths_per_reference_max".to_string(), gap_lengths_per_reference_box_max);

	let gap_lengths_per_reference_mean: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_gap_length_map())
		.map(|item| item.get_mean_entry().unwrap())
		.map(|item| item as f64)
		.collect();
	let gap_lengths_per_reference_mode: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_gap_length_map())
		.map(|item| item.get_max_frequency().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let gap_lengths_per_reference_median: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_gap_length_map())
		.map(|item| item.get_median_entry().unwrap_or(0.0))
		.map(|item| item as f64)
		.collect();
	let gap_lengths_per_reference_shortest: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_gap_length_map())
		.map(|item| item.get_min_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let gap_lengths_per_reference_longest: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_gap_length_map())
		.map(|item| item.get_max_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();

	let complete_lengths_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
		    .map(|item| item.get_split_read_data().get_assembler_length_map().clone())
		    .collect()
	);

	let (
		complete_lengths_per_reference_box_min,
		complete_lengths_per_reference_box_q1,
		complete_lengths_per_reference_box_median,
		complete_lengths_per_reference_box_mean,
		complete_lengths_per_reference_box_mode,
		complete_lengths_per_reference_box_q3,
		complete_lengths_per_reference_box_max
	) = split_box_plot(complete_lengths_per_reference_box_plot);

	repository.insert("complete_lengths_per_reference_min".to_string(), complete_lengths_per_reference_box_min);
	repository.insert("complete_lengths_per_reference_q1".to_string(), complete_lengths_per_reference_box_q1);
	repository.insert("complete_lengths_per_reference_median".to_string(), complete_lengths_per_reference_box_median);
	repository.insert("complete_lengths_per_reference_mean".to_string(), complete_lengths_per_reference_box_mean);
	repository.insert("complete_lengths_per_reference_mode".to_string(), complete_lengths_per_reference_box_mode);
	repository.insert("complete_lengths_per_reference_q3".to_string(), complete_lengths_per_reference_box_q3);
	repository.insert("complete_lengths_per_reference_max".to_string(), complete_lengths_per_reference_box_max);

	let complete_lengths_per_reference_mean: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_assembler_length_map())
		.map(|item| item.get_mean_entry().unwrap())
		.map(|item| item as f64)
		.collect();
	let complete_lengths_per_reference_mode: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_assembler_length_map())
		.map(|item| item.get_max_frequency().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let complete_lengths_per_reference_median: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_assembler_length_map())
		.map(|item| item.get_median_entry().unwrap_or(0.0))
		.map(|item| item as f64)
		.collect();
	let complete_lengths_per_reference_shortest: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_assembler_length_map())
		.map(|item| item.get_min_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let complete_lengths_per_reference_longest: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_assembler_length_map())
		.map(|item| item.get_max_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();

	let split_counts_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
		    .map(|item| item.get_split_read_data().get_assembler_length_map().clone())
		    .collect()
	);

	let (
		split_counts_per_reference_box_min,
		split_counts_per_reference_box_q1,
		split_counts_per_reference_box_median,
		split_counts_per_reference_box_mean,
		split_counts_per_reference_box_mode,
		split_counts_per_reference_box_q3,
		split_counts_per_reference_box_max
	) = split_box_plot(split_counts_per_reference_box_plot);

	repository.insert("split_counts_per_reference_min".to_string(), split_counts_per_reference_box_min);
	repository.insert("split_counts_per_reference_q1".to_string(), split_counts_per_reference_box_q1);
	repository.insert("split_counts_per_reference_median".to_string(), split_counts_per_reference_box_median);
	repository.insert("split_counts_per_reference_mean".to_string(), split_counts_per_reference_box_mean);
	repository.insert("split_counts_per_reference_mode".to_string(), split_counts_per_reference_box_mode);
	repository.insert("split_counts_per_reference_q3".to_string(), split_counts_per_reference_box_q3);
	repository.insert("split_counts_per_reference_max".to_string(), split_counts_per_reference_box_max);

	let split_counts_per_reference_mean: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_split_count_map())
		.map(|item| item.get_mean_entry().unwrap())
		.map(|item| item as f64)
		.collect();
	let split_counts_per_reference_mode: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_split_count_map())
		.map(|item| item.get_max_frequency().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let split_counts_per_reference_median: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_split_count_map())
		.map(|item| item.get_median_entry().unwrap_or(0.0))
		.map(|item| item as f64)
		.collect();
	let split_counts_per_reference_least: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_split_count_map())
		.map(|item| item.get_min_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();
	let split_counts_per_reference_most: Vec<f64> = data
		.get_per_reference_data()
		.map(|item| item.get_split_read_data().get_split_count_map())
		.map(|item| item.get_max_entry().unwrap_or((0,0)))
		.map(|item| item.0 as f64)
		.collect();

	repository.insert("gap_lengths_file".to_string(), vec![
		gap_lengths_file_mean,
		gap_lengths_file_mode,
		gap_lengths_file_median,
		gap_lengths_file_shortest,
		gap_lengths_file_longest
	]);
	repository.insert("complete_lengths_file".to_string(), vec![
		complete_lengths_file_mean,
		complete_lengths_file_mode,
		complete_lengths_file_median,
		complete_lengths_file_shortest,
		complete_lengths_file_longest
	]);
	repository.insert("split_counts_file".to_string(), vec![
		split_counts_file_mean,
		split_counts_file_mode,
		split_counts_file_median,
		split_counts_file_shortest,
		split_counts_file_longest
	]);
	repository.insert("gap_lengths_per_reference_Mean".to_string(), gap_lengths_per_reference_mean);
	repository.insert("gap_lengths_per_reference_Mode".to_string(), gap_lengths_per_reference_mode);
	repository.insert("gap_lengths_per_reference_Median".to_string(), gap_lengths_per_reference_median);
	repository.insert("gap_lengths_per_reference_Shortest".to_string(), gap_lengths_per_reference_shortest);
	repository.insert("gap_lengths_per_reference_Longest".to_string(), gap_lengths_per_reference_longest);

	repository.insert("complete_lengths_per_reference_Mean".to_string(), complete_lengths_per_reference_mean);
	repository.insert("complete_lengths_per_reference_Mode".to_string(), complete_lengths_per_reference_mode);
	repository.insert("complete_lengths_per_reference_Median".to_string(), complete_lengths_per_reference_median);
	repository.insert("complete_lengths_per_reference_Shortest".to_string(), complete_lengths_per_reference_shortest);
	repository.insert("complete_lengths_per_reference_Longest".to_string(), complete_lengths_per_reference_longest);

	repository.insert("split_counts_per_reference_Mean".to_string(), split_counts_per_reference_mean);
	repository.insert("split_counts_per_reference_Mode".to_string(), split_counts_per_reference_mode);
	repository.insert("split_counts_per_reference_Median".to_string(), split_counts_per_reference_median);
	repository.insert("split_counts_per_reference_Least".to_string(), split_counts_per_reference_least);
	repository.insert("split_counts_per_reference_Most".to_string(), split_counts_per_reference_most);

	////////////////////////////////////////////////////////////////////////////
	// ----------------------------- Unmapped ------------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let unmapped_read_count: u64 = data.get_unmapped_data()
		.get_read_length_map()
		.get_frequency_sum();

	let mapped_read_count: u64 = data.get_per_reference_data()
		.map(|item| item.get_read_length_sequence_map().get_frequency_sum())
		.sum();

	let total_read_count = unmapped_read_count + mapped_read_count;

	let unmapped_read_percentage = (total_read_count as f64 / unmapped_read_count as f64) * 100.0;

	let unmapped_read_length_box_plot_entry = boxplot_entry_from_frequency_map(
		data
			.get_unmapped_data()
			.get_read_length_map()
	);

	let (
		unmapped_read_length_box_min,
		unmapped_read_length_box_q1,
		unmapped_read_length_box_median,
		unmapped_read_length_box_mean,
		unmapped_read_length_box_mode,
		unmapped_read_length_box_q3,
		unmapped_read_length_box_max
	) = split_box_plot(BoxPlot{entries: vec![unmapped_read_length_box_plot_entry]});

	repository.insert("unmapped_read_length_min".to_string(), unmapped_read_length_box_min);
	repository.insert("unmapped_read_length_q1".to_string(), unmapped_read_length_box_q1);
	repository.insert("unmapped_read_length_median".to_string(), unmapped_read_length_box_median);
	repository.insert("unmapped_read_length_mean".to_string(), unmapped_read_length_box_mean);
	repository.insert("unmapped_read_length_mode".to_string(), unmapped_read_length_box_mode);
	repository.insert("unmapped_read_length_q3".to_string(), unmapped_read_length_box_q3);
	repository.insert("unmapped_read_length_max".to_string(), unmapped_read_length_box_max);

	let mean_unmapped_read_length = data
		.get_unmapped_data()
		.get_read_length_map()
		.get_mean_entry().unwrap();
	let mode_unmapped_read_length = data
		.get_unmapped_data()
		.get_read_length_map()
		.get_max_frequency()
		.unwrap_or((0,0)).0 as f64;
	let median_unmapped_read_length = data
		.get_unmapped_data()
		.get_read_length_map()
		.get_median_entry()
		.unwrap_or(0.0);
	let shortest_unmapped_read_length = data
		.get_unmapped_data()
		.get_read_length_map()
		.get_min_entry()
		.unwrap_or((0,0)).0 as f64;
	let longest_unmapped_read_length = data
		.get_unmapped_data()
		.get_read_length_map()
		.get_max_entry()
		.unwrap_or((0,0)).0 as f64;

	repository.insert("unmapped_read_count".to_string(), vec![unmapped_read_count as f64]);
	repository.insert("unmapped_read_percentage".to_string(), vec![unmapped_read_percentage]);
	repository.insert("unmapped_read_length".to_string(), vec![
		mean_unmapped_read_length,
		mode_unmapped_read_length,
		median_unmapped_read_length,
		shortest_unmapped_read_length,
		longest_unmapped_read_length
	]);

	////////////////////////////////////////////////////////////////////////////
	// ---------------------------- Read  Length ---------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let read_length_sequence_file_box_plot_entry = boxplot_entry_from_frequency_map(
		data.get_read_length_sequence_map()
	);

	let (
		read_length_sequence_file_box_min,
		read_length_sequence_file_box_q1,
		read_length_sequence_file_box_median,
		read_length_sequence_file_box_mean,
		read_length_sequence_file_box_mode,
		read_length_sequence_file_box_q3,
		read_length_sequence_file_box_max
	) = split_box_plot(BoxPlot{entries: vec![read_length_sequence_file_box_plot_entry]});

	repository.insert("read_length_sequence_file_min".to_string(), read_length_sequence_file_box_min);
	repository.insert("read_length_sequence_file_q1".to_string(), read_length_sequence_file_box_q1);
	repository.insert("read_length_sequence_file_median".to_string(), read_length_sequence_file_box_median);
	repository.insert("read_length_sequence_file_mean".to_string(), read_length_sequence_file_box_mean);
	repository.insert("read_length_sequence_file_mode".to_string(), read_length_sequence_file_box_mode);
	repository.insert("read_length_sequence_file_q3".to_string(), read_length_sequence_file_box_q3);
	repository.insert("read_length_sequence_file_max".to_string(), read_length_sequence_file_box_max);

	let mean_read_length_sequence_file = data.get_read_length_sequence_map()
	                                          .get_mean_entry().unwrap();
	let mode_read_length_sequence_file = data.get_read_length_sequence_map()
	                                          .get_max_frequency().unwrap_or((0,0)).0 as f64;
	let median_read_length_sequence_file = data.get_read_length_sequence_map()
	                                            .get_median_entry().unwrap_or(0.0);
	let shortest_read_length_sequence_file = data.get_read_length_sequence_map()
	                                              .get_min_entry().unwrap_or((0,0)).0 as f64;
	let longest_read_length_sequence_file = data.get_read_length_sequence_map()
	                                             .get_max_entry().unwrap_or((0,0)).0 as f64;

	let read_length_sequence_file_data = vec![
		mean_read_length_sequence_file,
		mode_read_length_sequence_file,
		median_read_length_sequence_file,
		shortest_read_length_sequence_file,
		longest_read_length_sequence_file
	];

	let read_length_reference_file_box_plot_entry = boxplot_entry_from_frequency_map(
		data.get_read_length_on_reference_map()
	);

	let (
		read_length_reference_file_box_min,
		read_length_reference_file_box_q1,
		read_length_reference_file_box_median,
		read_length_reference_file_box_mean,
		read_length_reference_file_box_mode,
		read_length_reference_file_box_q3,
		read_length_reference_file_box_max
	) = split_box_plot(BoxPlot{entries: vec![read_length_reference_file_box_plot_entry]});

	repository.insert("read_length_reference_file_min".to_string(), read_length_reference_file_box_min);
	repository.insert("read_length_reference_file_q1".to_string(), read_length_reference_file_box_q1);
	repository.insert("read_length_reference_file_median".to_string(), read_length_reference_file_box_median);
	repository.insert("read_length_reference_file_mean".to_string(), read_length_reference_file_box_mean);
	repository.insert("read_length_reference_file_mode".to_string(), read_length_reference_file_box_mode);
	repository.insert("read_length_reference_file_q3".to_string(), read_length_reference_file_box_q3);
	repository.insert("read_length_reference_file_max".to_string(), read_length_reference_file_box_max);

	let mean_read_length_reference_file = data.get_read_length_on_reference_map()
	                                          .get_mean_entry().unwrap();
	let mode_read_length_reference_file = data.get_read_length_on_reference_map()
	                                          .get_max_frequency().unwrap_or((0,0)).0 as f64;
	let median_read_length_reference_file = data.get_read_length_on_reference_map()
	                                            .get_median_entry().unwrap_or(0.0);
	let shortest_read_length_reference_file = data.get_read_length_on_reference_map()
	                                              .get_min_entry().unwrap_or((0,0)).0 as f64;
	let longest_read_length_reference_file = data.get_read_length_on_reference_map()
	                                             .get_max_entry().unwrap_or((0,0)).0 as f64;

	let read_length_reference_file_data = vec![
		mean_read_length_reference_file,
		mode_read_length_reference_file,
		median_read_length_reference_file,
		shortest_read_length_reference_file,
		longest_read_length_reference_file
	];

	let read_length_sequence_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
		    .map(|item| item.get_read_length_sequence_map())
		    .collect()
	);

	let (
		read_length_sequence_per_reference_box_min,
		read_length_sequence_per_reference_box_q1,
		read_length_sequence_per_reference_box_median,
		read_length_sequence_per_reference_box_mean,
		read_length_sequence_per_reference_box_mode,
		read_length_sequence_per_reference_box_q3,
		read_length_sequence_per_reference_box_max
	) = split_box_plot(read_length_sequence_per_reference_box_plot);

	repository.insert("read_length_sequence_per_reference_min".to_string(), read_length_sequence_per_reference_box_min);
	repository.insert("read_length_sequence_per_reference_q1".to_string(), read_length_sequence_per_reference_box_q1);
	repository.insert("read_length_sequence_per_reference_median".to_string(), read_length_sequence_per_reference_box_median);
	repository.insert("read_length_sequence_per_reference_mean".to_string(), read_length_sequence_per_reference_box_mean);
	repository.insert("read_length_sequence_per_reference_mode".to_string(), read_length_sequence_per_reference_box_mode);
	repository.insert("read_length_sequence_per_reference_q3".to_string(), read_length_sequence_per_reference_box_q3);
	repository.insert("read_length_sequence_per_reference_max".to_string(), read_length_sequence_per_reference_box_max);

	let read_length_sequence_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_sequence_map().get_mean_entry().unwrap())
	                                                                  .collect();
	let read_length_sequence_per_reference_mode_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_sequence_map().get_max_frequency())
	                                                                  .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                  .collect();
	let read_length_sequence_per_reference_median_data: Vec<f64> = data.get_per_reference_data()
	                                                                    .map(|item| item.get_read_length_sequence_map().get_median_entry())
	                                                                    .map(|item| item.unwrap_or(0.0))
	                                                                    .collect();
	let read_length_sequence_per_reference_shortest_data: Vec<f64> = data.get_per_reference_data()
	                                                                      .map(|item| item.get_read_length_sequence_map().get_min_entry())
	                                                                      .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                      .collect();
	let read_length_sequence_per_reference_longest_data: Vec<f64> = data.get_per_reference_data()
	                                                                     .map(|item| item.get_read_length_sequence_map().get_max_entry())
	                                                                     .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                     .collect();

	let read_length_reference_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
		    .map(|item| item.get_read_length_on_reference_map())
		    .collect()
	);

	let (
		read_length_reference_per_reference_box_min,
		read_length_reference_per_reference_box_q1,
		read_length_reference_per_reference_box_median,
		read_length_reference_per_reference_box_mean,
		read_length_reference_per_reference_box_mode,
		read_length_reference_per_reference_box_q3,
		read_length_reference_per_reference_box_max
	) = split_box_plot(read_length_reference_per_reference_box_plot);

	repository.insert("read_length_reference_per_reference_min".to_string(), read_length_reference_per_reference_box_min);
	repository.insert("read_length_reference_per_reference_q1".to_string(), read_length_reference_per_reference_box_q1);
	repository.insert("read_length_reference_per_reference_median".to_string(), read_length_reference_per_reference_box_median);
	repository.insert("read_length_reference_per_reference_mean".to_string(), read_length_reference_per_reference_box_mean);
	repository.insert("read_length_reference_per_reference_mode".to_string(), read_length_reference_per_reference_box_mode);
	repository.insert("read_length_reference_per_reference_q3".to_string(), read_length_reference_per_reference_box_q3);
	repository.insert("read_length_reference_per_reference_max".to_string(), read_length_reference_per_reference_box_max);

	let read_length_reference_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_on_reference_map().get_mean_entry().unwrap())
	                                                                  .collect();
	let read_length_reference_per_reference_mode_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_on_reference_map().get_max_frequency())
	                                                                  .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                  .collect();
	let read_length_reference_per_reference_median_data: Vec<f64> = data.get_per_reference_data()
	                                                                    .map(|item| item.get_read_length_on_reference_map().get_median_entry())
	                                                                    .map(|item| item.unwrap_or(0.0))
	                                                                    .collect();
	let read_length_reference_per_reference_shortest_data: Vec<f64> = data.get_per_reference_data()
	                                                                      .map(|item| item.get_read_length_on_reference_map().get_min_entry())
	                                                                      .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                      .collect();
	let read_length_reference_per_reference_longest_data: Vec<f64> = data.get_per_reference_data()
	                                                                     .map(|item| item.get_read_length_on_reference_map().get_max_entry())
	                                                                     .map(|item| item.unwrap_or((0,0)).0 as f64)
	                                                                     .collect();

	repository.insert("read_length_sequence_file".to_string(), read_length_sequence_file_data);
	repository.insert("read_length_reference_file".to_string(), read_length_reference_file_data);

	repository.insert("read_length_sequence_per_reference_".to_string(), vec![]);

	repository.insert("read_length_sequence_per_reference_Mean".to_string(), read_length_sequence_per_reference_mean_data);
	repository.insert("read_length_sequence_per_reference_Mode".to_string(), read_length_sequence_per_reference_mode_data);
	repository.insert("read_length_sequence_per_reference_Median".to_string(), read_length_sequence_per_reference_median_data);
	repository.insert("read_length_sequence_per_reference_Shortest".to_string(), read_length_sequence_per_reference_shortest_data);
	repository.insert("read_length_sequence_per_reference_Longest".to_string(), read_length_sequence_per_reference_longest_data);


	repository.insert("read_length_reference_per_reference_".to_string(), vec![]);

	repository.insert("read_length_reference_per_reference_Mean".to_string(), read_length_reference_per_reference_mean_data);
	repository.insert("read_length_reference_per_reference_Mode".to_string(), read_length_reference_per_reference_mode_data);
	repository.insert("read_length_reference_per_reference_Median".to_string(), read_length_reference_per_reference_median_data);
	repository.insert("read_length_reference_per_reference_Shortest".to_string(), read_length_reference_per_reference_shortest_data);
	repository.insert("read_length_reference_per_reference_Longest".to_string(), read_length_reference_per_reference_longest_data);

	////////////////////////////////////////////////////////////////////////////
	// ------------------------------ Coverage ------------------------------ //
	////////////////////////////////////////////////////////////////////////////

	let reads_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                              .map(|item| item.get_read_length_on_reference_map())
	                                              .map(|item| item.get_frequency_sum())
	                                              .map(|item| item as f64).collect();

	let total_read_length_per_reference_data: Vec<f64> = data.get_per_reference_data()
	                                                         .map(|item| item.get_read_length_on_reference_map())
	                                                         .map(|item| item.get_weighted_frequency_sum())
	                                                         .map(|item| item as f64).collect();

	let coverage_per_reference_data: Vec<f64> = total_read_length_per_reference_data
		.iter()
		.zip(reference_length_data.iter())
		.map(|(total_read_length, reference_length)| *total_read_length / *reference_length)
		.map(|item| item * 100.0)
		.collect();

	repository.insert("read_counts_per_reference".to_string(), reads_per_chromosome_data.clone());
	repository.insert("total_read_length_per_reference".to_string(), total_read_length_per_reference_data.clone());
	repository.insert("coverage_per_reference".to_string(), coverage_per_reference_data.clone());

	for reference in data.get_per_reference_data() {
		let read_count_name = format!("{}_read_counts_per_bin", reference.get_reference_name());
		let total_read_length_name = format!("{}_total_read_length_per_bin", reference.get_reference_name());
		let coverage_name = format!("{}_coverage_per_bin", reference.get_reference_name());

		let read_count_data: Vec<f64> = reference
		                                    .get_binned_statistics()
		                                    .get_bins()
		                                    .map(|item| item.get_read_count())
		                                    .map(|item| item as f64)
		                                    .collect();

		let total_read_length_data: Vec<f64> = reference
		                                           .get_binned_statistics()
		                                           .get_bins()
		                                           .map(|item| item.get_total_read_length())
		                                           .map(|item| item as f64)
		                                           .collect();

		let coverage_data: Vec<f64> = reference
		                                  .get_binned_statistics()
		                                  .get_bins()
		                                  .map(|item| item.get_coverage() * 100.0)
		                                  .collect();


		repository.insert(read_count_name, read_count_data);
		repository.insert(total_read_length_name, total_read_length_data);
		repository.insert(coverage_name, coverage_data);
	}

	////////////////////////////////////////////////////////////////////////////
	// -------------------------- Cigar  Operation -------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let file_cigar_operations = data.get_cigar_operations();

	let file_total_cigar_operations =
		file_cigar_operations.alignment_matches +
			file_cigar_operations.insertions +
			file_cigar_operations.deletions +
			file_cigar_operations.skips;

	let cigar_total_file_data = vec![
		file_cigar_operations.alignment_matches as f64,
		file_cigar_operations.insertions as f64,
		file_cigar_operations.deletions as f64,
		file_cigar_operations.skips as f64
	];

	let cigar_percentage_file_data = vec![
		(file_cigar_operations.alignment_matches as f64 / file_total_cigar_operations as f64) * 100.0,
		(file_cigar_operations.insertions as f64 / file_total_cigar_operations as f64) * 100.0,
		(file_cigar_operations.deletions as f64 / file_total_cigar_operations as f64) * 100.0,
		(file_cigar_operations.skips as f64 / file_total_cigar_operations as f64) * 100.0
	];

	repository.insert("cigar_total_file".to_string(), cigar_total_file_data);
	repository.insert("cigar_percentage_file".to_string(), cigar_percentage_file_data);

	let per_reference_cigar_operations: Vec<CigarOperations> = data.get_per_reference_data()
		.map(|item| item.get_cigar_operations())
		.collect();

	let per_reference_total_operations: Vec<u64> = per_reference_cigar_operations.iter()
		.map(|item|
			item.alignment_matches +
				item.deletions +
				item.insertions +
				item.skips
		).collect();

	let per_reference_total_match_data: Vec<f64> = per_reference_cigar_operations.iter()
		.map(|item| item.alignment_matches as f64).collect();

	let per_reference_total_insertion_data: Vec<f64> = per_reference_cigar_operations.iter()
		.map(|item| item.insertions as f64).collect();

	let per_reference_total_deletion_data: Vec<f64> = per_reference_cigar_operations.iter()
		.map(|item| item.deletions as f64).collect();

	let per_reference_total_skip_data: Vec<f64> = per_reference_cigar_operations.iter()
		.map(|item| item.skips as f64).collect();

	let per_reference_percentage_match_data: Vec<f64> = per_reference_total_match_data.iter()
		.zip(per_reference_total_operations.iter())
		.map(|(matches,total)| (*matches / *total as f64) * 100.0)
		.collect();

	let per_reference_percentage_insertion_data: Vec<f64> = per_reference_total_insertion_data.iter()
		.zip(per_reference_total_operations.iter())
		.map(|(insertions,total)| (*insertions / *total as f64) * 100.0)
		.collect();

	let per_reference_percentage_deletion_data: Vec<f64> = per_reference_total_deletion_data.iter()
		.zip(per_reference_total_operations.iter())
		.map(|(deletions,total)| (*deletions / *total as f64) * 100.0)
		.collect();

	let per_reference_percentage_skip_data: Vec<f64> = per_reference_total_skip_data.iter()
		.zip(per_reference_total_operations.iter())
		.map(|(skips,total)| (*skips / *total as f64) * 100.0)
		.collect();

	repository.insert("cigar_total_per_reference_match".to_string(), per_reference_total_match_data);
	repository.insert("cigar_total_per_reference_insertion".to_string(), per_reference_total_insertion_data);
	repository.insert("cigar_total_per_reference_deletion".to_string(), per_reference_total_deletion_data);
	repository.insert("cigar_total_per_reference_skip".to_string(), per_reference_total_skip_data);
	repository.insert("cigar_percentage_per_reference_match".to_string(), per_reference_percentage_match_data);
	repository.insert("cigar_percentage_per_reference_insertion".to_string(), per_reference_percentage_insertion_data);
	repository.insert("cigar_percentage_per_reference_deletion".to_string(), per_reference_percentage_deletion_data);
	repository.insert("cigar_percentage_per_reference_skip".to_string(), per_reference_percentage_skip_data);

	for reference in data.get_per_reference_data() {
		let name = reference.get_reference_name();

		let per_bin_cigar_operations: Vec<CigarOperations> = reference.get_binned_statistics().get_bins()
		                                                               .map(|item| item.get_cigar_operations())
		                                                               .collect();

		let per_bin_total_operations: Vec<u64> = per_bin_cigar_operations.iter()
		                                                                             .map(|item|
			                                                                             item.alignment_matches +
				                                                                             item.deletions +
				                                                                             item.insertions +
				                                                                             item.skips
		                                                                             ).collect();

		let per_bin_total_match_data: Vec<f64> = per_bin_cigar_operations.iter()
		                                                                             .map(|item| item.alignment_matches as f64).collect();

		let per_bin_total_insertion_data: Vec<f64> = per_bin_cigar_operations.iter()
		                                                                                 .map(|item| item.insertions as f64).collect();

		let per_bin_total_deletion_data: Vec<f64> = per_bin_cigar_operations.iter()
		                                                                                .map(|item| item.deletions as f64).collect();

		let per_bin_total_skip_data: Vec<f64> = per_bin_cigar_operations.iter()
		                                                                            .map(|item| item.skips as f64).collect();

		let per_bin_percentage_match_data: Vec<f64> = per_bin_total_match_data.iter()
		                                                                                  .zip(per_bin_total_operations.iter())
		                                                                                  .map(|(matches,total)| (*matches / *total as f64) * 100.0)
		                                                                                  .collect();

		let per_bin_percentage_insertion_data: Vec<f64> = per_bin_total_insertion_data.iter()
		                                                                                          .zip(per_bin_total_operations.iter())
		                                                                                          .map(|(insertions,total)| (*insertions / *total as f64) * 100.0)
		                                                                                          .collect();

		let per_bin_percentage_deletion_data: Vec<f64> = per_bin_total_deletion_data.iter()
		                                                                                        .zip(per_bin_total_operations.iter())
		                                                                                        .map(|(deletions,total)| (*deletions / *total as f64) * 100.0)
		                                                                                        .collect();

		let per_bin_percentage_skip_data: Vec<f64> = per_bin_total_skip_data.iter()
		                                                                                .zip(per_bin_total_operations.iter())
		                                                                                .map(|(skips,total)| (*skips / *total as f64) * 100.0)
		                                                                                .collect();

		repository.insert(format!("{}_cigar_total_per_bin_match",&name), per_bin_total_match_data);
		repository.insert(format!("{}_cigar_total_per_bin_insertion",&name), per_bin_total_insertion_data);
		repository.insert(format!("{}_cigar_total_per_bin_deletion",&name), per_bin_total_deletion_data);
		repository.insert(format!("{}_cigar_total_per_bin_skip",&name), per_bin_total_skip_data);
		repository.insert(format!("{}_cigar_percentage_per_bin_match",&name), per_bin_percentage_match_data);
		repository.insert(format!("{}_cigar_percentage_per_bin_insertion",&name), per_bin_percentage_insertion_data);
		repository.insert(format!("{}_cigar_percentage_per_bin_deletion",&name), per_bin_percentage_deletion_data);
		repository.insert(format!("{}_cigar_percentage_per_bin_skip",&name), per_bin_percentage_skip_data);
	}

	////////////////////////////////////////////////////////////////////////////
	// ------------------------------ Quality ------------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let file_quality_map: Vec<f64> = data.get_complete_quality_frequency_map()
		.into_iter()
		.map(|(_,b)| b)
		.map(|item| item as f64).collect();

	repository.insert("read_quality_file".to_string(), file_quality_map);

	let read_quality_per_reference_box_plot = box_plot_from_frequency_maps(
		data.get_per_reference_data()
		    .map(|item| item.get_read_length_on_reference_map())
		    .collect()
	);

	let (
		read_quality_per_reference_box_min,
		read_quality_per_reference_box_q1,
		read_quality_per_reference_box_median,
		read_quality_per_reference_box_mean,
		read_quality_per_reference_box_mode,
		read_quality_per_reference_box_q3,
		read_quality_per_reference_box_max
	) = split_box_plot(read_quality_per_reference_box_plot);

	repository.insert("read_quality_per_reference_min".to_string(), read_quality_per_reference_box_min);
	repository.insert("read_quality_per_reference_q1".to_string(), read_quality_per_reference_box_q1);
	repository.insert("read_quality_per_reference_median".to_string(), read_quality_per_reference_box_median);
	repository.insert("read_quality_per_reference_mean".to_string(), read_quality_per_reference_box_mean);
	repository.insert("read_quality_per_reference_mode".to_string(), read_quality_per_reference_box_mode);
	repository.insert("read_quality_per_reference_q3".to_string(), read_quality_per_reference_box_q3);
	repository.insert("read_quality_per_reference_max".to_string(), read_quality_per_reference_box_max);

	let per_reference_read_quality_mean: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_mean_entry().unwrap())
		.collect();
	let per_reference_read_quality_mode: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_max_frequency())
		.map(|item| item.map(|(quality,_)| quality ))
		.map(|item| item.unwrap_or(0) as f64)
		.collect();
	let per_reference_read_quality_median: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_median_entry())
		.map(|item| item.unwrap_or(0.0))
		.collect();
	let per_reference_read_quality_min: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_min_entry())
		.map(|item| item.map(|(quality,_)| quality))
		.map(|item| item.unwrap_or(0) as f64)
		.collect();
	let per_reference_read_quality_max: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_max_entry())
		.map(|item| item.map(|(quality,_)| quality))
		.map(|item| item.unwrap_or(0) as f64)
		.collect();

	repository.insert("read_quality_per_reference_".to_string(), vec![]);

	repository.insert("read_quality_per_reference_Mean".to_string(), per_reference_read_quality_mean);
	repository.insert("read_quality_per_reference_Mode".to_string(), per_reference_read_quality_mode);
	repository.insert("read_quality_per_reference_Median".to_string(), per_reference_read_quality_median);
	repository.insert("read_quality_per_reference_Min".to_string(), per_reference_read_quality_min);
	repository.insert("read_quality_per_reference_Max".to_string(), per_reference_read_quality_max);

	for reference in data.get_per_reference_data() {
		let reference_quality_data: Vec<f64> = reference.get_quality_frequency_map().into_iter()
			.map(|(_,frequency)| frequency as f64)
			.collect();

		let reference_quality_name = format!("{}_read_quality", reference.get_reference_name());

		repository.insert(reference_quality_name, reference_quality_data);
	}

	repository
}
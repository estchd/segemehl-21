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

lazy_static! {
	pub static ref FILE_LIST: Mutex<HashMap<String, (String, Option<(PresentationData, HashMap<String, Vec<f64>>)>)>> = {
        Mutex::new(HashMap::new())
    };
}

#[wasm_bindgen]
pub fn setup_file_list() {
	let _ = FILE_LIST.lock().unwrap();
}

#[wasm_bindgen]
pub fn add_file(file: web_sys::File, color: String) {
	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.insert(file.name(), (color, None));
}

#[wasm_bindgen]
pub async fn process_file(file: web_sys::File) -> Result<(),JsValue>{
	let promise = file.array_buffer();
	let future = JsFuture::from(promise);

	let result = future.await
		.map_err(|_| JsValue::from_str("Error getting Array Buffer From File"))?;

	let array: Uint8Array = Uint8Array::new(&result);
	let data: Vec<u8> = array.to_vec();

	let deserialized_data: PresentationData = bincode::deserialize(&data)
		.map_err(|_| JsValue::from_str("Error deserializing File Content"))?;

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
pub fn update_file_color(name: String, color: String) {
	let mut file_list = FILE_LIST.lock().unwrap();
	if file_list.contains_key(&name) {
		let old_value = file_list.remove(&name).unwrap();
		file_list.insert(name, (color, old_value.1));
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
	let items = file_list.iter().map(|(name,(color, content))| (JsValue::from_str(name.as_str()), JsValue::from_str(color.as_str()), JsValue::from_bool(content.is_some())));
	let items = items.map(|item| {
		Array::of3(&item.0, &item.1, &item.2)
	});

	Array::from_iter(items)
}

#[wasm_bindgen]
pub fn get_file_color(file_name: String) -> Option<String> {
	let file_list = FILE_LIST.lock().unwrap();

	let (color, _) = file_list.get(&file_name)?;

	Some(color.clone())
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

	let length_of_chromosomes_data: Vec<f64> = data.get_per_reference_data()
	                                               .map(|item| item.get_reference_length())
	                                               .map(|item| item as f64).collect();

	repository.insert("test_dataset".to_string(), length_of_chromosomes_data.clone());
	repository.insert("length_of_chromosomes".to_string(), length_of_chromosomes_data.clone());

	let covered_length_of_chromosomes_data: Vec<f64> = data.get_per_reference_data()
	                                                       .map(|item| item.get_covered_length())
	                                                       .map(|item| item as f64).collect();

	repository.insert("covered_length_of_chromosomes".to_string(), covered_length_of_chromosomes_data.clone());

	let coverage_per_chromosome_data: Vec<f64> = length_of_chromosomes_data.iter().zip(covered_length_of_chromosomes_data.iter())
	                                                                       .map(|(length, covered_length)| *covered_length / *length).collect();

	repository.insert("coverage_per_chromosome".to_string(), coverage_per_chromosome_data.clone());

	let quality_frequency_map: Vec<f64> = data.get_complete_quality_frequency_map()
	                                          .into_iter()
	                                          .map(|(_,b)| b)
	                                          .map(|item| item as f64).collect();

	repository.insert("file_quality_frequency_map".to_string(), quality_frequency_map.clone());

	let empty_data: Vec<f64> = vec![];

	repository.insert("_Total_coverage_per_bin".to_string(), empty_data.clone());
	repository.insert("_Average_coverage_per_bin".to_string(), empty_data.clone());
	repository.insert("_quality_frequency_map".to_string(), empty_data.clone());

	for (i, per_chromosome_data) in data.get_per_reference_data().enumerate() {
		let total_coverage_name = format!("{}_Total_coverage_per_bin", per_chromosome_data.get_reference_name().clone());
		let average_coverage_name = format!("{}_Average_coverage_per_bin", per_chromosome_data.get_reference_name().clone());
		let quality_name = format!("{}_quality_frequency_map", per_chromosome_data.get_reference_name().clone());

		let quality_data: Vec<f64> = data.get_per_reference_by_index(i).unwrap()
		                                 .get_single_read_data()
		                                 .get_quality_frequency_map()
		                                 .iter()
		                                 .map(|(_, item)| *item as f64).collect();
		let total_coverage_data: Vec<f64> = data.get_per_reference_by_index(i).unwrap()
		                                        .get_single_read_data()
		                                        .get_binned_statistics()
		                                        .get_bins()
		                                        .map(|item| item.get_coverage())
		                                        .map(|item| item as f64)
		                                        .collect();
		let average_coverage_data: Vec<f64> = data.get_per_reference_by_index(i).unwrap()
		                                          .get_single_read_data()
		                                          .get_binned_statistics()
		                                          .get_bins()
		                                          .map(|item| item.get_average_coverage())
		                                          .collect();

		repository.insert(quality_name, quality_data);
		repository.insert(total_coverage_name, total_coverage_data);
		repository.insert(average_coverage_name, average_coverage_data);
	}

	let reads_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                              .map(|item| item.get_single_read_data())
	                                              .map(|item| item.get_read_length_map())
	                                              .map(|item| item.get_frequency_sum())
	                                              .map(|item| item as f64).collect();

	repository.insert("reads_per_chromosome".to_string(), reads_per_chromosome_data.clone());

	let shortest_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                                .map(|item| item.get_single_read_data())
	                                                                .map(|item| item.get_read_length_map())
	                                                                .map(|item| item.get_min_entry())
	                                                                .map(|item| item.unwrap_or((0,0)))
	                                                                .map(|(item,_)| item as f64).collect();
	let longest_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                               .map(|item| item.get_single_read_data())
	                                                               .map(|item| item.get_read_length_map())
	                                                               .map(|item| item.get_max_entry())
	                                                               .map(|item| item.unwrap_or((0,0)))
	                                                               .map(|(item,_)| item as f64).collect();
	let mean_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                            .map(|item| item.get_single_read_data())
	                                                            .map(|item| item.get_read_length_map())
	                                                            .map(|item| item.get_mean_entry()).collect();
	let median_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                              .map(|item| item.get_single_read_data())
	                                                              .map(|item| item.get_read_length_map())
	                                                              .map(|item| item.get_median_entry())
	                                                              .map(|item| item.unwrap_or(0.0)).collect();
	let mode_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                            .map(|item| item.get_single_read_data())
	                                                            .map(|item| item.get_read_length_map())
	                                                            .map(|item| item.get_max_frequency())
	                                                            .map(|item| item.unwrap_or((0,0)))
	                                                            .map(|(item,_)| item)
	                                                            .map(|item| item as f64).collect();

	repository.insert("Shortest_length_of_read_per_chromosome".to_string(), shortest_length_of_read_per_chromosome_data.clone());
	repository.insert("Longest_length_of_read_per_chromosome".to_string(), longest_length_of_read_per_chromosome_data.clone());
	repository.insert("Mean_length_of_read_per_chromosome".to_string(), mean_length_of_read_per_chromosome_data.clone());
	repository.insert("Median_length_of_read_per_chromosome".to_string(), median_length_of_read_per_chromosome_data.clone());
	repository.insert("Mode_length_of_read_per_chromosome".to_string(), mode_length_of_read_per_chromosome_data.clone());

	repository
}
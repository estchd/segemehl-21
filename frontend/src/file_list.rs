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
use segemehl_21_core::statistics::presentation::frequency_map::PresentationFrequencyMap;
use segemehl_21_core::statistics::presentation::cigar_operations::CigarOperations;

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

	let length_of_chromosomes_data: Vec<f64> = data.get_per_reference_data()
	                                               .map(|item| item.get_reference_length())
	                                               .map(|item| item as f64).collect();

	repository.insert("test_dataset".to_string(), length_of_chromosomes_data.clone());
	repository.insert("length_of_chromosomes".to_string(), length_of_chromosomes_data.clone());

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
		                                        .map(|item| item.get_read_count())
		                                        .map(|item| item as f64)
		                                        .collect();
		let average_coverage_data: Vec<f64> = data.get_per_reference_by_index(i).unwrap()
		                                          .get_single_read_data()
		                                          .get_binned_statistics()
		                                          .get_bins()
		                                          .map(|item| item.get_coverage())
		                                          .collect();

		repository.insert(quality_name, quality_data);
		repository.insert(total_coverage_name, total_coverage_data);
		repository.insert(average_coverage_name, average_coverage_data);

	}

	let shortest_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                                .map(|item| item.get_single_read_data())
	                                                                .map(|item| item.get_read_length_on_reference_map())
	                                                                .map(|item| item.get_min_entry())
	                                                                .map(|item| item.unwrap_or((0,0)))
	                                                                .map(|(item,_)| item as f64).collect();
	let longest_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                               .map(|item| item.get_single_read_data())
	                                                               .map(|item| item.get_read_length_on_reference_map())
	                                                               .map(|item| item.get_max_entry())
	                                                               .map(|item| item.unwrap_or((0,0)))
	                                                               .map(|(item,_)| item as f64).collect();
	let mean_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                            .map(|item| item.get_single_read_data())
	                                                            .map(|item| item.get_read_length_on_reference_map())
	                                                            .map(|item| item.get_mean_entry()).collect();
	let median_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                              .map(|item| item.get_single_read_data())
	                                                              .map(|item| item.get_read_length_on_reference_map())
	                                                              .map(|item| item.get_median_entry())
	                                                              .map(|item| item.unwrap_or(0.0)).collect();
	let mode_length_of_read_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
	                                                            .map(|item| item.get_single_read_data())
	                                                            .map(|item| item.get_read_length_on_reference_map())
	                                                            .map(|item| item.get_max_frequency())
	                                                            .map(|item| item.unwrap_or((0,0)))
	                                                            .map(|(item,_)| item)
	                                                            .map(|item| item as f64).collect();

	repository.insert("Shortest_length_of_read_per_chromosome".to_string(), shortest_length_of_read_per_chromosome_data.clone());
	repository.insert("Longest_length_of_read_per_chromosome".to_string(), longest_length_of_read_per_chromosome_data.clone());
	repository.insert("Mean_length_of_read_per_chromosome".to_string(), mean_length_of_read_per_chromosome_data.clone());
	repository.insert("Median_length_of_read_per_chromosome".to_string(), median_length_of_read_per_chromosome_data.clone());
	repository.insert("Mode_length_of_read_per_chromosome".to_string(), mode_length_of_read_per_chromosome_data.clone());

	let file_gap_length_map = data.get_per_reference_data()
		.map(|item| item.get_split_read_data())
		.map(|item| item.get_gap_length_map())
		.fold(PresentationFrequencyMap::new(), |acc, item| {
			PresentationFrequencyMap::merge(&acc, item)
		});

	let mut file_gap_length_map: Vec<(i64, u64)> = file_gap_length_map.get_frequencies().map(|(a,b)| (*a,b)).collect();

	file_gap_length_map.sort_by(|(a,_),(b,_)| {
		a.cmp(b)
	});

	let gap_lengths: Vec<f64> = file_gap_length_map.iter().map(|(a,_)| *a as f64).collect();
	let gap_length_frequencies: Vec<f64> = file_gap_length_map.iter().map(|(_,b)| *b as f64).collect();

	repository.insert("file_gap_lengths".to_string(), gap_lengths);
	repository.insert("file_gap_length_frequencies".to_string(), gap_length_frequencies);

	let file_split_counts_map = data.get_per_reference_data()
		.map(|item| item.get_split_read_data())
		.map(|item| item.get_split_count_map())
		.fold(PresentationFrequencyMap::new(), |acc, item| {
			PresentationFrequencyMap::merge(&acc, item)
		});

	let mut file_split_counts_map: Vec<(usize, u64)> = file_split_counts_map.get_frequencies().map(|(a,b)| (*a,b)).collect();

	file_split_counts_map.sort_by(|(a,_),(b,_)| {
		a.cmp(b)
	});

	let split_counts: Vec<f64> = file_split_counts_map.iter().map(|(a,_)| *a as f64).collect();
	let split_count_frequencies: Vec<f64> = file_split_counts_map.iter().map(|(_,b)| *b as f64).collect();

	repository.insert("file_split_counts".to_string(), split_counts);
	repository.insert("file_split_count_frequencies".to_string(), split_count_frequencies);



	////////////////////////////////////////////////////////////////////////////
	// -------------------------- New Calculations -------------------------- //
	////////////////////////////////////////////////////////////////////////////



	////////////////////////////////////////////////////////////////////////////
	// ---------------------------- Read  Length ---------------------------- //
	////////////////////////////////////////////////////////////////////////////

	let mean_read_length_sequence_file = data.get_read_length_sequence_map()
	                                          .get_mean_entry();
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

	let mean_read_length_reference_file = data.get_read_length_on_reference_map()
	                                          .get_mean_entry();
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

	let read_length_sequence_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_sequence_map().get_mean_entry())
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

	let read_length_reference_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
	                                                                  .map(|item| item.get_read_length_on_reference_map().get_mean_entry())
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
		.zip(length_of_chromosomes_data.iter())
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

	let per_reference_read_quality_mean: Vec<f64> = data.get_per_reference_data()
		.map(|item| item.get_quality_frequency().get_mean_entry())
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



	////////////////////////////////////////////////////////////////////////////
	// --------------------- Old Inserts with new data ---------------------- //
	////////////////////////////////////////////////////////////////////////////



	repository.insert("reads_per_chromosome".to_string(), reads_per_chromosome_data.clone());


	repository
}
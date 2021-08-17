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
use crate::{regenerate_data_repository, update_plot_class};

lazy_static! {
	pub static ref FILE_LIST: Mutex<HashMap<String, Option<PresentationData>>> = {
        Mutex::new(HashMap::new())
    };
}

#[wasm_bindgen]
pub fn setup_file_list() {
	let _ = FILE_LIST.lock().unwrap();
}

#[wasm_bindgen]
pub fn add_file(file: web_sys::File) {
	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.insert(file.name(), None);
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

	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.insert(file.name(), Some(deserialized_data));
	drop(file_list);

	regenerate_data_repository();
	update_plot_class("all".to_string());

	Ok(())
}

#[wasm_bindgen]
pub fn remove_file(name: String) {
	let mut file_list = FILE_LIST.lock().unwrap();
	file_list.remove(&name);

	if file_list.is_empty() {
		clear_chromosomes();
	}

	drop(file_list);

	regenerate_data_repository();
	update_plot_class("all".to_string());
}

#[wasm_bindgen]
pub fn get_file_list() -> Array {
	let file_list = FILE_LIST.lock().unwrap();
	let items = file_list.iter().map(|(name,content)| (JsValue::from_str(name.as_str()), JsValue::from_bool(content.is_some())));
	let items = items.map(|item| {
		Array::of2(&item.0, &item.1)
	});

	Array::from_iter(items)
}

use js_sys::{Array};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static! {
    static ref CHROMOSOME_LIST: Mutex<Vec<String>> = {
        Mutex::new(Vec::new())
    };
}

pub fn check_chromosomes(chromosomes: Vec<String>) -> bool {
	let current_chromosomes = CHROMOSOME_LIST.lock().unwrap();
	let hash_a: HashSet<String> = HashSet::from_iter(chromosomes.clone().into_iter());
	let hash_b: HashSet<String> = HashSet::from_iter(current_chromosomes.clone().into_iter());

	hash_a == hash_b
}

pub fn set_chromosomes(chromosomes: Vec<String>) {
	let mut lock = CHROMOSOME_LIST.lock().unwrap();
	*lock = chromosomes;
}

pub fn has_chromosomes() -> bool {
	let lock = CHROMOSOME_LIST.lock().unwrap();
	!lock.is_empty()
}

pub fn clear_chromosomes() {
	let mut lock = CHROMOSOME_LIST.lock().unwrap();
	lock.clear();
}


#[wasm_bindgen]
pub fn get_chromosome_names() -> Array {
	let chromosomes = CHROMOSOME_LIST.lock().unwrap();

	let chromosomes = chromosomes.iter().map(|item| JsValue::from_str(item));
	Array::from_iter(chromosomes)
}
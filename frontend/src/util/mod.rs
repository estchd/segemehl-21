#![allow(dead_code)]

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlInputElement, Window};

#[macro_use]
pub mod console;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_cast_element<T: JsCast>(id: String) -> Option<T> {
    get_element(id)
        .map(
            |element|
            element.dyn_into::<T>().ok()
        )
        .flatten()
}

fn get_element(id: String) -> Option<Element> {
    get_document()
        .map(
            |document|
            document.get_element_by_id(id.as_str())
        )
        .flatten()
}

fn get_document() -> Option<Document> {
    get_window()
        .map(
            |window|
            window.document()
        )
        .flatten()
}

fn get_window() -> Option<Window> {
    web_sys::window()
}

fn get_input_element_from_id(id: String) -> Option<HtmlInputElement> {
	web_sys::window()
		.map(|window| window.document())
		.flatten()
		.map(|document| document.get_element_by_id(id.as_str()))
		.flatten()
		.map(|element| element.dyn_into::<HtmlInputElement>().ok())
		.flatten()
}

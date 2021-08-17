use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn error(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn debug(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn info(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	pub fn warn(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::util::console::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => ($crate::util::console::error(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_debug {
    ($($t:tt)*) => ($crate::util::console::debug(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_info {
    ($($t:tt)*) => ($crate::util::console::info(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => ($crate::util::console::warn(&format_args!($($t)*).to_string()))
}
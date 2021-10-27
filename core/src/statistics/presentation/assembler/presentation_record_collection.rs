use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::presentation::record::PresentationRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationRecordCollection {
	pub(crate) map: HashMap<String,Vec<PresentationRecord>>,
}

impl PresentationRecordCollection {
	pub fn get_map(&self) -> &HashMap<String,Vec<PresentationRecord>> {
		&self.map
	}

	pub fn into_map(self) -> HashMap<String,Vec<PresentationRecord>> {
		self.map
	}
}

impl AsRef<HashMap<String,Vec<PresentationRecord>>> for PresentationRecordCollection {
	fn as_ref(&self) -> &HashMap<String,Vec<PresentationRecord>> {
		self.get_map()
	}
}

impl From<CalculationAssemblerMap> for PresentationRecordCollection {
	fn from(value: CalculationAssemblerMap) -> Self {
		let map = value.map.into_inner().unwrap();

		let map = map.into_iter()
		             .map(|(key, value)| {
			             let values = value.into_inner().unwrap();
			             let values = values.into_iter().map(|item| item.into()).collect();
			             (key, values)
		             })
		             .collect();

		Self {
			map
		}
	}
}
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use crate::statistics::presentation::record::PresentationRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationRecordCollection {
	pub(crate) map: HashMap<u32, HashMap<String,PresentationRecord>>,
	pub(crate) starts: Vec<PresentationRecord>
}

impl PresentationRecordCollection {
	pub fn get_map(&self) -> &HashMap<u32, HashMap<String,PresentationRecord>> {
		&self.map
	}

	pub fn into_map(self) -> HashMap<u32, HashMap<String,PresentationRecord>> {
		self.map
	}
}

impl AsRef<HashMap<u32, HashMap<String,PresentationRecord>>> for PresentationRecordCollection {
	fn as_ref(&self) -> &HashMap<u32, HashMap<String,PresentationRecord>> {
		self.get_map()
	}
}

impl From<CalculationAssemblerMap> for PresentationRecordCollection {
	fn from(value: CalculationAssemblerMap) -> Self {
		let map = value.map.into_inner().unwrap();

		let map = map.into_iter()
		             .map(|(key, value)| {
			             (key, value.into_inner().unwrap())
		             })
					 .map(|(key, value)| {
						 (key, value.into_iter()
							.map(|(key, value)| (key, value.into_inner().unwrap().get(0).unwrap().clone().into()))
							.collect())
					 })
		             .collect();

		let starts = value.starts.into_inner().unwrap();

		let starts = starts.into_iter()
		                   .map(|value| {
			                   value.into()
		                   })
		                   .collect();

		Self {
			map,
			starts
		}
	}
}
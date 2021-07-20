pub mod single_read;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::assembler::map::PresentationAssemblerMap;
use crate::statistics::presentation::per_reference::single_read::SingleReadPerReferencePresentationData;
use crate::statistics::calculation::per_reference::PerReferenceCalculationData;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Deserialize, Serialize)]
pub struct PerReferencePresentationData {
    reference_name: String,
    reference_length: u32,
    read_length_map: PresentationFrequencyMap<u32>,
    single_read_data: SingleReadPerReferencePresentationData,
    split_read_assembler_map: PresentationAssemblerMap,
    covered_length: u32
}

impl PerReferencePresentationData {
    pub fn get_reference_name(&self) -> String {
        self.reference_name.clone()
    }

    pub fn get_reference_length(&self) -> u32 {
        self.reference_length
    }

    pub fn get_read_length_map(&self) -> &PresentationFrequencyMap<u32> {
        &self.read_length_map
    }

    pub fn get_single_read_data(&self) -> &SingleReadPerReferencePresentationData {
        &self.single_read_data
    }

    pub fn get_split_read_data(&self) -> &PresentationAssemblerMap {
        &self.split_read_assembler_map
    }

    pub fn get_covered_length(&self) -> u32 {
        self.covered_length
    }

    pub fn get_covered_percentage(&self) -> f64 {
        self.covered_length as f64 / self.reference_length as f64
    }
}

impl TryFrom<PerReferenceCalculationData> for PerReferencePresentationData {
    type Error = ();

    fn try_from(value: PerReferenceCalculationData) -> Result<Self, Self::Error> {
        let reference_name = value.reference_name;
        let reference_length = value.reference_length;
        let read_length_map = value.read_length_map.into();
        let single_read_data = value.single_read_data.into();
        let split_read_assembler_map = value.split_read_assembler_map.try_into()?;
        let coverage_lock = value.coverage_map.into_inner().unwrap();
        let coverage_map = coverage_lock.combine();
        let covered_length = coverage_map.get_total_covered_length();

        Ok(Self {
            reference_name,
            reference_length,
            read_length_map,
            single_read_data,
            split_read_assembler_map,
            covered_length
        })
    }
}
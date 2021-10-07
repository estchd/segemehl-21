use std::collections::HashMap;

use crate::statistics::presentation::record::PresentationRecord;

pub mod collection;
pub mod presentation_record_collection;

pub struct PresentationAssembler {
    records: Vec<PresentationRecord>
}

impl PresentationAssembler {
    pub fn inner(&self) -> &Vec<PresentationRecord> {
        &self.records
    }

    pub fn into_inner(self) -> Vec<PresentationRecord> {
        self.records
    }

    pub fn try_from_start_record_with_map(start_record: PresentationRecord, map: &HashMap<u32, HashMap<String,PresentationRecord>>) -> Result<Self,()> {
        let name = start_record.get_name();

        let mut records = Vec::<PresentationRecord>::new();

        records.push(start_record.clone());

        let mut current_record = start_record;

        loop {
            if current_record.get_flags().get_is_last_mate() {
                records.push(current_record);
                break;
            }

            let p_next = current_record.get_p_next();

            if p_next == -1 {
                return Err(());
            }

            let next = map.get(&(p_next as u32));

            if let None = next {
                return Err(());
            }

            let next = next.unwrap();

            if !next.contains_key(&name) {
                return Err(());
            }

            let next = next.get(&name).unwrap().clone();

            records.push(current_record);
            current_record = next;
        }

        return Ok(PresentationAssembler {
            records
        })
    }
}
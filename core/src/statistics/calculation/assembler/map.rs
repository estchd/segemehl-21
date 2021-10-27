use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use bam::Record;
use crate::util::get_record_name_as_string;

#[derive(Debug)]
pub struct CalculationAssemblerMap {
    pub(crate) map: RwLock<HashMap<String, Mutex<Vec<Record>>>>
}

impl CalculationAssemblerMap {
    pub fn new() -> Self {
        CalculationAssemblerMap {
            map: Default::default()
        }
    }

    pub fn add_record(&self, record: Record) {
        let name = get_record_name_as_string(&record);

        let read_lock = self.map.read().unwrap();
        if read_lock.contains_key(&name) {
            let mut template_records = read_lock.get(&name).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            drop(read_lock);
            self.insert_new(name, record);
        }
    }

    fn insert_new(&self, name: String, record: Record) {
        let mut write_lock = self.map.write().unwrap();

        if write_lock.contains_key(&name) {
            let mut template_records = write_lock.get(&name).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            write_lock.insert(name, Mutex::new(vec![record]));
        }
    }
}
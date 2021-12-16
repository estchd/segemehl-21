use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use bam::Record;
use crate::util::{get_record_t_len};

#[derive(Debug)]
pub struct CalculationAssemblerMap {
    pub(crate) map: RwLock<HashMap<u32, Mutex<Vec<Record>>>>
}

impl CalculationAssemblerMap {
    pub fn new() -> Self {
        CalculationAssemblerMap {
            map: Default::default()
        }
    }

    pub fn add_record(&self, record: Record) {
        let t_len = get_record_t_len(&record);

        let read_lock = self.map.read().unwrap();
        if read_lock.contains_key(&t_len) {
            let mut template_records = read_lock.get(&t_len).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            drop(read_lock);
            self.insert_new(t_len, record);
        }
    }

    fn insert_new(&self, t_len: u32, record: Record) {
        let mut write_lock = self.map.write().unwrap();

        if write_lock.contains_key(&t_len) {
            let mut template_records = write_lock.get(&t_len).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            write_lock.insert(t_len, Mutex::new(vec![record]));
        }
    }
}
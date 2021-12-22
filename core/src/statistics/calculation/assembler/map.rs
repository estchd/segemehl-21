use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use bam::Record;
use crate::util::{get_record_t_len};

#[derive(Debug)]
pub struct CalculationAssemblerMap {
    pub(crate) normals: RwLock<HashMap<u32, Mutex<Vec<Record>>>>,
    pub(crate) supplementaries: RwLock<HashMap<u32, Mutex<Vec<Record>>>>,
    pub(crate) secondaries: RwLock<HashMap<u32, Mutex<Vec<Record>>>>,
    pub(crate) duplicates: RwLock<HashMap<u32, Mutex<Vec<Record>>>>
}

impl CalculationAssemblerMap {
    pub fn new() -> Self {
        CalculationAssemblerMap {
            normals: Default::default(),
            supplementaries: Default::default(),
            secondaries: Default::default(),
            duplicates: Default::default()
        }
    }

    pub fn add_record(&self, record: Record) {
        let mut normal = true;

        if record.flag().is_supplementary() {
            Self::insert(record.clone(), &self.supplementaries);
            normal = false;
        }
        if record.flag().is_duplicate() {
            Self::insert(record.clone(), &self.duplicates);
            normal = false;
        }
        if record.flag().is_secondary() {
            Self::insert(record.clone(), &self.secondaries);
            normal = false;
        }
        if normal {
            Self::insert(record, &self.normals);
        }
    }

    fn insert(record: Record, map: &RwLock<HashMap<u32, Mutex<Vec<Record>>>>) {
        let t_len = get_record_t_len(&record);

        let read_lock = map.read().unwrap();
        if read_lock.contains_key(&t_len) {
            let mut template_records = read_lock.get(&t_len).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            drop(read_lock);
            Self::insert_new(t_len, record, map);
        }
    }

    fn insert_new(t_len: u32, record: Record, map: &RwLock<HashMap<u32, Mutex<Vec<Record>>>>) {
        let mut write_lock = map.write().unwrap();

        if write_lock.contains_key(&t_len) {
            let mut template_records = write_lock.get(&t_len).unwrap().lock().unwrap();

            template_records.push(record);
        }
        else {
            write_lock.insert(t_len, Mutex::new(vec![record]));
        }
    }
}
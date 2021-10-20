use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use bam::Record;
use crate::util::get_record_name_as_string;
use crate::util::record_to_sam;

#[derive(Debug)]
pub struct CalculationAssemblerMap {
    pub(crate) map: RwLock<HashMap<u32, RwLock<HashMap<String, Mutex<Vec<Record>>>>>>,
    pub(crate) starts: Mutex<Vec<Record>>
}

impl CalculationAssemblerMap {
    pub fn new() -> Self {
        CalculationAssemblerMap {
            map: Default::default(),
            starts: Mutex::new(vec![])
        }
    }

    pub fn add_record(&self, record: Record) {
        if record.flag().first_in_pair() {
            let mut starts_lock = self.starts.lock().unwrap();
            starts_lock.push(record);
            return;
        }

        let start = record.start() as u32;

        let read_lock = self.map.read().unwrap();
        if read_lock.contains_key(&start) {
            let pos_records = read_lock.get(&start).unwrap();

            let pos_records_lock = pos_records.read().unwrap();
            let record_name = get_record_name_as_string(&record);

            if pos_records_lock.contains_key(&record_name) {
                let mutex = pos_records_lock.get(&record_name).unwrap();
                let mut lock = mutex.lock().unwrap();

                println!();
                println!();
                println!("Multiple records at same position {}, with same name {}:", start, record_name);

                for record in lock.iter() {
                    println!("{}", record_to_sam(record));
                }
                println!("{}", record_to_sam(&record));

                lock.push(record);
            }
            else {
                drop(pos_records_lock);
                CalculationAssemblerMap::insert_new_pos_records(record, pos_records);
            }
        }
        else {
            drop(read_lock);
            self.insert_new(start, record);
        }
    }

    fn insert_new(&self, start: u32, record: Record) {
        let mut write_lock = self.map.write().unwrap();

        if write_lock.contains_key(&start) {
            let pos_records = write_lock.get(&start).unwrap();

            let pos_records_lock = pos_records.read().unwrap();
            let record_name = get_record_name_as_string(&record);

            if pos_records_lock.contains_key(&record_name) {
                let mutex = pos_records_lock.get(&record_name).unwrap();
                let mut lock = mutex.lock().unwrap();

                println!();
                println!();
                println!("Multiple records at same position {}, with same name {}:", start, record_name);

                for record in lock.iter() {
                    println!("{}", record_to_sam(record));
                }
                println!("{}", record_to_sam(&record));

                lock.push(record);
            }
            else {
                drop(pos_records_lock);
                CalculationAssemblerMap::insert_new_pos_records(record, pos_records);
            }
        }
        else {
            let mut hash_map = HashMap::<String, Mutex<Vec<Record>>>::new();

            let name = get_record_name_as_string(&record);

            hash_map.insert(name, Mutex::new(vec![record]));

            write_lock.insert(start, RwLock::new(hash_map));
        }
    }

    fn insert_new_pos_records(record: Record, pos_records: &RwLock<HashMap<String, Mutex<Vec<Record>>>>) {
        let mut write_lock = pos_records.write().unwrap();

        let name = get_record_name_as_string(&record);

        if write_lock.contains_key(&name) {
            let mutex = write_lock.get_mut(&name).unwrap();
            let mut vec_lock = mutex.lock().unwrap();

            let start = record.start();

            println!();
            println!();
            println!("Multiple records at same position {}, with same name {}:", start, name);

            for record in vec_lock.iter() {
                println!("{}", record_to_sam(record));
            }
            println!("{}", record_to_sam(&record));

            vec_lock.push(record);
        }
        else {
            write_lock.insert(name, Mutex::new(vec![record]));
        }
    }
}
use std::collections::HashMap;
use std::sync::RwLock;

use bam::Record;

use crate::statistics::calculation::assembler::CalculationAssembler;
use crate::util::get_record_name_as_string;

#[derive(Debug)]
pub struct CalculationAssemblerMap {
    pub(crate) map: RwLock<HashMap<String, CalculationAssembler>>
}

impl CalculationAssemblerMap {
    pub fn new() -> Self {
        CalculationAssemblerMap {
            map: Default::default()
        }
    }

    pub fn add_record(&self, record: Record) -> Result<(),()> {
        let read_lock = self.map.read().unwrap();

        let qname = get_record_name_as_string(&record)?;

        return if read_lock.contains_key(&qname) {
            let entry = read_lock.get(&qname).unwrap();
            entry.add_record(record)
        }
        else {
            drop(read_lock);
            self.insert_new(qname, record)
        }
    }

    fn insert_new(&self, qname: String, record: Record) -> Result<(),()> {
        let mut write_lock = self.map.write().unwrap();

        if write_lock.contains_key(&qname) {
            let entry = write_lock.get(&qname).unwrap();
            entry.add_record(record)?;
        }
        else {
            let assembler = CalculationAssembler::new(record)?;
            write_lock.insert(qname, assembler);
        }

        Ok(())
    }
}
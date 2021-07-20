use std::sync::Mutex;

use bam::Record;

use crate::util::get_record_name_as_string;

pub mod map;

#[derive(Debug)]
pub struct CalculationAssembler {
    pub(crate) qname: String,
    pub(crate) associated_records: Mutex<Vec<Record>>
}

impl CalculationAssembler {
    pub fn new(record: Record) -> Result<Self,()> {
        let name = get_record_name_as_string(&record)?;
        let vec = vec![record];

        Ok(Self {
            qname: name,
            associated_records: Mutex::new(vec)
        })
    }

    pub fn add_record(&self, record: Record) -> Result<(),()> {
        let name = get_record_name_as_string(&record)?;

        if name != self.qname {return Err(());}

        let mut lock = self.associated_records.lock().map_err(|_| ())?;

        lock.push(record);

        Ok(())
    }

    pub fn get_qname(&self) -> &str {
        self.qname.as_str()
    }
}

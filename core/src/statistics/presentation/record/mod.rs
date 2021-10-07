use crate::statistics::presentation::record::flags::PresentationFlags;

use serde_derive::{Deserialize, Serialize};
use bam::Record;
use crate::util::{get_record_name_as_string, get_record_mapping_quality, get_record_length_on_reference, get_record_start, get_record_end};

pub mod flags;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresentationRecord {
    name: String,
    flags: PresentationFlags,
    mapping_quality: u8,
    length: u32,
    p_next: i32,
    start: u32,
    end: u32
}

impl PresentationRecord {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_flags(&self) -> PresentationFlags {
        self.flags
    }

    pub fn get_mapping_quality(&self) -> u8 {
        self.mapping_quality
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_start(&self) -> u32 {
        self.start
    }

    pub fn get_end(&self) -> u32 {
        self.end
    }

    pub fn get_p_next(&self) -> i32 {
        self.p_next
    }
}

impl From<Record> for PresentationRecord {
    fn from(record: Record) -> Self {
        let name = get_record_name_as_string(&record);
        let flags = record.flag().into();
        let mapping_quality = get_record_mapping_quality(&record);
        let length = get_record_length_on_reference(&record);
        let start = get_record_start(&record);
        let end = get_record_end(&record);
        let p_next = record.mate_start();

        Self {
            name,
            flags,
            mapping_quality,
            length,
            p_next,
            start,
            end
        }
    }
}
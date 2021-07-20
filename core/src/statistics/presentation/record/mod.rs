use crate::statistics::presentation::record::flags::PresentationFlags;

use serde_derive::{Deserialize, Serialize};
use bam::Record;
use crate::util::{get_record_name_as_string, get_record_mapping_quality, get_record_length, get_record_start, get_record_end};
use std::convert::TryFrom;

pub mod flags;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationRecord {
    name: String,
    flags: PresentationFlags,
    mapping_quality: u8,
    length: u32,
    start: u32,
    end: u32
}

impl TryFrom<Record> for PresentationRecord {
    type Error = ();

    fn try_from(record: Record) -> Result<Self, Self::Error> {
        let name = get_record_name_as_string(&record)?;
        let flags = record.flag().into();
        let mapping_quality = get_record_mapping_quality(&record);
        let length = get_record_length(&record);
        let start = get_record_start(&record);
        let end = get_record_end(&record);

        Ok(Self {
            name,
            flags,
            mapping_quality,
            length,
            start,
            end
        })
    }
}
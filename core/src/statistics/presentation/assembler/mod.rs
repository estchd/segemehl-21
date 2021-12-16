use crate::statistics::presentation::record::PresentationRecord;

pub mod collection;

pub struct PresentationAssembler {
    pub(crate) associated_records: Vec<PresentationRecord>
}

impl From<Vec<PresentationRecord>> for PresentationAssembler {
    fn from(value: Vec<PresentationRecord>) -> Self {
        Self {
            associated_records: value
        }
    }
}
use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::calculation::assembler::CalculationAssembler;
use std::convert::{TryFrom, TryInto};

pub mod map;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationAssembler {
    qname: String,
    associated_records: Vec<PresentationRecord>
}

impl TryFrom<CalculationAssembler> for PresentationAssembler {
    type Error = ();

    fn try_from(assembler: CalculationAssembler) -> Result<Self, Self::Error> {
        let qname = assembler.qname;
        let associated_records = assembler.associated_records.into_inner().unwrap();
        let associated_records = associated_records.into_iter()
            .map(|record|
                record.try_into()
            ).collect::<Result<Vec<PresentationRecord>, ()>>()?;

        Ok(Self {
            qname,
            associated_records
        })
    }
}
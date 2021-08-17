use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::calculation::assembler::CalculationAssembler;
use std::convert::{TryFrom, TryInto};

pub mod map;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationAssembler {
    qname: String,
    forward_strand_records: Vec<PresentationRecord>,
    reverse_strand_records: Vec<PresentationRecord>,
    unmapped_records: Vec<PresentationRecord>
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

        let mut forward_strand_records: Vec<PresentationRecord> = Vec::new();
        let mut reverse_strand_records: Vec<PresentationRecord> = Vec::new();
        let mut unmapped_records: Vec<PresentationRecord> = Vec::new();

        for record in associated_records {
            let mapped = record.get_flags().get_is_mapped();
            let reverse_strand = record.get_flags().get_is_reverse_strand();

            if !mapped {
                unmapped_records.push(record);
                continue;
            }

            if reverse_strand {
                reverse_strand_records.push(record)
            }
            else {
                forward_strand_records.push(record)
            }
        }

        Ok(Self {
            qname,
            forward_strand_records,
            reverse_strand_records,
            unmapped_records
        })
    }
}
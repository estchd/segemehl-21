use serde_derive::{Deserialize, Serialize};
use crate::statistics::presentation::record::PresentationRecord;
use crate::statistics::calculation::assembler::CalculationAssembler;
use std::convert::{TryFrom, TryInto};
use std::cmp::{Ordering, max, min};
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;

pub mod map;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationAssembler {
    qname: String,
    forward_strand_records: Vec<PresentationRecord>,
    reverse_strand_records: Vec<PresentationRecord>,
    unmapped_records: Vec<PresentationRecord>
}

impl PresentationAssembler {
    pub fn get_gap_length_map(&self, ref_length: u32) -> PresentationFrequencyMap<i64> {
        let calculation_map = CalculationFrequencyMap::new();

        let mut forward_max = 0u32;
        let mut reverse_max = 0u32;

        let mut last_end = None;
        for record in &self.forward_strand_records {
            forward_max = max(forward_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        last_end = None;
        for record in &self.reverse_strand_records {
            reverse_max = max(reverse_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        let forward_gap = ref_length - forward_max;
        let reverse_gap = ref_length - reverse_max;

        let end_gap = forward_gap + reverse_gap;

        calculation_map.add_entry(end_gap as i64);

        calculation_map.into()
    }

    pub fn calculate_gap_lengths_into_map(&self, ref_length: u32, calculation_map: &CalculationFrequencyMap<i64>) {
        let mut forward_max = 0u32;
        let mut reverse_max = 0u32;

        let mut last_end = None;
        for record in &self.forward_strand_records {
            forward_max = max(forward_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        last_end = None;
        for record in &self.reverse_strand_records {
            reverse_max = max(reverse_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        let forward_gap = ref_length - forward_max;
        let reverse_gap = ref_length - reverse_max;

        let end_gap = forward_gap + reverse_gap;

        calculation_map.add_entry(end_gap as i64);
    }

    pub fn get_split_count(&self, include_unmapped: bool) -> usize {
        let mapped_splits =
            self.forward_strand_records.len() +
            self.reverse_strand_records.len();

        return if include_unmapped {
            mapped_splits + self.unmapped_records.len()
        }
        else {
            mapped_splits
        }
    }

    pub fn get_total_length(&self, wrap: Option<u32>) -> u32 {
        let forward_min = self.forward_strand_records
            .get(0)
            .map(|record| record.get_start())
            .unwrap_or(0);

        let mut forward_max = 0u32;
        for record in &self.forward_strand_records {
            forward_max = max(forward_max, record.get_end());
        }

        let reverse_min = self.reverse_strand_records
            .get(0)
            .map(|record| record.get_start())
            .unwrap_or(0);

        let mut reverse_max = 0u32;
        for record in &self.reverse_strand_records {
            reverse_max = max(reverse_max, record.get_end());
        }

        return match wrap {
            None => {
                let min = min(forward_min, reverse_min);
                let max = max(forward_max, reverse_max);

                max - min
            }
            Some(ref_len) => {
                let forward_length = forward_max - forward_min;
                let reverse_length = reverse_max - reverse_min;

                let wrap_forward_length = ref_len - forward_max;
                let wrap_reverse_length = ref_len - reverse_max;

                forward_length +
                    wrap_forward_length +
                    wrap_reverse_length +
                    reverse_length
            }
        };
    }

    pub fn get_statistics(&self, ref_length: u32) -> (PresentationFrequencyMap<i64>, u64, usize, usize) {
        let calculation_map = CalculationFrequencyMap::new();

        let forward_split_count = self.forward_strand_records.len();
        let reverse_split_count = self.reverse_strand_records.len();

        let split_count = forward_split_count + reverse_split_count;
        let split_count_unmapped = forward_split_count + reverse_split_count + self.unmapped_records.len();

        let mut forward_max = 0u32;
        let mut reverse_max = 0u32;

        let forward_min = self.forward_strand_records
            .get(0)
            .map(|record| record.get_start())
            .unwrap_or(0);
        let reverse_min = self.reverse_strand_records
                              .get(0)
                              .map(|record| record.get_start())
                              .unwrap_or(0);

        let mut last_end = None;
        for record in &self.forward_strand_records {
            forward_max = max(forward_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        last_end = None;
        for record in &self.reverse_strand_records {
            reverse_max = max(reverse_max, record.get_end());

            if let Some(end) = last_end {
                let gap_length = record.get_start() as i64 - end as i64;
                calculation_map.add_entry(gap_length);
            }

            last_end = Some(record.get_end());
        }

        let forward_gap = ref_length - forward_max;
        let reverse_gap = ref_length - reverse_max;

        let end_gap = forward_gap as u64 + reverse_gap as u64;

        calculation_map.add_entry(end_gap as i64);

        let forward_length = forward_max - forward_min;
        let reverse_length = reverse_max - reverse_min;

        let total_length =
            forward_length as u64 +
            forward_gap as u64 +
            reverse_gap as u64 +
            reverse_length as u64;

        (calculation_map.into(), total_length, split_count, split_count_unmapped)
    }
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

        let ordering_func: fn(&PresentationRecord, &PresentationRecord) -> Ordering =
            |a,b| {
                return if a.get_start() < b.get_start() {
                    Ordering::Less
                } else if a.get_start() > b.get_start() {
                    Ordering::Greater
                } else {
                    if a.get_end() < b.get_end() {
                        Ordering::Less
                    } else if a.get_end() > b.get_end() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
        };

        forward_strand_records.sort_by(ordering_func);
        reverse_strand_records.sort_by(ordering_func);

        Ok(Self {
            qname,
            forward_strand_records,
            reverse_strand_records,
            unmapped_records
        })
    }
}
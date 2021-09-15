use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::calculation::unmapped::split_read::UnmappedSplitReadCalculationData;
use crate::util::get_record_length_on_reference;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnmappedSplitReadPresentationData {
    read_length_map: PresentationFrequencyMap<u32>
}

impl UnmappedSplitReadPresentationData {
    pub fn get_read_length_map(&self) -> &PresentationFrequencyMap<u32> {
        &self.read_length_map
    }
}

impl AsRef<PresentationFrequencyMap<u32>> for UnmappedSplitReadPresentationData {
    fn as_ref(&self) -> &PresentationFrequencyMap<u32> {
        self.get_read_length_map()
    }
}

impl TryFrom<UnmappedSplitReadCalculationData> for UnmappedSplitReadPresentationData {
    type Error = ();

    fn try_from(value: UnmappedSplitReadCalculationData) -> Result<Self, Self::Error> {
        let mut map = PresentationFrequencyMap::new();

        let entries = value.assembler.map.into_inner().map_err(|_| ())?;

        let assembler_iter = entries.into_iter().map(|(_,entry)| entry);

        for assembler in assembler_iter {
            let records = assembler.associated_records.into_inner().map_err(|_| ())?;

            for record in records {
                map.add_entry(get_record_length_on_reference(&record));
            }
        }

        Ok(Self {
            read_length_map: map,
        })
    }
}
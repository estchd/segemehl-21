use std::collections::HashMap;
use std::convert::TryFrom;

use crate::statistics::presentation::record::PresentationRecord;

pub mod collection;
pub mod presentation_record_collection;

pub struct PresentationAssembler {
    pub(crate) template_length_map: HashMap<i32, Vec<PresentationRecord>>
}

impl TryFrom<Vec<PresentationRecord>> for PresentationAssembler {
    type Error = ();

    fn try_from(value: Vec<PresentationRecord>) -> Result<Self, Self::Error> {
        let mut template_name: Option<String> = None;

        let mut template_length_map: HashMap<i32, Vec<PresentationRecord>> = HashMap::new();

        for record in value {
            let name = record.get_name();

            match &template_name {
                None => { template_name = Some(name) }
                Some(current) => {
                    if current != &name {
                        return Err(())
                    }
                }
            }

            let template_length = record.get_template_length();

            if template_length_map.contains_key(&template_length) {
                let vec = template_length_map.get_mut(&template_length).unwrap();
                vec.push(record);
            }
            else {
                template_length_map.insert(template_length, vec![record]);
            }
        }

        Ok(Self {
            template_length_map
        })
    }
}
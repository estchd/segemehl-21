use crate::statistics::presentation::assembler::PresentationAssembler;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationAssemblerMap {
    map: HashMap<String, PresentationAssembler>
}

impl TryFrom<CalculationAssemblerMap> for PresentationAssemblerMap {
    type Error = ();

    fn try_from(value: CalculationAssemblerMap) -> Result<Self, Self::Error> {
        let map = value.map.into_inner().unwrap();

        let map = map.into_iter()
            .map(|(key, value)| {
                let mapped = (key, value.try_into());

                let mapped = match mapped {
                    (key, Ok(value)) => Ok((key, value)),
                    (_, Err(_)) => Err(())
                };
                mapped
            })
            .collect::<Result<HashMap<String, PresentationAssembler>, ()>>()?;

        Ok(Self {
            map
        })
    }
}
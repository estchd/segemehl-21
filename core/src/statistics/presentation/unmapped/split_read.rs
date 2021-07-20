use crate::statistics::presentation::assembler::map::PresentationAssemblerMap;

use serde_derive::{Deserialize, Serialize};
use crate::statistics::calculation::assembler::map::CalculationAssemblerMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmappedSplitReadPresentationData {
    assembler: PresentationAssemblerMap
}

impl UnmappedSplitReadPresentationData {
    pub fn get_assembler(&self) -> &PresentationAssemblerMap {
        &self.assembler
    }
}

impl AsRef<PresentationAssemblerMap> for UnmappedSplitReadPresentationData {
    fn as_ref(&self) -> &PresentationAssemblerMap {
        self.get_assembler()
    }
}

impl TryFrom<CalculationAssemblerMap> for UnmappedSplitReadPresentationData {
    type Error = ();

    fn try_from(value: CalculationAssemblerMap) -> Result<Self, Self::Error> {
        let assembler = value.try_into()?;

        Ok(Self {
            assembler
        })
    }
}
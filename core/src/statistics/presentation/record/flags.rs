use serde_derive::{Deserialize, Serialize};
use bam::record::Flag;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationFlags {
    is_mapped: bool,
    is_split: bool
}

impl From<Flag> for PresentationFlags {
    fn from(flags: Flag) -> Self {
        let is_mapped = flags.is_mapped();
        let is_split = flags.is_paired();

        Self {
            is_mapped,
            is_split
        }
    }
}


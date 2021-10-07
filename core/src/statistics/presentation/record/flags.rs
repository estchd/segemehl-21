use serde_derive::{Deserialize, Serialize};
use bam::record::Flag;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct PresentationFlags {
    is_mapped: bool,
    is_split: bool,
    is_reverse_strand: bool,
    is_last_mate: bool
}

impl PresentationFlags {
    pub fn get_is_mapped(&self) -> bool {
        self.is_mapped
    }

    pub fn get_is_split(&self) -> bool {
        self.is_split
    }

    pub fn get_is_reverse_strand(&self) -> bool {
        self.is_reverse_strand
    }

    pub fn get_is_last_mate(&self) -> bool {
        self.is_last_mate
    }
}

impl From<Flag> for PresentationFlags {
    fn from(flags: Flag) -> Self {
        let is_mapped = flags.is_mapped();
        let is_split = flags.is_paired();
        let is_reverse_strand = flags.is_reverse_strand();
        let is_last_mate = flags.last_in_pair();

        Self {
            is_mapped,
            is_split,
             is_reverse_strand,
            is_last_mate
        }
    }
}
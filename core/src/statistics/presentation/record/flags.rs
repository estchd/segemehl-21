use serde_derive::{Deserialize, Serialize};
use bam::record::Flag;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct PresentationFlags {
    is_mapped: bool,
    is_split: bool,
    is_reverse_strand: bool,
    is_last_mate: bool,
    is_first_mate: bool,
    is_supplementary: bool,
    is_next_unmapped: bool,
    is_next_reverse_strand: bool,
    is_secondary: bool,
    is_duplicate: bool,
    fails_quality_checks: bool,
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

    pub fn get_is_first_mate(&self) -> bool {
        self.is_first_mate
    }

    pub fn get_is_supplementary(&self) -> bool {
        self.is_supplementary
    }

    pub fn get_is_next_unmapped(&self) -> bool {
        self.is_next_unmapped
    }

    pub fn new(
        is_mapped: bool,
        is_split: bool,
        is_reverse_strand: bool,
        is_last_mate: bool,
        is_first_mate: bool,
        is_supplementary: bool,
        is_next_unmapped: bool,
        is_next_reverse_strand: bool,
        is_secondary: bool,
        is_duplicate: bool,
        fails_quality_checks: bool
    ) -> Self {

        Self {
            is_mapped,
            is_split,
            is_reverse_strand,
            is_last_mate,
            is_first_mate,
            is_supplementary,
            is_next_unmapped,
            is_next_reverse_strand,
            is_secondary,
            is_duplicate,
            fails_quality_checks
        }
    }
}

impl From<Flag> for PresentationFlags {
    fn from(flags: Flag) -> Self {
        let is_mapped = flags.is_mapped();
        let is_split = flags.is_paired();
        let is_reverse_strand = flags.is_reverse_strand();
        let is_next_reverse_strand = flags.mate_is_reverse_strand();
        let is_last_mate = flags.last_in_pair();
        let is_first_mate = flags.first_in_pair();
        let is_secondary = flags.is_secondary();
        let is_supplementary = flags.is_supplementary();
        let is_next_unmapped = !flags.mate_is_mapped();
        let is_duplicate = flags.is_duplicate();
        let fails_quality_checks = flags.fails_quality_controls();

        Self {
            is_mapped,
            is_split,
            is_reverse_strand,
            is_last_mate,
            is_first_mate,
            is_supplementary,
            is_next_unmapped,
            is_next_reverse_strand,
            is_secondary,
            is_duplicate,
            fails_quality_checks
        }
    }
}
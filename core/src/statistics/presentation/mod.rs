use crate::statistics::presentation::per_reference::PerReferencePresentationData;

use serde_derive::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use crate::statistics::calculation::CalculationData;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::util::get_quality_frequency_map;
use crate::statistics::presentation::unmapped::UnmappedPresentationData;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use crate::statistics::presentation::cigar_operations::CigarOperations;
use crate::statistics::presentation::meta::BinConfig::FixedCount;
use crate::statistics::presentation::meta::Meta;

pub mod frequency_map;
pub mod binned;
pub mod per_reference;
pub mod unmapped;
pub mod assembler;
pub mod record;
pub mod cigar_operations;
pub mod meta;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationData {
    per_reference: Vec<PerReferencePresentationData>,
    unmapped: UnmappedPresentationData,
    meta: Meta,
}

impl PresentationData {
    pub fn get_complete_quality_frequency(&self) -> PresentationFrequencyMap<u8> {
        self.per_reference.iter()
            .map(|item| item.get_quality_frequency())
            .fold(PresentationFrequencyMap::<u8>::new(),
                |a,b|
                PresentationFrequencyMap::<u8>::merge(&a, &b)
            )
    }

    pub fn get_read_length_on_reference_map(&self) -> PresentationFrequencyMap<u32> {
        self.per_reference.iter()
            .map(|item| item.get_read_length_on_reference_map())
            .fold(PresentationFrequencyMap::<u32>::new(),
                |a, b|
                    PresentationFrequencyMap::<u32>::merge(&a, &b)
            )
    }

    pub fn get_read_length_sequence_map(&self) -> PresentationFrequencyMap<u32> {
        self.per_reference.iter()
            .map(|item| item.get_read_length_sequence_map())
            .fold(PresentationFrequencyMap::<u32>::new(),
                  |a, b|
                      PresentationFrequencyMap::<u32>::merge(&a, &b)
            )
    }

    pub fn get_assembler_length_map(&self) -> PresentationFrequencyMap<u32> {
        self.per_reference.iter()
            .map(|item| item.get_split_read_data().get_assembler_length_map())
            .fold(PresentationFrequencyMap::new(),
                  |a, b|
                    PresentationFrequencyMap::merge(&a,&b)
            )
    }

    pub fn get_gap_length_map(&self) -> PresentationFrequencyMap<i64> {
        self.per_reference.iter()
            .map(|item| item.get_split_read_data().get_gap_length_map())
            .fold(PresentationFrequencyMap::new(),
                  |a,b|
                      PresentationFrequencyMap::merge(&a,&b)
            )
    }

    pub fn get_split_count_map(&self) -> PresentationFrequencyMap<usize> {
        self.per_reference.iter()
            .map(|item| item.get_split_read_data().get_split_count_map())
            .fold(PresentationFrequencyMap::new(),
                  |a,b|
                PresentationFrequencyMap::merge(&a,&b)
            )
    }

    pub fn get_split_count_unmapped_map(&self) -> PresentationFrequencyMap<usize> {
        self.per_reference.iter()
            .map(|item| item.get_split_read_data().get_split_count_unmapped_map())
            .fold(PresentationFrequencyMap::new(),
                  |a,b|
                      PresentationFrequencyMap::merge(&a,&b)
            )
    }

    pub fn get_complete_quality_frequency_map(&self) -> Vec<(u8,u64)> {
        get_quality_frequency_map(&self.get_complete_quality_frequency())
    }

    pub fn get_per_reference_data(&self) -> impl Iterator<Item = &PerReferencePresentationData> {
        self.per_reference.iter()
    }

    pub fn get_per_reference_by_index(&self, index: usize) -> Option<&PerReferencePresentationData> {
        self.per_reference.get(index)
    }

    pub fn get_per_reference_by_name(&self, name: &String) -> Option<&PerReferencePresentationData> {
        for per_reference in &self.per_reference {
            if &per_reference.get_reference_name() == name {
                return Some(per_reference)
            }
        }

        return None;
    }

    pub fn get_unmapped_data(&self) -> &UnmappedPresentationData {
        &self.unmapped
    }

    pub fn get_cigar_operations(&self) -> CigarOperations {
        self.per_reference.iter().fold(Default::default(), |a,b|{
            CigarOperations::merge(&a, &b.get_cigar_operations())
        })
    }

    pub fn get_least_read_count(&self) -> u64 {
        self.per_reference.iter()
            .map(|item| item.get_single_read_data())
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .min()
            .unwrap_or(0)
    }

    pub fn get_most_read_count(&self) -> u64 {
        self.per_reference.iter()
            .map(|item| item.get_single_read_data())
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .max()
            .unwrap_or(0)
    }

    pub fn get_mean_read_count(&self) -> f64 {
        let mut read_count_map = PresentationFrequencyMap::<u64>::new();

        let iter = self.per_reference.iter()
            .map(|item| item.get_single_read_data())
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum());

        for item in iter {
            read_count_map.add_entry(item);
        }

        read_count_map.get_mean_entry().unwrap()
    }

    pub fn get_median_read_count(&self) -> f64 {
        let mut read_count_map = PresentationFrequencyMap::<u64>::new();

        let iter = self.per_reference.iter()
            .map(|item| item.get_single_read_data())
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum());

        for item in iter {
            read_count_map.add_entry(item);
        }

        read_count_map.get_median_entry().unwrap_or(0.0)
    }

    pub fn get_mode_read_count(&self) -> u64 {
        let mut read_count_map = PresentationFrequencyMap::<u64>::new();

        let iter = self.per_reference.iter()
            .map(|item| item.get_single_read_data())
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum());

        for item in iter {
            read_count_map.add_entry(item);
        }

        read_count_map.get_max_frequency().unwrap_or((0,0)).0
    }
}

impl TryFrom<CalculationData> for PresentationData {
    type Error = ();

    fn try_from(value: CalculationData) -> Result<Self, Self::Error> {
        let mpb = MultiProgress::new();

        let pb = mpb.add(ProgressBar::new(value.per_reference.len() as u64));

        pb.set_message("Calculating Per Reference Statistics...");
        pb.set_prefix("[1/2]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}     {spinner} [{elapsed_precise}] [{bar}] {pos}/{len} ({eta}) {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));
        pb.enable_steady_tick(60/15);

        let per_reference = value.per_reference
            .into_iter()
            .map(|per_reference| {
                let value = PerReferencePresentationData::calculate_from_data(per_reference, &mpb);
                pb.inc(1);
                value
            })
            .collect::<Vec<PerReferencePresentationData>>();

        pb.reset_elapsed();
        pb.reset_eta();

        pb.set_message("Calculating Unmapped Statistics...");
        pb.set_prefix("[2/2]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}     {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));

        let unmapped = value.unmapped.try_into()?;

        pb.finish_with_message("Completed, waiting...");

        mpb.clear().map_err(|_| ())?;

        Ok(Self {
            per_reference,
            unmapped,
            meta: Meta {
                bin_config: FixedCount(1000),
            }
        })
    }
}
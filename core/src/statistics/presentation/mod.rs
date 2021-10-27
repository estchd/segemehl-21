use std::convert::{TryFrom, TryInto};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_derive::{Deserialize, Serialize};

use crate::statistics::calculation::CalculationData;
use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use crate::statistics::presentation::assembler::presentation_record_collection::PresentationRecordCollection;
use crate::statistics::presentation::cigar_operations::CigarOperations;
use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::per_reference::PerReferencePresentationData;
use crate::statistics::presentation::split_read::collection::SplitReadCollection;
use crate::statistics::presentation::split_read::statistics::SplitReadStatistics;
use crate::statistics::presentation::unmapped::UnmappedPresentationData;
use crate::statistics::shared::meta::Meta;
use crate::util::get_quality_frequency_map;

pub mod frequency_map;
pub mod binned;
pub mod per_reference;
pub mod unmapped;
pub mod assembler;
pub mod record;
pub mod cigar_operations;
pub mod split_read;

#[derive(Debug, Serialize, Deserialize)]
pub struct PresentationData {
    per_reference: Vec<PerReferencePresentationData>,
    unmapped: UnmappedPresentationData,
    split_read: SplitReadStatistics,
    meta: Meta,
}

impl PresentationData {
    pub fn get_metadata(&self) -> &Meta {
        &self.meta
    }

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
        self.split_read.get_total_length_map().clone()
    }

    pub fn get_gap_length_map(&self) -> PresentationFrequencyMap<i64> {
        self.split_read.get_gap_length_map().clone()
    }

    pub fn get_split_count_map(&self) -> PresentationFrequencyMap<usize> {
        self.split_read.get_split_count_map().clone()
    }

    pub fn get_split_count_unmapped_map(&self) -> PresentationFrequencyMap<usize> {
        self.split_read.get_split_count_unmapped_map().clone()
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

    pub fn get_median_coverage(&self) -> f64 {
        let total_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .map(|item| item as f64).collect();

        let reference_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_reference_length())
            .map(|item| item as f64).collect();

        let coverage_data: Vec<f64> = total_length_data.iter()
            .zip(reference_length_data.iter())
            .map(|(total_length, reference_length)| {
                *total_length / *reference_length
            })
            .collect();

        let min_coverage = coverage_data.iter().reduce(|a,b| {
            if *a >= *b {
                b
            }
            else {
                a
            }
        }).map(|item| *item).unwrap_or(0.0);

        let max_coverage = coverage_data.iter().reduce(|a,b| {
            if *a >= *b {
                a
            }
            else {
                b
            }
        }).map(|item| *item).unwrap_or(0.0);

        return (min_coverage + max_coverage) / 2.0;
    }

    pub fn get_mean_coverage(&self) -> f64 {
        let total_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .map(|item| item as f64).collect();

        let reference_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_reference_length())
            .map(|item| item as f64).collect();

        let coverage_data: Vec<f64> = total_length_data.iter()
            .zip(reference_length_data.iter())
            .map(|(total_length, reference_length)| {
                *total_length / *reference_length
            })
            .collect();

        let reference_count = self.per_reference.len();

        let sum: f64 = coverage_data.iter().sum();

        return sum / reference_count as f64;
    }

    pub fn get_least_coverage(&self) -> f64 {
        let total_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .map(|item| item as f64).collect();

        let reference_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_reference_length())
            .map(|item| item as f64).collect();

        let coverage_data: Vec<f64> = total_length_data.iter()
            .zip(reference_length_data.iter())
            .map(|(total_length, reference_length)| {
                *total_length / *reference_length
            })
            .collect();

        coverage_data.iter().reduce(|a,b| {
            if *a >= *b {
                b
            }
            else {
                a
            }
        }).map(|item| *item).unwrap_or(0.0)
    }

    pub fn get_most_coverage(&self) -> f64 {
        let total_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .map(|item| item.get_frequency_sum())
            .map(|item| item as f64).collect();

        let reference_length_data: Vec<f64> = self.get_per_reference_data()
            .map(|item| item.get_reference_length())
            .map(|item| item as f64).collect();

        let coverage_data: Vec<f64> = total_length_data.iter()
            .zip(reference_length_data.iter())
            .map(|(total_length, reference_length)| {
                *total_length / *reference_length
            })
            .collect();

        coverage_data.iter().reduce(|a,b| {
            if *a >= *b {
                a
            }
            else {
                b
            }
        }).map(|item| *item).unwrap_or(0.0)
    }
}

impl TryFrom<CalculationData> for PresentationData {
    type Error = ();

    fn try_from(value: CalculationData) -> Result<Self, Self::Error> {
        let mpb = MultiProgress::new();

        let pb = mpb.add(ProgressBar::new(value.per_reference.len() as u64));

        pb.set_message("Calculating Per Reference Statistics...");
        pb.set_prefix("[1/3]");
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
            .collect::<Result<Vec<PerReferencePresentationData>,()>>()?;

        pb.reset_elapsed();
        pb.reset_eta();

        pb.set_message("Calculating Unmapped Statistics...");
        pb.set_prefix("[2/3]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}     {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));

        let unmapped = value.unmapped.try_into()?;

        pb.set_message("Calculating Split Read Statistics...");
        pb.set_prefix("[3/3]");
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix}     {spinner} [{elapsed_precise}] {msg}")
            .progress_chars("#>-")
            .tick_chars("/-\\|"));

        let record_collection: PresentationRecordCollection = value.split_read.into();
        let presentation_assembler_collection: PresentationAssemblerCollection = record_collection.try_into()?;
        let split_read_collection: SplitReadCollection = presentation_assembler_collection.try_into()?;
        let split_read = split_read_collection.into();

        pb.finish_with_message("Completed, waiting...");

        mpb.clear().map_err(|_| ())?;

        Ok(Self {
            split_read,
            per_reference,
            unmapped,
            meta: value.meta
        })
    }
}
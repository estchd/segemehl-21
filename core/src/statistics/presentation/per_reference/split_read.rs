use crate::statistics::presentation::frequency_map::PresentationFrequencyMap;
use crate::statistics::presentation::binned::map::BinnedStatisticsPresentationMap;
use crate::util::get_quality_frequency_map;
use crate::statistics::calculation::per_reference::split_read::SplitReadPerReferenceCalculationData;
use crate::statistics::calculation::frequency_map::CalculationFrequencyMap;
use serde_derive::{Serialize, Deserialize};
use std::convert::TryInto;
use crate::statistics::presentation::assembler::collection::PresentationAssemblerCollection;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::statistics::presentation::assembler::presentation_record_collection::PresentationRecordCollection;
use crate::statistics::presentation::cigar_operations::CigarOperations;
use crate::statistics::presentation::split_read::collection::SplitReadCollection;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SplitReadPerReferencePresentationData {
	quality_map: PresentationFrequencyMap<u8>,
	read_length_on_reference_map: PresentationFrequencyMap<u32>,
	read_length_sequence_map: PresentationFrequencyMap<u32>,
	binned_statistics: BinnedStatisticsPresentationMap,
	assembler_length_map: PresentationFrequencyMap<u32>,
	gap_length_map: PresentationFrequencyMap<i64>,
	split_count_map: PresentationFrequencyMap<usize>,
	split_count_unmapped_map: PresentationFrequencyMap<usize>
}

impl SplitReadPerReferencePresentationData {
	pub fn get_quality_frequency(&self) -> &PresentationFrequencyMap<u8> {
		&self.quality_map
	}

	pub fn get_quality_frequency_map(&self) -> Vec<(u8, u64)> {
		get_quality_frequency_map(&self.quality_map)
	}

	pub fn get_read_length_on_reference_map(&self) -> &PresentationFrequencyMap<u32> {
		&self.read_length_on_reference_map
	}

	pub fn get_read_length_sequence_map(&self) -> &PresentationFrequencyMap<u32> {
		&self.read_length_sequence_map
	}

	pub fn get_binned_statistics(&self) -> &BinnedStatisticsPresentationMap {
		&self.binned_statistics
	}

	pub fn get_assembler_length_map(&self) -> &PresentationFrequencyMap<u32> {
		&self.assembler_length_map
	}

	pub fn get_gap_length_map(&self) -> &PresentationFrequencyMap<i64> {
		&self.gap_length_map
	}

	pub fn get_split_count_map(&self) -> &PresentationFrequencyMap<usize> {
		&self.split_count_map
	}

	pub fn get_split_count_unmapped_map(&self) -> &PresentationFrequencyMap<usize> {
		&self.split_count_unmapped_map
	}

	pub fn get_cigar_operations(&self) -> CigarOperations {
		self.binned_statistics.get_bins().fold(Default::default(), |a,b|
			CigarOperations::merge(&a, &b.get_cigar_operations())
		)
	}

	pub fn from_calculation_data(value: SplitReadPerReferenceCalculationData, ref_length: u32, mpb: &MultiProgress) -> Result<Self,()> {
		let pb = mpb.add(ProgressBar::new(4));
		pb.set_message("Converting Calculation Data...");
		pb.set_prefix("[1/4]");
		pb.set_style(ProgressStyle::default_bar()
			.template("{prefix}         {spinner} [{elapsed_precise}] [{bar}] {pos}/{len} ({eta}) {msg}")
			.progress_chars("#>-")
			.tick_chars("/-\\|"));
		pb.enable_steady_tick(60/15);

		pb.set_position(1);

		let quality_map = value.quality_map.into();

		pb.set_position(2);

		let read_length_on_reference_map = value.read_length_on_reference_map.into();
		let read_length_sequence_map = value.read_length_sequence_map.into();

		pb.set_position(3);

		let binned_statistics = value.binned_statistics.into();

		pb.set_position(4);

		let presentation_record_collection: PresentationRecordCollection = value.assembler.into();
		let presentation_assembler_collection: PresentationAssemblerCollection = presentation_record_collection.try_into()?;
		let split_read_collection: SplitReadCollection = presentation_assembler_collection.into();
		let split_reads = split_read_collection
			.into_inner();

		pb.reset_eta();
		pb.reset_elapsed();
		pb.set_message("Calculating Gap Length Map...");
		pb.set_prefix("[2/4]");
		pb.set_position(0);

		let length = split_reads.len();
		pb.set_length(length as u64);

		let calculated_count = AtomicUsize::new(0);

		let gap_length_map = CalculationFrequencyMap::<i64>::new();

		(&split_reads)
			.into_par_iter()
			.for_each(|assembler| {
				assembler.calculate_gap_lengths_into_map(ref_length, &gap_length_map);

				let current_count = calculated_count.fetch_add(1, Ordering::Relaxed);

				if current_count % 100 == 0 {
					pb.set_position(current_count as u64);
				}
		});

		let gap_length_map = gap_length_map.into();

		pb.reset_eta();
		pb.reset_elapsed();
		pb.set_message("Calculating Length Data...");
		pb.set_prefix("[3/4]");
		pb.set_position(0);
		pb.set_style(ProgressStyle::default_bar()
			.template("{prefix}         {spinner} [{elapsed_precise}] [{bar}] {pos}/{len} ({eta}) {msg}")
			.progress_chars("#>-")
			.tick_chars("/-\\|"));

		calculated_count.store(0, Ordering::Relaxed);

		let assembler_length_map = CalculationFrequencyMap::new();
		let split_count_map = CalculationFrequencyMap::new();
		let split_count_unmapped_map = CalculationFrequencyMap::new();

		(&split_reads)
			.into_par_iter()
			.for_each(
				|assembler| {
				assembler_length_map.add_entry(assembler.get_total_length(Some(ref_length)));
				split_count_map.add_entry(assembler.get_split_count(false));
				split_count_unmapped_map.add_entry(assembler.get_split_count(true));

				let current_count = calculated_count.fetch_add(1, Ordering::Relaxed);

				if current_count % 100 == 0 {
					pb.set_position(current_count as u64);
				}
			}
		);

		pb.reset_eta();
		pb.reset_elapsed();
		pb.set_message("Finishing Data...");
		pb.set_prefix("[4/4]");
		pb.set_length(3);
		pb.set_position(0);
		pb.set_style(ProgressStyle::default_bar()
			.template("{prefix}         {spinner} [{elapsed_precise}] [{bar}] {pos}/{len} {msg}")
			.progress_chars("#>-")
			.tick_chars("/-\\|"));

		let assembler_length_map = assembler_length_map.into();

		pb.set_position(1);

		let split_count_map = split_count_map.into();

		pb.set_position(2);

		let split_count_unmapped_map = split_count_unmapped_map.into();

		pb.set_position(3);

		pb.finish_with_message("completed, waiting...");

		Ok(Self {
			quality_map,
			read_length_on_reference_map,
			read_length_sequence_map,
			binned_statistics,
			assembler_length_map,
			gap_length_map,
			split_count_map,
			split_count_unmapped_map
		})
	}
}
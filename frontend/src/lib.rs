use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use js_sys::{ArrayBuffer, JSON, Uint8Array};

use serde_derive::{Serialize};

use segemehl_21_core::{
    statistics::presentation::frequency_map::PresentationFrequencyMap
};
use segemehl_21_core::statistics::presentation::cigar_operations::CigarOperations;
use segemehl_21_core::statistics::presentation::PresentationData;
use crate::box_plots::{box_plot_from_frequency_maps, BoxPlot, boxplot_entry_from_frequency_map, split_box_plot};
use crate::util::set_panic_hook;

mod box_plots;
mod util;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook()
}

#[derive(Serialize)]
struct FileStats {
    stats: PerFileStatistics,
    data: HashMap<String, Vec<f64>>,
    references: Vec<String>
}

#[derive(Serialize)]
pub struct PerFileStatistics {
    pub number_reads_in_file: u64,
    pub total_length_of_reads_in_file: u64,

    pub length_of_smallest_read: u32,
    pub length_of_longest_read: u32,
    pub smallest_number_of_reads_for_single_chromosome: u64,
    pub biggest_number_of_reads_for_single_chromosome: u64,

    pub median_length_of_read_in_file: f64,
    pub mode_length_of_read_in_file: u32,
    pub mean_length_of_read_in_file: f64,

    pub median_number_of_reads_per_chromosome: f64,
    pub mode_number_of_reads_per_chromosome: u64,
    pub mean_number_of_reads_per_chromosome: f64,

    pub total_chromosome_length: u64,
    pub median_length_of_chromosomes: f64,
    pub mode_length_of_chromosomes: u32,
    pub mean_length_of_chromosomes: f64,
    pub shortest_chromosome_length: u32,
    pub longest_chromosome_length: u32,

    pub median_chromosome_coverage: f64,
    pub mean_chromosome_coverage: f64,
    pub least_chromosome_coverage: f64,
    pub most_chromosome_coverage: f64
}

#[wasm_bindgen]
pub fn generate_file_stats(buffer: ArrayBuffer) -> JsValue {
    let result = match generate_file_stats_inner(buffer) {
        Ok(value) => {
            JsValue::from_str(format!("{{\"result\": \"success\",\"data\": {}}}",JSON::stringify(&value).unwrap()).as_str())
        }
        Err(value) => {
            JsValue::from_str(format!("{{\"result\": \"error\",\"data\": {}}}",JSON::stringify(&value).unwrap()).as_str())
        }
    };
    result
}

pub fn generate_file_stats_inner(buffer: ArrayBuffer) -> Result<JsValue, JsValue>  {
    let array: Uint8Array = Uint8Array::new(&buffer);
    let data: Vec<u8> = array.to_vec();

    let data: String = String::from_utf8(data)
        .map_err(|err| JsValue::from_str(format!("Error Converting File Data to String: {}", err).as_str()))?;

    let deserialized_data: PresentationData = serde_json::from_str(&data)
        .map_err(|err| JsValue::from_str(format!("Error deserializing File Content: {}", err).as_str()))?;

    //let deserialized_data: PresentationData = bincode::deserialize(&data)
    //	.map_err(|err| JsValue::from_str(format!("Error deserializing File Content: {}", err).as_str()))?;

    let references: Vec<String> = deserialized_data
        .get_per_reference_data()
        .map(|item| item.get_reference_name())
        .collect();

    let stats = generate_per_file_stats(&deserialized_data);
    let data = generate_data_repository(&deserialized_data);

    let file_stats = FileStats {
        stats,
        data,
        references
    };

    return JsValue::from_serde(&file_stats).map_err(|_| JsValue::from_str("Error Converting stats to JSON"));
}

fn generate_per_file_stats(statistics: &PresentationData) -> PerFileStatistics {
    let total_chromosome_length: u64 = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length() as u64)
        .sum();

    let chromosome_count = statistics.get_per_reference_data().count();

    let shortest_chromosome_length = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length())
        .min()
        .unwrap_or(0);

    let longest_chromosome_length = statistics.get_per_reference_data()
        .map(|item| item.get_reference_length())
        .max()
        .unwrap_or(0);

    let median_length_of_chromosomes = (longest_chromosome_length as usize + shortest_chromosome_length as usize) as f64 / 2.0;

    let mean_length_of_chromosomes = total_chromosome_length as f64 / chromosome_count as f64;

    let mut chromosome_length_map = PresentationFrequencyMap::<u32>::new();

    for chromosome in statistics.get_per_reference_data() {
        chromosome_length_map.add_entry(chromosome.get_reference_length())
    }
    let (mode_length_of_chromosomes, _) = chromosome_length_map.get_max_frequency().unwrap_or((0,0));

    let complete_read_length_map = statistics.get_read_length_on_reference_map();

    let statistics = PerFileStatistics {
        number_reads_in_file: complete_read_length_map.get_frequency_sum(),
        total_length_of_reads_in_file: complete_read_length_map.get_weighted_frequency_sum(),
        length_of_smallest_read: complete_read_length_map.get_min_entry().unwrap_or((0,0)).0,
        length_of_longest_read: complete_read_length_map.get_max_entry().unwrap_or((0,0)).0,
        smallest_number_of_reads_for_single_chromosome: statistics.get_least_read_count(),
        biggest_number_of_reads_for_single_chromosome: statistics.get_most_read_count(),
        median_length_of_read_in_file:complete_read_length_map.get_median_entry().unwrap_or(0.0),
        mode_length_of_read_in_file: complete_read_length_map.get_max_frequency().unwrap_or((0,0)).0,
        mean_length_of_read_in_file: complete_read_length_map.get_mean_entry().unwrap(),
        median_number_of_reads_per_chromosome: statistics.get_median_read_count(),
        mode_number_of_reads_per_chromosome: statistics.get_mode_read_count(),
        mean_number_of_reads_per_chromosome: statistics.get_mean_read_count(),
        total_chromosome_length,
        median_length_of_chromosomes,
        mode_length_of_chromosomes,
        mean_length_of_chromosomes,
        shortest_chromosome_length,
        longest_chromosome_length,
        median_chromosome_coverage: statistics.get_median_coverage(),
        mean_chromosome_coverage: statistics.get_mean_coverage(),
        least_chromosome_coverage: statistics.get_least_coverage(),
        most_chromosome_coverage: statistics.get_most_coverage()
    };

    return statistics;
}

fn generate_data_repository(data: &PresentationData) -> HashMap<String, Vec<f64>> {
    let mut repository = HashMap::<String, Vec<f64>>::new();

    ////////////////////////////////////////////////////////////////////////////
    // --------------------------- Reference -------------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let reference_length_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_reference_length())
        .map(|item| item as f64).collect();

    repository.insert("reference_length".to_string(), reference_length_data.clone());

    let bin_size_data: Vec<f64> = data.get_per_reference_data().map(
        |_| data.get_metadata().bin_size.get() as f64
    ).collect();

    repository.insert("bin_size".to_string(), bin_size_data);

    ////////////////////////////////////////////////////////////////////////////
    // --------------------------- Split Read ------------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let gap_lengths_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

    let (
        gap_lengths_file_box_min,
        gap_lengths_file_box_q1,
        gap_lengths_file_box_median,
        gap_lengths_file_box_mean,
        gap_lengths_file_box_mode,
        gap_lengths_file_box_q3,
        gap_lengths_file_box_max
    ) = split_box_plot(BoxPlot{ entries: vec![gap_lengths_file_box_plot_entry]});

    repository.insert("gap_lengths_file_min".to_string(), gap_lengths_file_box_min);
    repository.insert("gap_lengths_file_q1".to_string(), gap_lengths_file_box_q1);
    repository.insert("gap_lengths_file_median".to_string(), gap_lengths_file_box_median);
    repository.insert("gap_lengths_file_mean".to_string(), gap_lengths_file_box_mean);
    repository.insert("gap_lengths_file_mode".to_string(), gap_lengths_file_box_mode);
    repository.insert("gap_lengths_file_q3".to_string(), gap_lengths_file_box_q3);
    repository.insert("gap_lengths_file_max".to_string(), gap_lengths_file_box_max);

    let gap_lengths_file_mean: f64 = data
        .get_gap_length_map()
        .get_mean_entry().unwrap();
    let gap_lengths_file_mode: f64 = data
        .get_gap_length_map()
        .get_max_frequency().unwrap_or((0,0)).0 as f64;
    let gap_lengths_file_median: f64 = data
        .get_gap_length_map()
        .get_median_entry().unwrap_or(0.0);
    let gap_lengths_file_shortest: f64 = data
        .get_gap_length_map()
        .get_min_entry()
        .unwrap_or((0,0)).0 as f64;
    let gap_lengths_file_longest: f64 = data
        .get_gap_length_map()
        .get_max_entry()
        .unwrap_or((0,0)).0 as f64;

    let complete_lengths_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

    let (
        complete_lengths_file_box_min,
        complete_lengths_file_box_q1,
        complete_lengths_file_box_median,
        complete_lengths_file_box_mean,
        complete_lengths_file_box_mode,
        complete_lengths_file_box_q3,
        complete_lengths_file_box_max
    ) = split_box_plot(BoxPlot{ entries: vec![complete_lengths_file_box_plot_entry]});

    repository.insert("complete_lengths_file_min".to_string(), complete_lengths_file_box_min);
    repository.insert("complete_lengths_file_q1".to_string(), complete_lengths_file_box_q1);
    repository.insert("complete_lengths_file_median".to_string(), complete_lengths_file_box_median);
    repository.insert("complete_lengths_file_mean".to_string(), complete_lengths_file_box_mean);
    repository.insert("complete_lengths_file_mode".to_string(), complete_lengths_file_box_mode);
    repository.insert("complete_lengths_file_q3".to_string(), complete_lengths_file_box_q3);
    repository.insert("complete_lengths_file_max".to_string(), complete_lengths_file_box_max);

    let complete_lengths_file_mean: f64 = data
        .get_assembler_length_map()
        .get_mean_entry().unwrap();
    let complete_lengths_file_mode: f64 = data
        .get_assembler_length_map()
        .get_max_frequency().unwrap_or((0,0)).0 as f64;
    let complete_lengths_file_median: f64 = data
        .get_assembler_length_map()
        .get_median_entry().unwrap_or(0.0);
    let complete_lengths_file_shortest: f64 = data
        .get_assembler_length_map()
        .get_min_entry()
        .unwrap_or((0,0)).0 as f64;
    let complete_lengths_file_longest: f64 = data
        .get_assembler_length_map()
        .get_max_entry()
        .unwrap_or((0,0)).0 as f64;

    let split_counts_file_box_plot_entry = boxplot_entry_from_frequency_map(data.get_gap_length_map());

    let (
        split_counts_file_box_min,
        split_counts_file_box_q1,
        split_counts_file_box_median,
        split_counts_file_box_mean,
        split_counts_file_box_mode,
        split_counts_file_box_q3,
        split_counts_file_box_max
    ) = split_box_plot(BoxPlot{ entries: vec![split_counts_file_box_plot_entry]});

    repository.insert("split_counts_file_min".to_string(), split_counts_file_box_min);
    repository.insert("split_counts_file_q1".to_string(), split_counts_file_box_q1);
    repository.insert("split_counts_file_median".to_string(), split_counts_file_box_median);
    repository.insert("split_counts_file_mean".to_string(), split_counts_file_box_mean);
    repository.insert("split_counts_file_mode".to_string(), split_counts_file_box_mode);
    repository.insert("split_counts_file_q3".to_string(), split_counts_file_box_q3);
    repository.insert("split_counts_file_max".to_string(), split_counts_file_box_max);

    let split_counts_file_mean: f64 = data
        .get_split_count_map()
        .get_mean_entry().unwrap();
    let split_counts_file_mode: f64 = data
        .get_split_count_map()
        .get_max_frequency().unwrap_or((0,0)).0 as f64;
    let split_counts_file_median: f64 = data
        .get_split_count_map()
        .get_median_entry().unwrap_or(0.0);
    let split_counts_file_shortest: f64 = data
        .get_split_count_map()
        .get_min_entry()
        .unwrap_or((0,0)).0 as f64;
    let split_counts_file_longest: f64 = data
        .get_split_count_map()
        .get_max_entry()
        .unwrap_or((0,0)).0 as f64;

    repository.insert("gap_lengths_file".to_string(), vec![
        gap_lengths_file_mean,
        gap_lengths_file_mode,
        gap_lengths_file_median,
        gap_lengths_file_shortest,
        gap_lengths_file_longest
    ]);
    repository.insert("complete_lengths_file".to_string(), vec![
        complete_lengths_file_mean,
        complete_lengths_file_mode,
        complete_lengths_file_median,
        complete_lengths_file_shortest,
        complete_lengths_file_longest
    ]);
    repository.insert("split_counts_file".to_string(), vec![
        split_counts_file_mean,
        split_counts_file_mode,
        split_counts_file_median,
        split_counts_file_shortest,
        split_counts_file_longest
    ]);

    ////////////////////////////////////////////////////////////////////////////
    // ----------------------------- Unmapped ------------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let unmapped_read_count: u64 = data.get_unmapped_data()
        .get_read_length_map()
        .get_frequency_sum();

    let mapped_read_count: u64 = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_frequency_sum())
        .sum();

    let total_read_count = unmapped_read_count + mapped_read_count;

    let unmapped_read_percentage = (total_read_count as f64 / unmapped_read_count as f64) * 100.0;

    let unmapped_read_length_box_plot_entry = boxplot_entry_from_frequency_map(
        data
            .get_unmapped_data()
            .get_read_length_map()
    );

    let (
        unmapped_read_length_box_min,
        unmapped_read_length_box_q1,
        unmapped_read_length_box_median,
        unmapped_read_length_box_mean,
        unmapped_read_length_box_mode,
        unmapped_read_length_box_q3,
        unmapped_read_length_box_max
    ) = split_box_plot(BoxPlot{entries: vec![unmapped_read_length_box_plot_entry]});

    repository.insert("unmapped_read_length_min".to_string(), unmapped_read_length_box_min);
    repository.insert("unmapped_read_length_q1".to_string(), unmapped_read_length_box_q1);
    repository.insert("unmapped_read_length_median".to_string(), unmapped_read_length_box_median);
    repository.insert("unmapped_read_length_mean".to_string(), unmapped_read_length_box_mean);
    repository.insert("unmapped_read_length_mode".to_string(), unmapped_read_length_box_mode);
    repository.insert("unmapped_read_length_q3".to_string(), unmapped_read_length_box_q3);
    repository.insert("unmapped_read_length_max".to_string(), unmapped_read_length_box_max);

    let mean_unmapped_read_length = data
        .get_unmapped_data()
        .get_read_length_map()
        .get_mean_entry().unwrap();
    let mode_unmapped_read_length = data
        .get_unmapped_data()
        .get_read_length_map()
        .get_max_frequency()
        .unwrap_or((0,0)).0 as f64;
    let median_unmapped_read_length = data
        .get_unmapped_data()
        .get_read_length_map()
        .get_median_entry()
        .unwrap_or(0.0);
    let shortest_unmapped_read_length = data
        .get_unmapped_data()
        .get_read_length_map()
        .get_min_entry()
        .unwrap_or((0,0)).0 as f64;
    let longest_unmapped_read_length = data
        .get_unmapped_data()
        .get_read_length_map()
        .get_max_entry()
        .unwrap_or((0,0)).0 as f64;

    repository.insert("unmapped_read_count".to_string(), vec![unmapped_read_count as f64]);
    repository.insert("unmapped_read_percentage".to_string(), vec![unmapped_read_percentage]);
    repository.insert("unmapped_read_length".to_string(), vec![
        mean_unmapped_read_length,
        mode_unmapped_read_length,
        median_unmapped_read_length,
        shortest_unmapped_read_length,
        longest_unmapped_read_length
    ]);

    ////////////////////////////////////////////////////////////////////////////
    // ---------------------------- Read  Length ---------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let read_length_sequence_file_box_plot_entry = boxplot_entry_from_frequency_map(
        data.get_read_length_sequence_map()
    );

    let (
        read_length_sequence_file_box_min,
        read_length_sequence_file_box_q1,
        read_length_sequence_file_box_median,
        read_length_sequence_file_box_mean,
        read_length_sequence_file_box_mode,
        read_length_sequence_file_box_q3,
        read_length_sequence_file_box_max
    ) = split_box_plot(BoxPlot{entries: vec![read_length_sequence_file_box_plot_entry]});

    repository.insert("read_length_sequence_file_min".to_string(), read_length_sequence_file_box_min);
    repository.insert("read_length_sequence_file_q1".to_string(), read_length_sequence_file_box_q1);
    repository.insert("read_length_sequence_file_median".to_string(), read_length_sequence_file_box_median);
    repository.insert("read_length_sequence_file_mean".to_string(), read_length_sequence_file_box_mean);
    repository.insert("read_length_sequence_file_mode".to_string(), read_length_sequence_file_box_mode);
    repository.insert("read_length_sequence_file_q3".to_string(), read_length_sequence_file_box_q3);
    repository.insert("read_length_sequence_file_max".to_string(), read_length_sequence_file_box_max);

    let mean_read_length_sequence_file = data.get_read_length_sequence_map()
        .get_mean_entry().unwrap();
    let mode_read_length_sequence_file = data.get_read_length_sequence_map()
        .get_max_frequency().unwrap_or((0,0)).0 as f64;
    let median_read_length_sequence_file = data.get_read_length_sequence_map()
        .get_median_entry().unwrap_or(0.0);
    let shortest_read_length_sequence_file = data.get_read_length_sequence_map()
        .get_min_entry().unwrap_or((0,0)).0 as f64;
    let longest_read_length_sequence_file = data.get_read_length_sequence_map()
        .get_max_entry().unwrap_or((0,0)).0 as f64;

    let read_length_sequence_file_data = vec![
        mean_read_length_sequence_file,
        mode_read_length_sequence_file,
        median_read_length_sequence_file,
        shortest_read_length_sequence_file,
        longest_read_length_sequence_file
    ];

    let read_length_reference_file_box_plot_entry = boxplot_entry_from_frequency_map(
        data.get_read_length_on_reference_map()
    );

    let (
        read_length_reference_file_box_min,
        read_length_reference_file_box_q1,
        read_length_reference_file_box_median,
        read_length_reference_file_box_mean,
        read_length_reference_file_box_mode,
        read_length_reference_file_box_q3,
        read_length_reference_file_box_max
    ) = split_box_plot(BoxPlot{entries: vec![read_length_reference_file_box_plot_entry]});

    repository.insert("read_length_reference_file_min".to_string(), read_length_reference_file_box_min);
    repository.insert("read_length_reference_file_q1".to_string(), read_length_reference_file_box_q1);
    repository.insert("read_length_reference_file_median".to_string(), read_length_reference_file_box_median);
    repository.insert("read_length_reference_file_mean".to_string(), read_length_reference_file_box_mean);
    repository.insert("read_length_reference_file_mode".to_string(), read_length_reference_file_box_mode);
    repository.insert("read_length_reference_file_q3".to_string(), read_length_reference_file_box_q3);
    repository.insert("read_length_reference_file_max".to_string(), read_length_reference_file_box_max);

    let mean_read_length_reference_file = data.get_read_length_on_reference_map()
        .get_mean_entry().unwrap();
    let mode_read_length_reference_file = data.get_read_length_on_reference_map()
        .get_max_frequency().unwrap_or((0,0)).0 as f64;
    let median_read_length_reference_file = data.get_read_length_on_reference_map()
        .get_median_entry().unwrap_or(0.0);
    let shortest_read_length_reference_file = data.get_read_length_on_reference_map()
        .get_min_entry().unwrap_or((0,0)).0 as f64;
    let longest_read_length_reference_file = data.get_read_length_on_reference_map()
        .get_max_entry().unwrap_or((0,0)).0 as f64;

    let read_length_reference_file_data = vec![
        mean_read_length_reference_file,
        mode_read_length_reference_file,
        median_read_length_reference_file,
        shortest_read_length_reference_file,
        longest_read_length_reference_file
    ];

    let read_length_sequence_per_reference_box_plot = box_plot_from_frequency_maps(
        data.get_per_reference_data()
            .map(|item| item.get_read_length_sequence_map())
            .collect()
    );

    let (
        read_length_sequence_per_reference_box_min,
        read_length_sequence_per_reference_box_q1,
        read_length_sequence_per_reference_box_median,
        read_length_sequence_per_reference_box_mean,
        read_length_sequence_per_reference_box_mode,
        read_length_sequence_per_reference_box_q3,
        read_length_sequence_per_reference_box_max
    ) = split_box_plot(read_length_sequence_per_reference_box_plot);

    repository.insert("read_length_sequence_per_reference_min".to_string(), read_length_sequence_per_reference_box_min);
    repository.insert("read_length_sequence_per_reference_q1".to_string(), read_length_sequence_per_reference_box_q1);
    repository.insert("read_length_sequence_per_reference_median".to_string(), read_length_sequence_per_reference_box_median);
    repository.insert("read_length_sequence_per_reference_mean".to_string(), read_length_sequence_per_reference_box_mean);
    repository.insert("read_length_sequence_per_reference_mode".to_string(), read_length_sequence_per_reference_box_mode);
    repository.insert("read_length_sequence_per_reference_q3".to_string(), read_length_sequence_per_reference_box_q3);
    repository.insert("read_length_sequence_per_reference_max".to_string(), read_length_sequence_per_reference_box_max);

    let read_length_sequence_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_mean_entry().unwrap())
        .collect();
    let read_length_sequence_per_reference_mode_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_max_frequency())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();
    let read_length_sequence_per_reference_median_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_median_entry())
        .map(|item| item.unwrap_or(0.0))
        .collect();
    let read_length_sequence_per_reference_shortest_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_min_entry())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();
    let read_length_sequence_per_reference_longest_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_sequence_map().get_max_entry())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();

    let read_length_reference_per_reference_box_plot = box_plot_from_frequency_maps(
        data.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .collect()
    );

    let (
        read_length_reference_per_reference_box_min,
        read_length_reference_per_reference_box_q1,
        read_length_reference_per_reference_box_median,
        read_length_reference_per_reference_box_mean,
        read_length_reference_per_reference_box_mode,
        read_length_reference_per_reference_box_q3,
        read_length_reference_per_reference_box_max
    ) = split_box_plot(read_length_reference_per_reference_box_plot);

    repository.insert("read_length_reference_per_reference_min".to_string(), read_length_reference_per_reference_box_min);
    repository.insert("read_length_reference_per_reference_q1".to_string(), read_length_reference_per_reference_box_q1);
    repository.insert("read_length_reference_per_reference_median".to_string(), read_length_reference_per_reference_box_median);
    repository.insert("read_length_reference_per_reference_mean".to_string(), read_length_reference_per_reference_box_mean);
    repository.insert("read_length_reference_per_reference_mode".to_string(), read_length_reference_per_reference_box_mode);
    repository.insert("read_length_reference_per_reference_q3".to_string(), read_length_reference_per_reference_box_q3);
    repository.insert("read_length_reference_per_reference_max".to_string(), read_length_reference_per_reference_box_max);

    let read_length_reference_per_reference_mean_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map().get_mean_entry().unwrap())
        .collect();
    let read_length_reference_per_reference_mode_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map().get_max_frequency())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();
    let read_length_reference_per_reference_median_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map().get_median_entry())
        .map(|item| item.unwrap_or(0.0))
        .collect();
    let read_length_reference_per_reference_shortest_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map().get_min_entry())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();
    let read_length_reference_per_reference_longest_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map().get_max_entry())
        .map(|item| item.unwrap_or((0,0)).0 as f64)
        .collect();

    repository.insert("read_length_sequence_file".to_string(), read_length_sequence_file_data);
    repository.insert("read_length_reference_file".to_string(), read_length_reference_file_data);

    repository.insert("read_length_sequence_per_reference_".to_string(), vec![]);

    repository.insert("read_length_sequence_per_reference_Mean".to_string(), read_length_sequence_per_reference_mean_data);
    repository.insert("read_length_sequence_per_reference_Mode".to_string(), read_length_sequence_per_reference_mode_data);
    repository.insert("read_length_sequence_per_reference_Median".to_string(), read_length_sequence_per_reference_median_data);
    repository.insert("read_length_sequence_per_reference_Shortest".to_string(), read_length_sequence_per_reference_shortest_data);
    repository.insert("read_length_sequence_per_reference_Longest".to_string(), read_length_sequence_per_reference_longest_data);


    repository.insert("read_length_reference_per_reference_".to_string(), vec![]);

    repository.insert("read_length_reference_per_reference_Mean".to_string(), read_length_reference_per_reference_mean_data);
    repository.insert("read_length_reference_per_reference_Mode".to_string(), read_length_reference_per_reference_mode_data);
    repository.insert("read_length_reference_per_reference_Median".to_string(), read_length_reference_per_reference_median_data);
    repository.insert("read_length_reference_per_reference_Shortest".to_string(), read_length_reference_per_reference_shortest_data);
    repository.insert("read_length_reference_per_reference_Longest".to_string(), read_length_reference_per_reference_longest_data);

    ////////////////////////////////////////////////////////////////////////////
    // ------------------------------ Coverage ------------------------------ //
    ////////////////////////////////////////////////////////////////////////////

    let reads_per_chromosome_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map())
        .map(|item| item.get_frequency_sum())
        .map(|item| item as f64).collect();

    let total_read_length_per_reference_data: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_read_length_on_reference_map())
        .map(|item| item.get_weighted_frequency_sum())
        .map(|item| item as f64).collect();

    let coverage_per_reference_data: Vec<f64> = total_read_length_per_reference_data
        .iter()
        .zip(reference_length_data.iter())
        .map(|(total_read_length, reference_length)| *total_read_length / *reference_length)
        .map(|item| item * 100.0)
        .collect();

    repository.insert("read_counts_per_reference".to_string(), reads_per_chromosome_data.clone());
    repository.insert("total_read_length_per_reference".to_string(), total_read_length_per_reference_data.clone());
    repository.insert("coverage_per_reference".to_string(), coverage_per_reference_data.clone());

    for reference in data.get_per_reference_data() {
        let read_count_name = format!("{}_read_counts_per_bin", reference.get_reference_name());
        let total_read_length_name = format!("{}_total_read_length_per_bin", reference.get_reference_name());
        let coverage_name = format!("{}_coverage_per_bin", reference.get_reference_name());

        let read_count_data: Vec<f64> = reference
            .get_binned_statistics()
            .get_bins()
            .map(|item| item.get_read_count())
            .map(|item| item as f64)
            .collect();

        let total_read_length_data: Vec<f64> = reference
            .get_binned_statistics()
            .get_bins()
            .map(|item| item.get_total_read_length())
            .map(|item| item as f64)
            .collect();

        let coverage_data: Vec<f64> = reference
            .get_binned_statistics()
            .get_bins()
            .map(|item| item.get_coverage() * 100.0)
            .collect();


        repository.insert(read_count_name, read_count_data);
        repository.insert(total_read_length_name, total_read_length_data);
        repository.insert(coverage_name, coverage_data);
    }

    ////////////////////////////////////////////////////////////////////////////
    // -------------------------- Cigar  Operation -------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let file_cigar_operations = data.get_cigar_operations();

    let file_total_cigar_operations =
        file_cigar_operations.alignment_matches +
            file_cigar_operations.insertions +
            file_cigar_operations.deletions +
            file_cigar_operations.skips;

    let cigar_total_file_data = vec![
        file_cigar_operations.alignment_matches as f64,
        file_cigar_operations.insertions as f64,
        file_cigar_operations.deletions as f64,
        file_cigar_operations.skips as f64
    ];

    let cigar_percentage_file_data = vec![
        (file_cigar_operations.alignment_matches as f64 / file_total_cigar_operations as f64) * 100.0,
        (file_cigar_operations.insertions as f64 / file_total_cigar_operations as f64) * 100.0,
        (file_cigar_operations.deletions as f64 / file_total_cigar_operations as f64) * 100.0,
        (file_cigar_operations.skips as f64 / file_total_cigar_operations as f64) * 100.0
    ];

    repository.insert("cigar_total_file".to_string(), cigar_total_file_data);
    repository.insert("cigar_percentage_file".to_string(), cigar_percentage_file_data);

    let per_reference_cigar_operations: Vec<CigarOperations> = data.get_per_reference_data()
        .map(|item| item.get_cigar_operations())
        .collect();

    let per_reference_total_operations: Vec<u64> = per_reference_cigar_operations.iter()
        .map(|item|
            item.alignment_matches +
                item.deletions +
                item.insertions +
                item.skips
        ).collect();

    let per_reference_total_match_data: Vec<f64> = per_reference_cigar_operations.iter()
        .map(|item| item.alignment_matches as f64).collect();

    let per_reference_total_insertion_data: Vec<f64> = per_reference_cigar_operations.iter()
        .map(|item| item.insertions as f64).collect();

    let per_reference_total_deletion_data: Vec<f64> = per_reference_cigar_operations.iter()
        .map(|item| item.deletions as f64).collect();

    let per_reference_total_skip_data: Vec<f64> = per_reference_cigar_operations.iter()
        .map(|item| item.skips as f64).collect();

    let per_reference_percentage_match_data: Vec<f64> = per_reference_total_match_data.iter()
        .zip(per_reference_total_operations.iter())
        .map(|(matches,total)| (*matches / *total as f64) * 100.0)
        .collect();

    let per_reference_percentage_insertion_data: Vec<f64> = per_reference_total_insertion_data.iter()
        .zip(per_reference_total_operations.iter())
        .map(|(insertions,total)| (*insertions / *total as f64) * 100.0)
        .collect();

    let per_reference_percentage_deletion_data: Vec<f64> = per_reference_total_deletion_data.iter()
        .zip(per_reference_total_operations.iter())
        .map(|(deletions,total)| (*deletions / *total as f64) * 100.0)
        .collect();

    let per_reference_percentage_skip_data: Vec<f64> = per_reference_total_skip_data.iter()
        .zip(per_reference_total_operations.iter())
        .map(|(skips,total)| (*skips / *total as f64) * 100.0)
        .collect();

    repository.insert("cigar_total_per_reference_match".to_string(), per_reference_total_match_data);
    repository.insert("cigar_total_per_reference_insertion".to_string(), per_reference_total_insertion_data);
    repository.insert("cigar_total_per_reference_deletion".to_string(), per_reference_total_deletion_data);
    repository.insert("cigar_total_per_reference_skip".to_string(), per_reference_total_skip_data);
    repository.insert("cigar_percentage_per_reference_match".to_string(), per_reference_percentage_match_data);
    repository.insert("cigar_percentage_per_reference_insertion".to_string(), per_reference_percentage_insertion_data);
    repository.insert("cigar_percentage_per_reference_deletion".to_string(), per_reference_percentage_deletion_data);
    repository.insert("cigar_percentage_per_reference_skip".to_string(), per_reference_percentage_skip_data);

    for reference in data.get_per_reference_data() {
        let name = reference.get_reference_name();

        let per_bin_cigar_operations: Vec<CigarOperations> = reference.get_binned_statistics().get_bins()
            .map(|item| item.get_cigar_operations())
            .collect();

        let per_bin_total_operations: Vec<u64> = per_bin_cigar_operations.iter()
            .map(|item|
                item.alignment_matches +
                    item.deletions +
                    item.insertions +
                    item.skips
            ).collect();

        let per_bin_total_match_data: Vec<f64> = per_bin_cigar_operations.iter()
            .map(|item| item.alignment_matches as f64).collect();

        let per_bin_total_insertion_data: Vec<f64> = per_bin_cigar_operations.iter()
            .map(|item| item.insertions as f64).collect();

        let per_bin_total_deletion_data: Vec<f64> = per_bin_cigar_operations.iter()
            .map(|item| item.deletions as f64).collect();

        let per_bin_total_skip_data: Vec<f64> = per_bin_cigar_operations.iter()
            .map(|item| item.skips as f64).collect();

        let per_bin_percentage_match_data: Vec<f64> = per_bin_total_match_data.iter()
            .zip(per_bin_total_operations.iter())
            .map(|(matches,total)| (*matches / *total as f64) * 100.0)
            .collect();

        let per_bin_percentage_insertion_data: Vec<f64> = per_bin_total_insertion_data.iter()
            .zip(per_bin_total_operations.iter())
            .map(|(insertions,total)| (*insertions / *total as f64) * 100.0)
            .collect();

        let per_bin_percentage_deletion_data: Vec<f64> = per_bin_total_deletion_data.iter()
            .zip(per_bin_total_operations.iter())
            .map(|(deletions,total)| (*deletions / *total as f64) * 100.0)
            .collect();

        let per_bin_percentage_skip_data: Vec<f64> = per_bin_total_skip_data.iter()
            .zip(per_bin_total_operations.iter())
            .map(|(skips,total)| (*skips / *total as f64) * 100.0)
            .collect();

        repository.insert(format!("{}_cigar_total_per_bin_match",&name), per_bin_total_match_data);
        repository.insert(format!("{}_cigar_total_per_bin_insertion",&name), per_bin_total_insertion_data);
        repository.insert(format!("{}_cigar_total_per_bin_deletion",&name), per_bin_total_deletion_data);
        repository.insert(format!("{}_cigar_total_per_bin_skip",&name), per_bin_total_skip_data);
        repository.insert(format!("{}_cigar_percentage_per_bin_match",&name), per_bin_percentage_match_data);
        repository.insert(format!("{}_cigar_percentage_per_bin_insertion",&name), per_bin_percentage_insertion_data);
        repository.insert(format!("{}_cigar_percentage_per_bin_deletion",&name), per_bin_percentage_deletion_data);
        repository.insert(format!("{}_cigar_percentage_per_bin_skip",&name), per_bin_percentage_skip_data);
    }

    ////////////////////////////////////////////////////////////////////////////
    // ------------------------------ Quality ------------------------------- //
    ////////////////////////////////////////////////////////////////////////////

    let file_quality_map: Vec<f64> = data.get_complete_quality_frequency_map()
        .into_iter()
        .map(|(_,b)| b)
        .map(|item| item as f64).collect();

    repository.insert("read_quality_file".to_string(), file_quality_map);

    let read_quality_per_reference_box_plot = box_plot_from_frequency_maps(
        data.get_per_reference_data()
            .map(|item| item.get_read_length_on_reference_map())
            .collect()
    );

    let (
        read_quality_per_reference_box_min,
        read_quality_per_reference_box_q1,
        read_quality_per_reference_box_median,
        read_quality_per_reference_box_mean,
        read_quality_per_reference_box_mode,
        read_quality_per_reference_box_q3,
        read_quality_per_reference_box_max
    ) = split_box_plot(read_quality_per_reference_box_plot);

    repository.insert("read_quality_per_reference_min".to_string(), read_quality_per_reference_box_min);
    repository.insert("read_quality_per_reference_q1".to_string(), read_quality_per_reference_box_q1);
    repository.insert("read_quality_per_reference_median".to_string(), read_quality_per_reference_box_median);
    repository.insert("read_quality_per_reference_mean".to_string(), read_quality_per_reference_box_mean);
    repository.insert("read_quality_per_reference_mode".to_string(), read_quality_per_reference_box_mode);
    repository.insert("read_quality_per_reference_q3".to_string(), read_quality_per_reference_box_q3);
    repository.insert("read_quality_per_reference_max".to_string(), read_quality_per_reference_box_max);

    let per_reference_read_quality_mean: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_quality_frequency().get_mean_entry().unwrap())
        .collect();
    let per_reference_read_quality_mode: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_quality_frequency().get_max_frequency())
        .map(|item| item.map(|(quality,_)| quality ))
        .map(|item| item.unwrap_or(0) as f64)
        .collect();
    let per_reference_read_quality_median: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_quality_frequency().get_median_entry())
        .map(|item| item.unwrap_or(0.0))
        .collect();
    let per_reference_read_quality_min: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_quality_frequency().get_min_entry())
        .map(|item| item.map(|(quality,_)| quality))
        .map(|item| item.unwrap_or(0) as f64)
        .collect();
    let per_reference_read_quality_max: Vec<f64> = data.get_per_reference_data()
        .map(|item| item.get_quality_frequency().get_max_entry())
        .map(|item| item.map(|(quality,_)| quality))
        .map(|item| item.unwrap_or(0) as f64)
        .collect();

    repository.insert("read_quality_per_reference_".to_string(), vec![]);

    repository.insert("read_quality_per_reference_Mean".to_string(), per_reference_read_quality_mean);
    repository.insert("read_quality_per_reference_Mode".to_string(), per_reference_read_quality_mode);
    repository.insert("read_quality_per_reference_Median".to_string(), per_reference_read_quality_median);
    repository.insert("read_quality_per_reference_Min".to_string(), per_reference_read_quality_min);
    repository.insert("read_quality_per_reference_Max".to_string(), per_reference_read_quality_max);

    for reference in data.get_per_reference_data() {
        let reference_quality_data: Vec<f64> = reference.get_quality_frequency_map().into_iter()
            .map(|(_,frequency)| frequency as f64)
            .collect();

        let reference_quality_name = format!("{}_read_quality", reference.get_reference_name());

        repository.insert(reference_quality_name, reference_quality_data);
    }

    repository
}
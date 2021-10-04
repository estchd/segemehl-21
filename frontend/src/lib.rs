use wasm_bindgen::prelude::*;

use util::set_panic_hook;

use crate::file_list::FILE_LIST;
use segemehl_21_core::{
    statistics::presentation::frequency_map::PresentationFrequencyMap
};

#[macro_use]
mod util;
mod file_list;
mod chromosome_list;
mod box_plots;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();
    //setup_plots();
}


#[wasm_bindgen]
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
}

#[wasm_bindgen]
pub fn generate_per_file_stats(file_name: String) -> Option<PerFileStatistics> {
    let file_list = FILE_LIST.lock().unwrap();

    let (_,statistics) = file_list.get(&file_name)?;

    if statistics.is_none() {return None;}

    let (statistics,_) = statistics.as_ref().unwrap();

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
    };

    return Some(statistics);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

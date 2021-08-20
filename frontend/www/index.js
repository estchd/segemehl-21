import {setup_file_system} from "./js/file_list";
import {setup_plots} from "./js/plots";
import {setup_functions} from "./js/wasm_binding";

const chromosome_select_input = document.getElementById("chromosome_select");

const main_overlay = document.getElementById("main-overlay");

const total_read_count_output = document.getElementById("total-read-number-output");
const total_read_length_output = document.getElementById("total-read-length-output");
const shortest_read_length_output = document.getElementById("shortest-read-length-output");
const longest_read_length_output = document.getElementById("longest-read-length-output");
const median_read_length_output = document.getElementById("median-read-length-output");
const mode_read_length_output = document.getElementById("mode-read-length-output");
const mean_read_length_output = document.getElementById("mean-read-length-output");

const total_chromosome_length_output = document.getElementById("total-chromosome-length-output");
const median_chromosome_length_output = document.getElementById("median-chromosome-length-output");
const mode_chromosome_length_output = document.getElementById("mode-chromosome-length-output");
const mean_chromosome_length_output = document.getElementById("mean-chromosome-length-output");
const shortest_chromosome_length_output = document.getElementById("shortest-chromosome-length-output");
const longest_chromosome_length_output = document.getElementById("longest-chromosome-length-output");

const median_chromosome_coverage_output = document.getElementById("median-chromosome-coverage-output");
const mean_chromosome_coverage_output = document.getElementById("mean-chromosome-coverage-output");
const least_chromosome_coverage_output = document.getElementById("least-chromosome-coverage-output");
const most_chromosome_coverage_output = document.getElementById("most-chromosome-coverage-output");

const mean_number_of_reads_per_chromosome_output = document.getElementById("mean-number-of-reads-per-chromosome-output");
const median_number_of_reads_per_chromosome_output = document.getElementById("median-number-of-reads-per-chromosome-output");
const mode_number_of_reads_per_chromosome_output = document.getElementById("mode-number-of-reads-per-chromosome-output");

export function setup(
    generate_per_file_stats,
    get_chromosome_names,
    setup_file_list,
    add_file,
    process_file,
    remove_file,
    get_file_list,
    get_dataset,
    get_file_color,
    update_file_color
) {
    setup_functions(
        generate_per_file_stats,
        get_chromosome_names,
        setup_file_list,
        add_file,
        process_file,
        remove_file,
        get_file_list,
        get_dataset,
        get_file_color,
        update_file_color
    );

    setup_plots();

    setup_file_system();
}
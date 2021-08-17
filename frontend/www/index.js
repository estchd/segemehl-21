import {setup_file_system} from "./js/file_list";

let parse_data_file_func;
let generate_per_file_stats_func;
let get_chromosome_names_func;
let update_plot_class_func;
let draw_complete_quality_frequency_plot_func;
let draw_selected_chromosome_quality_frequency_plot_func;
let draw_length_of_chromosome_plot_func;
let draw_covered_length_of_chromosome_plot_func;
let draw_number_of_reads_per_chromosome_plot_func;
let draw_coverage_of_chromosomes_plot_func;
let draw_median_length_of_reads_per_chromosome_plot_func;
let draw_mean_length_of_reads_per_chromosome_plot_func;
let draw_mode_length_of_reads_per_chromosome_plot_func;
let draw_shortest_length_of_reads_per_chromosome_plot_func;
let draw_longest_length_of_reads_per_chromosome_plot_func;
let draw_coverage_per_bin_plot_func;

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
    update_plot_class,
    draw_complete_quality_frequency_plot,
    draw_single_chromosome_quality_frequency_plot,
    draw_coverage_per_bin_plot,
    draw_length_of_chromosome_plot,
    draw_covered_length_of_chromosome_plot,
    draw_number_of_reads_per_chromosome_plot,
    draw_coverage_of_chromosomes_plot,
    draw_median_length_of_reads_per_chromosome_plot,
    draw_mean_length_of_reads_per_chromosome_plot,
    draw_mode_length_of_reads_per_chromosome_plot,
    draw_shortest_length_of_reads_per_chromosome_plot,
    draw_longest_length_of_reads_per_chromosome_plot,
    setup_file_list,
    add_file,
    process_file,
    remove_file,
    get_file_list
) {
    get_chromosome_names_func = get_chromosome_names;
    update_plot_class_func = update_plot_class;
    draw_complete_quality_frequency_plot_func = draw_complete_quality_frequency_plot;
    draw_selected_chromosome_quality_frequency_plot_func = draw_single_chromosome_quality_frequency_plot;
    draw_coverage_per_bin_plot_func = draw_coverage_per_bin_plot;
    draw_length_of_chromosome_plot_func = draw_length_of_chromosome_plot;
    draw_covered_length_of_chromosome_plot_func = draw_covered_length_of_chromosome_plot;
    draw_number_of_reads_per_chromosome_plot_func = draw_number_of_reads_per_chromosome_plot;
    draw_coverage_of_chromosomes_plot_func = draw_coverage_of_chromosomes_plot;
    draw_median_length_of_reads_per_chromosome_plot_func = draw_median_length_of_reads_per_chromosome_plot;
    draw_mean_length_of_reads_per_chromosome_plot_func = draw_mean_length_of_reads_per_chromosome_plot;
    draw_mode_length_of_reads_per_chromosome_plot_func = draw_mode_length_of_reads_per_chromosome_plot;
    draw_shortest_length_of_reads_per_chromosome_plot_func = draw_shortest_length_of_reads_per_chromosome_plot;
    draw_longest_length_of_reads_per_chromosome_plot_func = draw_longest_length_of_reads_per_chromosome_plot;

    setup_file_system(
        setup_file_list,
        add_file,
        process_file,
        remove_file,
        get_file_list,
        get_chromosome_names,
        generate_per_file_stats
    );
}

function onMouseMove(event) {
    let canvas = event.target;
    console.log("Event:");
    console.log("(X: " + event.offsetX + ", Y: " + event.offsetY + ")");
    let actualRect = canvas.getBoundingClientRect();
    let logicX = event.offsetX * canvas.width / actualRect.width;
    let logicY = event.offsetY * canvas.height / actualRect.height;
    console.log("(Logic X: " + logicX + ", Logic Y: " + logicY + ")")
}
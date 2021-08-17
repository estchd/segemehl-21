//Table Rows
import {stat} from "copy-webpack-plugin/dist/utils/promisify";

const per_file_file_row = document.getElementById("numeric-statistics-per-file-file-row");

const number_of_reads_row = document.getElementById("numeric-statistics-number-of-reads-row");
const total_read_length_row = document.getElementById("numeric-statistics-total-read-length-row");
const shortest_read_length_row = document.getElementById("numeric-statistics-shortest-read-length-row");
const longest_read_length_row = document.getElementById("numeric-statistics-longest-read-length-row");
const median_read_length_row = document.getElementById("numeric-statistics-median-read-length-row");
const mode_read_length_row = document.getElementById("numeric-statistics-mode-read-length-row");
const mean_read_length_row = document.getElementById("numeric-statistics-mean-read-length-row");

const chromosome_file_row = document.getElementById("numeric-statistics-chromosome-file-row");

const total_length_of_chromosomes_row = document.getElementById("numeric-statistics-total-length-of-chromosomes-row");
const median_length_of_chromosomes_row = document.getElementById("numeric-statistics-median-length-of-chromosomes-row");
const mode_length_of_chromosomes_row = document.getElementById("numeric-statistics-mode-length-of-chromosomes-row");
const mean_length_of_chromosomes_row = document.getElementById("numeric-statistics-mean-length-of-chromosomes-row");
const shortest_length_of_chromosomes_row = document.getElementById("numeric-statistics-shortest-chromosome-length-row");
const longest_length_of_chromosomes_row = document.getElementById("numeric-statistics-longest-chromosome-length-row");

const chromosome_coverage_file_row = document.getElementById("numeric-statistics-chromosome-coverage-file-row");

const median_chromosome_coverage_row = document.getElementById("numeric-statistics-median-chromosome-coverage-row");
const mean_chromosome_coverage_row = document.getElementById("numeric-statistics-mean-chromosome-coverage-row");
const least_chromosome_coverage_row = document.getElementById("numeric-statistics-least-chromosome-coverage-row");
const most_chromosome_coverage_row = document.getElementById("numeric-statistics-most-chromosome-coverage-row");

const per_chromosome_file_row = document.getElementById("numeric-statistics-per-chromosome-file-row");

const median_number_of_reads_per_chromosome_row = document.getElementById("numeric-statistics-median-number-of-reads-per-chromosome-row");
const mode_number_of_reads_per_chromosome_row = document.getElementById("numeric-statistics-mode-number-of-reads-per-chromosome-row");
const mean_number_of_reads_per_chromosome_row = document.getElementById("numeric-statistics-mean-number-of-reads-per-chromosome-row");

export function rebuild_numeric_statistics(file_names, numeric_statistics) {
    rebuild_per_file_numeric_statistics(file_names, numeric_statistics);
    rebuild_chromosome_numeric_statistics(file_names, numeric_statistics);
    rebuild_chromosome_coverage_numeric_statistics(file_names, numeric_statistics);
    rebuild_per_chromosome_numeric_statistics(file_names, numeric_statistics);
}

function rebuild_per_file_numeric_statistics(file_names, numeric_statistics) {
    let number_of_reads = [];
    let total_read_lengths = [];
    let shortest_read_lengths = [];
    let longest_read_lengths = [];
    let median_read_lengths = [];
    let mode_read_lengths = [];
    let mean_read_lengths = [];

    for (let i = 0; i < file_names.length; i++) {
        const statistic = numeric_statistics[i];

        number_of_reads.push(statistic.number_reads_in_file);
        total_read_lengths.push(statistic.total_length_of_reads_in_file);
        shortest_read_lengths.push(statistic.length_of_smallest_read);
        longest_read_lengths.push(statistic.length_of_longest_read);
        median_read_lengths.push(statistic.median_length_of_read_in_file);
        mode_read_lengths.push(statistic.mode_length_of_read_in_file);
        mean_read_lengths.push(statistic.mean_length_of_read_in_file);
    }

    rebuild_table_header("Per File Statistics", file_names, per_file_file_row);

    rebuild_table_row("Number of Reads", number_of_reads_row, number_of_reads);
    rebuild_table_row("Total Length of Reads", total_read_length_row, total_read_lengths);
    rebuild_table_row("Shortest Read Length", shortest_read_length_row, shortest_read_lengths);
    rebuild_table_row("Longest Read Length", longest_read_length_row, longest_read_lengths);
    rebuild_table_row("Median Read Length", median_read_length_row, median_read_lengths);
    rebuild_table_row("Mode Read Length", mode_read_length_row, mode_read_lengths);
    rebuild_table_row("Mean Read Length", mean_read_length_row, mean_read_lengths);
}

function rebuild_chromosome_numeric_statistics(file_names, numeric_statistics) {
    let total_lengths = [];
    let median_lengths = [];
    let mode_lengths = [];
    let mean_lengths = [];
    let shortest_lengths = [];
    let longest_lengths = [];

    for (let i = 0; i < file_names.length; i++) {
        const statistic = numeric_statistics[i];

        total_lengths.push(statistic.total_chromosome_length);
        median_lengths.push(statistic.median_length_of_chromosomes);
        mode_lengths.push(statistic.mode_length_of_chromosomes);
        mean_lengths.push(statistic.mean_length_of_chromosomes);
        shortest_lengths.push(statistic.shortest_chromosome_length);
        longest_lengths.push(statistic.longest_chromosome_length);
    }

    rebuild_table_header("Chromosome Statistics", file_names, chromosome_file_row);

    rebuild_table_row("Total Length of Chromosomes", total_length_of_chromosomes_row, total_lengths);
    rebuild_table_row("Median Length of Chromosomes", median_length_of_chromosomes_row, median_lengths);
    rebuild_table_row("Mode Length of Chromosomes", mode_length_of_chromosomes_row, mode_lengths);
    rebuild_table_row("Mean Length of Chromosomes", mean_length_of_chromosomes_row, mean_lengths);
    rebuild_table_row("Length of shortest Chromosomes", shortest_length_of_chromosomes_row, shortest_lengths);
    rebuild_table_row("Length of longest Chromosomes", longest_length_of_chromosomes_row, longest_lengths);
}

function rebuild_chromosome_coverage_numeric_statistics(file_names, numeric_statistics) {
    let median_coverages = [];
    let mean_coverages = [];
    let least_coverages = [];
    let most_coverages = [];

    for (let i = 0; i < file_names.length; i++) {
        const statistic = numeric_statistics[i];

        median_coverages.push((statistic.median_chromosome_coverage * 100).toFixed(2) + "%");
        mean_coverages.push((statistic.mean_chromosome_coverage * 100).toFixed(2) + "%");
        least_coverages.push((statistic.least_chromosome_coverage * 100).toFixed(2) + "%");
        most_coverages.push((statistic.most_chromosome_coverage * 100).toFixed(2) + "%");
    }

    rebuild_table_header("Chromosome Coverage Statistics",file_names, chromosome_coverage_file_row);

    rebuild_table_row("Median Coverage of Chromosome", median_chromosome_coverage_row, median_coverages);
    rebuild_table_row("Mean Coverage of Chromosome", mean_chromosome_coverage_row, mean_coverages);
    rebuild_table_row("Least Coverage of Chromosomes", least_chromosome_coverage_row, least_coverages);
    rebuild_table_row("Most Coverage of Chromosomes", most_chromosome_coverage_row, most_coverages);
}

function rebuild_per_chromosome_numeric_statistics(file_names, numeric_statistics) {
    let median_number_of_reads = [];
    let mode_number_of_reads = [];
    let mean_number_of_reads = [];

    for (let i = 0; i < file_names.length; i++) {
        const statistic = numeric_statistics[i];

        median_number_of_reads.push(statistic.median_number_of_reads_per_chromosome);
        mode_number_of_reads.push(statistic.mode_number_of_reads_per_chromosome);
        mean_number_of_reads.push(statistic.mean_number_of_reads_per_chromosome);
   }

    rebuild_table_header("Per Chromosome Statistics", file_names, per_chromosome_file_row);

    rebuild_table_row("Median Number of Reads per Chromosome", median_number_of_reads_per_chromosome_row, median_number_of_reads);
    rebuild_table_row("Mode Number of Reads per Chromosome", mode_number_of_reads_per_chromosome_row, mode_number_of_reads);
    rebuild_table_row("Mean Number of Reads per Chromosomes", mean_number_of_reads_per_chromosome_row, mean_number_of_reads);

}

function rebuild_table_header(header_name, file_names, header_row) {
    clear_table_row(header_row);

    const elements = clone_table_header(header_name, file_names);

    for (const element of elements) {
        header_row.appendChild(element);
    }
}

function rebuild_table_row(stat_name, row, stats) {
    clear_table_row(row);

    const elements = clone_table_row(stat_name, stats);

    for (const element of elements) {
        row.appendChild(element);
    }
}

function clear_table_row(row) {
    row.innerHTML = "";
}

function clone_table_header(header_name, file_names) {
    let first = document.createElement("th");
    first.scope = "row";
    first.innerHTML = header_name;

    let elements = [first];

    for (const name of file_names) {
        let element = document.createElement("th");
        element.scope = "col";
        element.innerHTML = name;

        elements.push(element);
    }

    return elements;
}

function clone_table_row(stat_name, stats) {
        let first = document.createElement("th");
        first.scope = "row";
        first.innerHTML = stat_name;

        let elements = [first];

        for (const stat of stats) {
            let element = document.createElement("td");
            element.innerHTML = stat;

            elements.push(element);
        }

        return elements;
}
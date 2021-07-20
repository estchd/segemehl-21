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

const file_input = document.getElementById("file_input");
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

const top_div = document.getElementById("top-div");

export function setup(
    parse_data_file,
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
    draw_longest_length_of_reads_per_chromosome_plot
) {
    parse_data_file_func = parse_data_file;
    generate_per_file_stats_func = generate_per_file_stats;
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

    bind_input_change_handlers();
}

function bind_input_change_handlers() {
    file_input.addEventListener("change", () => handleInputFileChange(), false);

    document.addEventListener("scroll", handleWindowScroll, false);

    document.getElementById("test_canvas").addEventListener("mousemove", onMouseMove)
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

function handleWindowScroll() {
    let top = top_div.offsetTop;

    if (window.scrollY > top) {
        top_div.classList.toggle("sticky", true);
    }
}

async function handleInputFileChange() {
    if (main_overlay.classList.contains("fade")) {
        main_overlay.classList.remove("fade");
    }

    await recalculateAll();

    if (!main_overlay.classList.contains("fade")) {
        main_overlay.classList.add("fade");
    }
}

async function recalculateAll() {
    const fileList = file_input.files;

    let file = fileList[0];

    if (!file) return;

    await parseData(file);

    update_plot_class_func("all");

    recalculatePerFileStatistics();
    recalculatePerChromosomeStatistics();
    updateChromosomeChoices();
}

function updateChromosomeChoices() {
    let chromosome_names = get_chromosome_names_func();

    chromosome_select_input.innerHTML = "";

    for (const chromosome_name of chromosome_names) {
        var opt = document.createElement('option');
        opt.value = chromosome_name;
        opt.innerHTML = chromosome_name;
        chromosome_select_input.appendChild(opt);
    }

    if (chromosome_names.length > 0) {
        chromosome_select_input.value = chromosome_names[0];
    }
}

async function parseData(file) {
    if (!file || !parse_data_file_func) return;

    let arrayBuffer = await file.arrayBuffer();

    let data = new Uint8Array(arrayBuffer);

    parse_data_file_func(data);
}

function recalculatePerFileStatistics() {
    let stats = generate_per_file_stats_func();
    redrawPerFileStatistics(stats);
}

function redrawPerFileStatistics(stats) {
    total_read_count_output.value = stats.number_reads_in_file;
    total_read_length_output.value = stats.total_length_of_reads_in_file;
    shortest_read_length_output.value = stats.length_of_smallest_read;
    longest_read_length_output.value = stats.length_of_longest_read;
    median_read_length_output.value = stats.median_length_of_read_in_file;
    mode_read_length_output.value = stats.mode_length_of_read_in_file;
    mean_read_length_output.value = stats.mean_length_of_read_in_file;

    total_chromosome_length_output.value = stats.total_chromosome_length;
    median_chromosome_length_output.value = stats.median_length_of_chromosomes;
    mode_chromosome_length_output.value = stats.mode_length_of_chromosomes;
    mean_chromosome_length_output.value = stats.mean_length_of_chromosomes;
    shortest_chromosome_length_output.value = stats.shortest_chromosome_length;
    longest_chromosome_length_output.value = stats.longest_chromosome_length;

    median_chromosome_coverage_output.value = (stats.median_chromosome_coverage * 100).toFixed(2);
    mean_chromosome_coverage_output.value = (stats.mean_chromosome_coverage * 100).toFixed(2);
    least_chromosome_coverage_output.value = (stats.least_chromosome_coverage * 100).toFixed(2);
    most_chromosome_coverage_output.value = (stats.most_chromosome_coverage * 100).toFixed(2);
}

function recalculatePerChromosomeStatistics() {
    let stats = generate_per_file_stats_func();
    redrawPerChromosomeStatistics(stats);
}

function redrawPerChromosomeStatistics(stats) {
    mean_number_of_reads_per_chromosome_output.value = stats.mean_number_of_reads_per_chromosome;
    median_number_of_reads_per_chromosome_output.value = stats.median_number_of_reads_per_chromosome;
    mode_number_of_reads_per_chromosome_output.value = stats.mode_number_of_reads_per_chromosome;
}
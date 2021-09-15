import {get_reference_list, get_dataset, get_file_list} from "./wasm_binding";
import {
    setup_coverage_plots,
    update_reference_dependent_coverage_plots,
    update_coverage_plots
} from "./plots/coverage_plots";
import {setup_cigar_plots, update_cigar_plots, update_reference_dependent_cigar_plots} from "./plots/cigar_plots";
import {
    setup_read_quality_plots,
    update_read_quality_plots,
    update_reference_dependent_read_quality_plots
} from "./plots/read_quality_plots";
import {
    setup_read_length_plots,
    update_read_length_plots,
    update_reference_dependent_read_length_plots
} from "./plots/read_length_plots";


export async function setup_plots() {
    setup_chromosome_length_plot();
    setup_number_reads_plot();
    setup_length_reads_plot();
    setup_file_gap_lengths_plot();
    setup_file_split_counts_plot();

    setup_coverage_plots();
    setup_cigar_plots();
    setup_read_quality_plots();
    setup_read_length_plots();

    await update_all_plots();

    selected_chromosome.addEventListener("change", () => update_reference_dependent_plots());
}

export async function update_all_plots() {
    chromosome_names = get_reference_list();

    let promises = [];

    promises.push(update_chromosome_length_plot());
    promises.push(update_number_reads_plot());
    promises.push(update_length_reads_plot());
    //promises.push(update_file_gap_lengths_plot());
    promises.push(update_file_split_counts_plot());

    update_coverage_plots();
    update_cigar_plots();
    update_read_quality_plots();
    update_read_length_plots();

    update_reference_dependent_plots();

    await Promise.all(promises);
}

function update_reference_dependent_plots() {
    update_reference_dependent_coverage_plots();
    update_reference_dependent_cigar_plots();
    update_reference_dependent_read_quality_plots();
    update_reference_dependent_read_length_plots();
}

export function linking_update_selected_reference(index) {
    let reference_names = get_reference_list();
    if (reference_names[index]) {
        selected_chromosome.value = reference_names[index];
    }
    else {
        selected_chromosome.value = reference_names[0];
    }

    update_reference_dependent_plots();
}

const selected_chromosome = document.getElementById("chromosome-select");

let chromosome_names = [];

let chromosome_length_plot;
let chromosome_length_data;

const chromosome_length_logarithmic = document.getElementById("length_of_chromosomes_logarithmic")

function setup_chromosome_length_plot() {
    chromosome_length_data = {
        labels: chromosome_names,
        datasets: []
    };

    let config = {
        type: 'bar',
        data: chromosome_length_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Length of Chromosomes"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: true,
                },
                y: {
                    stacked: true
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            }
        }
    };

    chromosome_length_plot = new Chart(
        document.getElementById('chromosome_length_canvas'),
        config
    );

    chromosome_length_logarithmic.addEventListener("change", () => update_chromosome_length_plot());
}

async function update_chromosome_length_plot() {
    if (chromosome_length_plot) {
        chromosome_length_data.labels = chromosome_names;
        chromosome_length_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"length_of_chromosomes");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            chromosome_length_data.datasets.push(dataset);
        }

        const logarithmic = chromosome_length_logarithmic.checked;

        if (logarithmic) {
            chromosome_length_plot.config.options.scales.y.type="logarithmic";
            chromosome_length_plot.config.options.scales.y.stacked=false;
            chromosome_length_plot.config.options.scales.x.stacked=false;
        }
        else {
            chromosome_length_plot.config.options.scales.y.type=undefined;
            chromosome_length_plot.config.options.scales.y.stacked=true;
            chromosome_length_plot.config.options.scales.x.stacked=true;
        }

        chromosome_length_plot.update();
    }
}

let number_reads_plot;
let number_reads_data;

const number_reads_logarithmic = document.getElementById("number_reads_logarithmic");

function setup_number_reads_plot() {
    number_reads_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: number_reads_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Number of Reads per Chromosome"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false
            },
            scales: {
                x: {
                    stacked: true,
                },
                y: {
                    stacked: true
                }
            }
        }
    };

    number_reads_logarithmic.addEventListener("change", () => update_number_reads_plot())

    number_reads_plot = new Chart(
        document.getElementById('number_reads_canvas'),
        config
    );
}

async function update_number_reads_plot() {
    if (number_reads_plot) {
        number_reads_data.labels = chromosome_names;
        number_reads_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let data = get_dataset(name, "reads_per_chromosome");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            number_reads_data.datasets.push(dataset);
        }

        const logarithmic = number_reads_logarithmic.checked;

        if (logarithmic) {
            number_reads_plot.config.options.scales.y.type="logarithmic";
            number_reads_plot.config.options.scales.y.stacked=false;
            number_reads_plot.config.options.scales.x.stacked=false;
        }
        else {
            number_reads_plot.config.options.scales.y.type=undefined;
            number_reads_plot.config.options.scales.y.stacked=true;
            number_reads_plot.config.options.scales.x.stacked=true;
        }

        number_reads_plot.update();
    }
}

let length_reads_plot;
let length_reads_data;

const length_reads_logarithmic = document.getElementById("length_reads_logarithmic");
const length_reads_stat = document.getElementById("length_reads_stat");

function setup_length_reads_plot() {
    length_reads_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: length_reads_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Length of Reads per Chromosome"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false
            },
            scales: {
                x: {
                    stacked: true,
                },
                y: {
                    stacked: true
                }
            }
        }
    };

    length_reads_logarithmic.addEventListener("change", () => update_length_reads_plot());
    length_reads_stat.addEventListener("change", () => update_length_reads_plot());

    length_reads_plot = new Chart(
        document.getElementById('length_reads_canvas'),
        config
    );
}

async function update_length_reads_plot() {
    if (length_reads_plot) {
        length_reads_data.labels = chromosome_names;
        length_reads_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let stat = length_reads_stat.value;

            let data = get_dataset(name, stat + "_length_of_read_per_chromosome");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            length_reads_data.datasets.push(dataset);
        }

        const logarithmic = length_reads_logarithmic.checked;

        if (logarithmic) {
            length_reads_plot.config.options.scales.y.type="logarithmic";
            length_reads_plot.config.options.scales.y.stacked=false;
            length_reads_plot.config.options.scales.x.stacked=false;
        }
        else {
            length_reads_plot.config.options.scales.y.type=undefined;
            length_reads_plot.config.options.scales.y.stacked=true;
            length_reads_plot.config.options.scales.x.stacked=true;
        }

        length_reads_plot.update();
    }
}

let file_gap_lengths_data;
let file_gap_lengths_plot;

function setup_file_gap_lengths_plot() {
    file_gap_lengths_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: file_gap_lengths_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Gap Lengths for whole File"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false
            },
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            }
        }
    };

    file_gap_lengths_plot = new Chart(
        document.getElementById('file_gap_lengths_canvas'),
        config
    );
}

async function update_file_gap_lengths_plot() {
    if (file_gap_lengths_plot) {
        file_gap_lengths_data.datasets = [];

        console.log("updating file gap length plot")

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let gap_lengths = get_dataset(name, "file_gap_lengths");
            let gap_length_frequencies = get_dataset(name, "file_gap_length_frequencies");

            console.log("file: " + name + ", gap_length length: " + gap_lengths.length);
            console.log("file: " + name + ", gap_lengths: " + gap_lengths);
            console.log("file: " + name + ", gap_length_frequencies length: " + gap_length_frequencies.length);
            console.log("file: " + name + ", gap_length_frequencies: " + gap_length_frequencies);
            console.log("file: " + name + ", gap_length_frequencies sum: " +  gap_length_frequencies.reduce((a, b) => a + b, 0));

            let data = [];

            for (let i = 0; i < gap_lengths.length; i++) {
                let object = {
                    x: gap_lengths[i],
                    y: gap_length_frequencies[i]
                }

                data.push(object);
            }

            console.log("file: " + name + ", data length: " + data.length);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            file_gap_lengths_data.datasets.push(dataset);
        }

        file_gap_lengths_plot.update();
    }
}

let file_split_counts_data;
let file_split_counts_plot;

const file_split_counts_logarithmic = document.getElementById("file_split_counts_logarithmic");

function setup_file_split_counts_plot() {
    file_split_counts_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: file_split_counts_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Gap Lengths for whole File"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false
            },
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            }
        }
    };

    file_split_counts_logarithmic.addEventListener("change", () => update_file_split_counts_plot())

    file_split_counts_plot = new Chart(
        document.getElementById('file_split_counts_canvas'),
        config
    );
}

async function update_file_split_counts_plot() {
    if (file_split_counts_plot) {

        let file_names = get_file_list();

        let max_split_count = 0;

        for (const file_info of file_names) {
            let name = file_info[0];
            let split_counts = get_dataset(name, "file_split_counts");

            if (!split_counts) continue;

            let file_max_split_count = Math.max(...split_counts);

            max_split_count = Math.max(max_split_count, file_max_split_count);
        }

        let labels = [];

        for (let i = 1; i <= max_split_count; i++) {
            labels.push(i);
        }

        file_split_counts_data.labels = labels;
        file_split_counts_data.datasets = [];

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let split_counts = get_dataset(name, "file_split_counts");
            let split_count_frequencies = get_dataset(name, "file_split_count_frequencies");

            let data = fill_sparse_list(1, max_split_count, split_counts, split_count_frequencies);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            file_split_counts_data.datasets.push(dataset);
        }

        const logarithmic = file_split_counts_logarithmic.checked;

        if (logarithmic) {
            file_split_counts_plot.config.options.scales.y.type="logarithmic";
            file_split_counts_plot.config.options.scales.y.stacked=false;
            file_split_counts_plot.config.options.scales.x.stacked=false;
        }
        else {
            file_split_counts_plot.config.options.scales.y.type=undefined;
            file_split_counts_plot.config.options.scales.y.stacked=true;
            file_split_counts_plot.config.options.scales.x.stacked=true;
        }

        file_split_counts_plot.update();
    }
}

function fill_sparse_list(min, max, indices, values) {
    let result = [];

    let current_index = min;

    for (let i = 0; i < indices.length; i++) {
        while (current_index < indices[i]) {
            result.push(0);
            current_index++;
        }

        result.push(values[i]);
        current_index++;
    }

    while (current_index <= max) {
        result.push(0);
        current_index++;
    }

    return result;
}
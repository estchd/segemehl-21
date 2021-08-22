import {get_chromosome_list, get_dataset, get_file_list} from "./wasm_binding";


export async function setup_plots() {
    setup_test_plot();
    setup_chromosome_length_plot();
    setup_chromosome_covered_length_plot();
    setup_chromosome_coverage_plot();
    setup_chromosome_coverage_per_bin_plot();
    setup_file_quality_plot();
    setup_chromosome_quality_plot();
    setup_number_reads_plot();
    setup_length_reads_plot();

    await update_all_plots();

    selected_chromosome.addEventListener("change", () => update_chromosome_dependent_plots());
}

export async function update_all_plots() {
    chromosome_names = get_chromosome_list();

    let promises = [];

    promises.push(update_test_plot());
    promises.push(update_chromosome_length_plot());
    promises.push(update_chromosome_covered_length_plot());
    promises.push(update_chromosome_coverage_plot());
    promises.push(update_file_quality_plot());
    promises.push(update_number_reads_plot());
    promises.push(update_length_reads_plot());

    promises.push(update_chromosome_dependent_plots());

    await Promise.all(promises)
}

async function update_chromosome_dependent_plots() {
    let promises = [];

    promises.push(update_chromosome_coverage_per_bin_plot());
    promises.push(update_chromosome_quality_plot());

    await Promise.all(promises);
}

const selected_chromosome = document.getElementById("chromosome-select");

let chromosome_names = [];

let test_plot;

function setup_test_plot() {
    const labels = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
    ];
    const data = {
        labels: labels,
        datasets: [{
            label: 'My First dataset',
            backgroundColor: 'rgb(255, 99, 132)',
            borderColor: 'rgb(255, 99, 132)',
            data: [0, 10, 5, 2, 20, 30, 45],
        }]
    };

    const config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Chart-js Test Plot"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true
        }
    };

    test_plot = new Chart(
        document.getElementById('chartjs_test_canvas'),
        config
    );
}

async function update_test_plot() {
    if (test_plot) {
        test_plot.update();
    }
}

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
            const color = file_info[1];

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

let chromosome_covered_length_plot;
let chromosome_covered_length_data;

const chromosome_covered_length_logarithmic = document.getElementById("chromosome_covered_length_logarithmic")

function setup_chromosome_covered_length_plot() {
    chromosome_covered_length_data = {
        labels: chromosome_names,
        datasets: []
    };

    let config = {
        type: 'bar',
        data: chromosome_covered_length_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Covered Length of Chromosomes"
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

    chromosome_covered_length_plot = new Chart(
        document.getElementById('chromosome_covered_length_canvas'),
        config
    );

    chromosome_covered_length_logarithmic.addEventListener("change", () => update_chromosome_covered_length_plot());

}

async function update_chromosome_covered_length_plot() {
    if (chromosome_covered_length_plot) {
        chromosome_covered_length_data.labels = chromosome_names;
        chromosome_covered_length_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const data = get_dataset(name,"covered_length_of_chromosomes");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            chromosome_covered_length_data.datasets.push(dataset);
        }

        const logarithmic = chromosome_covered_length_logarithmic.checked;

        if (logarithmic) {
            chromosome_covered_length_plot.config.options.scales.y.type="logarithmic";
            chromosome_covered_length_plot.config.options.scales.y.stacked=false;
            chromosome_covered_length_plot.config.options.scales.x.stacked=false;
        }
        else {
            chromosome_covered_length_plot.config.options.scales.y.type=undefined;
            chromosome_covered_length_plot.config.options.scales.y.stacked=true;
            chromosome_covered_length_plot.config.options.scales.x.stacked=true;
        }

        chromosome_covered_length_plot.update();
    }
}

let chromosome_coverage_plot;
let chromosome_coverage_data;

function setup_chromosome_coverage_plot() {
    chromosome_coverage_data = {
        labels: chromosome_names,
        datasets: []
    };

    let config = {
        type: 'bar',
        data: chromosome_coverage_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Coverage per Chromosome"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            interaction: {
                mode: 'index',
                intersect: false
            }
        }
    };

    chromosome_coverage_plot = new Chart(
        document.getElementById('chromosome_coverage_canvas'),
        config
    );
}

async function update_chromosome_coverage_plot() {
    if (chromosome_coverage_plot) {
        chromosome_coverage_data.labels = chromosome_names;
        chromosome_coverage_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            let data = get_dataset(name,"coverage_per_chromosome");

            for (let i = 0; i < data.length; i++) {
                data[i] = data[i] * 100;
            }

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            chromosome_coverage_data.datasets.push(dataset);
        }

        chromosome_coverage_plot.update();
    }
}

let chromosome_coverage_per_bin_plot;
let chromosome_coverage_per_bin_data;

const chromosome_coverage_per_bin_stat = document.getElementById("chromosome_coverage_per_bin_stat");

function setup_chromosome_coverage_per_bin_plot() {
    chromosome_coverage_per_bin_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: chromosome_coverage_per_bin_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Coverage per Bin on selected Chromosome"
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
            },
            animation: false
        }
    };

    chromosome_coverage_per_bin_stat.addEventListener("change", () => update_chromosome_coverage_per_bin_plot())

    chromosome_coverage_per_bin_plot = new Chart(
        document.getElementById('chromosome_coverage_per_bin_canvas'),
        config
    );
}

async function update_chromosome_coverage_per_bin_plot() {
    if (chromosome_coverage_per_bin_plot) {
        chromosome_coverage_per_bin_data.datasets = [];

        let file_names = get_file_list();

        let chromosome = selected_chromosome.value;
        let stat = chromosome_coverage_per_bin_stat.value;

        let max_bin_count = 0;

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const dataset_name = chromosome + "_" + stat + "_coverage_per_bin";

            let data = get_dataset(name, dataset_name);

            max_bin_count = Math.max(max_bin_count, data.length);

            for (let i = 0; i < data.length; i++) {
                data[i] = data[i] * 100;
            }

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            chromosome_coverage_per_bin_data.datasets.push(dataset);
        }

        let labels = [];

        for (let i = 0; i < max_bin_count; i++) {
            labels.push(i);
        }

        chromosome_coverage_per_bin_data.labels = labels;
        chromosome_coverage_per_bin_plot.update();
    }
}

let file_quality_plot;
let file_quality_data;

const file_quality_logarithmic = document.getElementById("file_quality_logarithmic");

function setup_file_quality_plot() {
    file_quality_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: file_quality_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Quality Frequency Map for whole File"
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

    file_quality_logarithmic.addEventListener("change", () => update_file_quality_plot())

    file_quality_plot = new Chart(
        document.getElementById('file_quality_canvas'),
        config
    );
}

async function update_file_quality_plot() {
    if (file_quality_plot) {
        let labels = [];

        for (let i = 0; i < 256; i++) {
            labels.push(i);
        }

        file_quality_data.labels = labels;
        file_quality_data.datasets = [];

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            let data = get_dataset(name, "file_quality_frequency_map");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            file_quality_data.datasets.push(dataset);
        }

        const logarithmic = file_quality_logarithmic.checked;

        if (logarithmic) {
            file_quality_plot.config.options.scales.y.type="logarithmic";
            file_quality_plot.config.options.scales.y.stacked=false;
            file_quality_plot.config.options.scales.x.stacked=false;
        }
        else {
            file_quality_plot.config.options.scales.y.type=undefined;
            file_quality_plot.config.options.scales.y.stacked=true;
            file_quality_plot.config.options.scales.x.stacked=true;
        }

        file_quality_plot.update();
    }
}

let chromosome_quality_plot;
let chromosome_quality_data;

const chromosome_quality_logarithmic = document.getElementById("chromosome_quality_logarithmic");

function setup_chromosome_quality_plot() {
    chromosome_quality_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: chromosome_quality_data,
        options: {
            plugins: {
                title: {
                    display: true,
                    text: "Quality Frequency Map selected Chromosome"
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

    chromosome_quality_logarithmic.addEventListener("change", () => update_chromosome_quality_plot())

    chromosome_quality_plot = new Chart(
        document.getElementById('chromosome_quality_canvas'),
        config
    );
}

async function update_chromosome_quality_plot() {
    if (chromosome_quality_plot) {
        let labels = [];

        for (let i = 0; i < 256; i++) {
            labels.push(i);
        }

        chromosome_quality_data.labels = labels;
        chromosome_quality_data.datasets = [];

        let file_names = get_file_list();

        const chromosome = selected_chromosome.value;

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const dataset_name = chromosome + "_quality_frequency_map";

            let data = get_dataset(name, dataset_name);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            chromosome_quality_data.datasets.push(dataset);
        }

        const logarithmic = chromosome_quality_logarithmic.checked;

        if (logarithmic) {
            chromosome_quality_plot.config.options.scales.y.type="logarithmic";
            chromosome_quality_plot.config.options.scales.y.stacked=false;
            chromosome_quality_plot.config.options.scales.x.stacked=false;
        }
        else {
            chromosome_quality_plot.config.options.scales.y.type=undefined;
            chromosome_quality_plot.config.options.scales.y.stacked=true;
            chromosome_quality_plot.config.options.scales.x.stacked=true;
        }

        chromosome_quality_plot.update();
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
            const color = file_info[1];

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
            const color = file_info[1];

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
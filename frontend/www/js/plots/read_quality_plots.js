import {get_reference_list, get_dataset, get_file_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";
import {boxplot_from_separate_arrays, boxplot_tooltip} from "./box_plot";

export function setup_read_quality_plots() {
    setup_read_quality_file_plot();
    setup_read_quality_selected_reference_plot();
    setup_read_quality_per_reference_plot();
}

export function update_read_quality_plots() {
    reference_names = get_reference_list();

    update_read_quality_file_plot();
    update_read_quality_per_reference_plot();
}

export function update_reference_dependent_read_quality_plots() {
    update_read_quality_selected_reference_plot();
}

const selected_reference = document.getElementById("chromosome-select");

let reference_names = [];

let read_quality_file_plot;
const read_quality_file_logarithmic = document.getElementById("read_quality_file_logarithmic");

function setup_read_quality_file_plot() {
    let plot_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: plot_data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Read Quality Map for whole File"
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

    read_quality_file_logarithmic.addEventListener("change", () => update_read_quality_file_plot())

    read_quality_file_plot = new Chart(
        document.getElementById('read_quality_file_canvas'),
        config
    );
}

function update_read_quality_file_plot() {
    if (read_quality_file_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        }

        for (let i = 0; i < 256; i++) {
            plot_data.labels.push(i);
        }

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let data = get_dataset(name, "read_quality_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = read_quality_file_logarithmic.checked;

        if (logarithmic) {
            read_quality_file_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            read_quality_file_plot.config.options.scales.y.type=undefined;
        }

        read_quality_file_plot.data = plot_data;

        read_quality_file_plot.update();
    }
}

let read_quality_selected_reference_plot;
const read_quality_selected_reference_logarithmic = document.getElementById("read_quality_selected_reference_logarithmic");

function setup_read_quality_selected_reference_plot() {
    let plot_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: plot_data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Read Quality Map for selected Reference"
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

    read_quality_selected_reference_logarithmic.addEventListener("change", () => update_read_quality_selected_reference_plot())

    read_quality_selected_reference_plot = new Chart(
        document.getElementById('read_quality_selected_reference_canvas'),
        config
    );
}

function update_read_quality_selected_reference_plot() {
    if (read_quality_selected_reference_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        }

        for (let i = 0; i < 256; i++) {
            plot_data.labels.push(i);
        }

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            let data = get_dataset(name, selected_reference.value + "_read_quality");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = read_quality_selected_reference_logarithmic.checked;

        if (logarithmic) {
            read_quality_selected_reference_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            read_quality_selected_reference_plot.config.options.scales.y.type=undefined;
        }

        read_quality_selected_reference_plot.data = plot_data;

        read_quality_selected_reference_plot.update();
    }
}

let read_quality_per_reference_plot;

function setup_read_quality_per_reference_plot() {
    let plot_data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: plot_data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Read Quality per Reference"
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
            },
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            },
            onClick: function (_, elements) {
                linking_update_selected_reference(elements[0]);
            }
        }
    };

    read_quality_per_reference_plot = new Chart(
        document.getElementById('read_quality_per_reference_canvas'),
        config
    );
}

function update_read_quality_per_reference_plot() {
    if (read_quality_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        }

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"read_quality_per_reference_min"),
                get_dataset(name,"read_quality_per_reference_q1"),
                get_dataset(name,"read_quality_per_reference_median"),
                get_dataset(name,"read_quality_per_reference_mean"),
                get_dataset(name,"read_quality_per_reference_mode"),
                get_dataset(name,"read_quality_per_reference_q3"),
                get_dataset(name,"read_quality_per_reference_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        read_quality_per_reference_plot.data = plot_data;

        read_quality_per_reference_plot.update();
    }
}
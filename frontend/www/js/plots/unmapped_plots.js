import { Chart, registerables } from 'chart.js';
import { BoxPlotController, BoxAndWiskers } from '@sgratzl/chartjs-chart-boxplot';

// register controller in chart.js and ensure the defaults are set
Chart.register(BoxPlotController, BoxAndWiskers);
Chart.register(...registerables)

import {boxplot_from_separate_arrays, boxplot_tooltip} from "./box_plot";
import {get_reference_names} from "../reference_list";
import {get_dataset, get_file_list} from "../file_storage";

export function setup_unmapped_plots() {
    setup_unmapped_read_count_plot();
    setup_unmapped_read_percentage_plot();
    setup_unmapped_read_length_plot();
}

export function update_unmapped_plots() {
    reference_names = get_reference_names();

    update_unmapped_read_count_plot();
    update_unmapped_read_percentage_plot();
    update_unmapped_read_length_plot();
}

export function update_reference_dependent_unmapped_plots() {

}

let reference_names = [];

let unmapped_read_count_plot;
const unmapped_read_count_logarithmic = document.getElementById("unmapped_read_count_logarithmic");

function setup_unmapped_read_count_plot() {
    let data = {
        labels: [""],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Unmapped Read Count"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'File',
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: 'Count',
                    }
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false
        }
    };

    unmapped_read_count_plot = new Chart(
        document.getElementById('unmapped_read_count_canvas'),
        config
    );
}

function update_unmapped_read_count_plot() {
    if (unmapped_read_count_plot) {
        let plot_data = {
            labels: [""],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"unmapped_read_count");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = unmapped_read_count_logarithmic.checked;

        if (logarithmic) {
            unmapped_read_count_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            unmapped_read_count_plot.config.options.scales.y.type=undefined;
        }

        unmapped_read_count_plot.config.data = plot_data;
        unmapped_read_count_plot.update();
    }
}

let unmapped_read_percentage_plot;

function setup_unmapped_read_percentage_plot() {
    let data = {
        labels: [""],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Unmapped Read Percentage"
                },
                tooltip: {
                    enabled: true,
                    callbacks: {
                        label: function(context) {
                            var label = context.dataset.label || '';

                            if (label) {
                                label += ': ';
                            }
                            if (context.parsed.y !== null) {
                                label += new Intl.NumberFormat('en-US').format(context.parsed.y);
                                label += "%";
                            }
                            return label;
                        }
                    }
                }
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'File',
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: 'Percentage',
                    },
                    ticks: {
                        callback: function(value){return value+"%"}
                    }
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false
        }
    };

    unmapped_read_percentage_plot = new Chart(
        document.getElementById('unmapped_read_percentage_canvas'),
        config
    );
}

function update_unmapped_read_percentage_plot() {
    if (unmapped_read_percentage_plot) {
        let plot_data = {
            labels: [""],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"unmapped_read_percentage");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        unmapped_read_percentage_plot.config.data = plot_data;
        unmapped_read_percentage_plot.update();
    }
}

let unmapped_read_length_plot;

function setup_unmapped_read_length_plot() {
    let data = {
        labels: [""],
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Unmapped Read Length"
                },
                tooltip: {
                    callbacks: {
                        label: boxplot_tooltip
                    }
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'File',
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: 'Length',
                    }
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
        }
    };

    unmapped_read_length_plot = new Chart(
        document.getElementById('unmapped_read_length_canvas'),
        config
    );
}

function update_unmapped_read_length_plot() {
    if (unmapped_read_length_plot) {
        let plot_data = {
            labels: [""],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"unmapped_read_length_min"),
                get_dataset(name,"unmapped_read_length_q1"),
                get_dataset(name,"unmapped_read_length_median"),
                get_dataset(name,"unmapped_read_length_mean"),
                get_dataset(name,"unmapped_read_length_mode"),
                get_dataset(name,"unmapped_read_length_q3"),
                get_dataset(name,"unmapped_read_length_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        unmapped_read_length_plot.config.data = plot_data;
        unmapped_read_length_plot.update();

        console.log(unmapped_read_length_plot)
    }
}
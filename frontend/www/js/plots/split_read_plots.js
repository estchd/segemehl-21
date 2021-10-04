import {get_dataset, get_file_list, get_reference_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";
import {boxplot_from_separate_arrays, boxplot_tooltip} from "./box_plot";

export function setup_split_read_plots() {
    setup_gap_lengths_file_plot();
    setup_gap_lengths_per_reference_plot();
    setup_complete_lengths_file_plot();
    setup_complete_lengths_per_reference_plot();
    setup_split_counts_file_plot();
    setup_split_counts_per_reference_plot();
}

export function update_split_read_plots() {
    reference_names = get_reference_list();

    update_gap_lengths_file_plot();
    update_gap_lengths_per_reference_plot();
    update_complete_lengths_file_plot();
    update_complete_lengths_per_reference_plot();
    update_split_counts_file_plot();
    update_split_counts_per_reference_plot();
}

export function update_reference_dependent_split_read_plots() {

}

let reference_names = [];

let gap_lengths_file_plot;

function setup_gap_lengths_file_plot() {
    let data = {
        labels: ["File"],
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Gap Lengths per File"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            }
        }
    };

   gap_lengths_file_plot = new Chart(
        document.getElementById('gap_lengths_file_canvas'),
        config
    );
}

function update_gap_lengths_file_plot() {
    if (gap_lengths_file_plot) {
        let plot_data = {
            labels: ["File"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"gap_lengths_file_min"),
                get_dataset(name,"gap_lengths_file_q1"),
                get_dataset(name,"gap_lengths_file_median"),
                get_dataset(name,"gap_lengths_file_mean"),
                get_dataset(name,"gap_lengths_file_mode"),
                get_dataset(name,"gap_lengths_file_q3"),
                get_dataset(name,"gap_lengths_file_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        gap_lengths_file_plot.config.data = plot_data;
        gap_lengths_file_plot.update();
    }
}

let gap_lengths_per_reference_plot;

function setup_gap_lengths_per_reference_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Gap Lengths per Reference"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            },
            onClick: function (_, elements) {
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
            }
        }
    };

    gap_lengths_per_reference_plot = new Chart(
        document.getElementById('gap_lengths_per_reference_canvas'),
        config
    );
}

function update_gap_lengths_per_reference_plot() {
    if (gap_lengths_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"gap_lengths_per_reference_min"),
                get_dataset(name,"gap_lengths_per_reference_q1"),
                get_dataset(name,"gap_lengths_per_reference_median"),
                get_dataset(name,"gap_lengths_per_reference_mean"),
                get_dataset(name,"gap_lengths_per_reference_mode"),
                get_dataset(name,"gap_lengths_per_reference_q3"),
                get_dataset(name,"gap_lengths_per_reference_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        gap_lengths_per_reference_plot.config.data = plot_data;
        gap_lengths_per_reference_plot.update();
    }
}

let complete_lengths_file_plot;

function setup_complete_lengths_file_plot() {
    let data = {
        labels: ["File"],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Complete Lengths per File"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            },
        }
    };

    complete_lengths_file_plot = new Chart(
        document.getElementById('complete_lengths_file_canvas'),
        config
    );
}

function update_complete_lengths_file_plot() {
    if (complete_lengths_file_plot) {
        let plot_data = {
            labels: ["File"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"complete_lengths_file_min"),
                get_dataset(name,"complete_lengths_file_q1"),
                get_dataset(name,"complete_lengths_file_median"),
                get_dataset(name,"complete_lengths_file_mean"),
                get_dataset(name,"complete_lengths_file_mode"),
                get_dataset(name,"complete_lengths_file_q3"),
                get_dataset(name,"complete_lengths_file_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        complete_lengths_file_plot.config.data = plot_data;
        complete_lengths_file_plot.update();
    }
}

let complete_lengths_per_reference_plot;

function setup_complete_lengths_per_reference_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Complete Lengths per Reference"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            },
            onClick: function (_, elements) {
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
            }
        }
    };

    complete_lengths_per_reference_plot = new Chart(
        document.getElementById('complete_lengths_per_reference_canvas'),
        config
    );
}

function update_complete_lengths_per_reference_plot() {
    if (complete_lengths_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"complete_lengths_per_reference_min"),
                get_dataset(name,"complete_lengths_per_reference_q1"),
                get_dataset(name,"complete_lengths_per_reference_median"),
                get_dataset(name,"complete_lengths_per_reference_mean"),
                get_dataset(name,"complete_lengths_per_reference_mode"),
                get_dataset(name,"complete_lengths_per_reference_q3"),
                get_dataset(name,"complete_lengths_per_reference_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        complete_lengths_per_reference_plot.config.data = plot_data;
        complete_lengths_per_reference_plot.update();
    }
}


let split_counts_file_plot;
const split_counts_file_logarithmic = document.getElementById("split_counts_file_logarithmic");

function setup_split_counts_file_plot() {
    let data = {
        labels: ["File"],
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Split Counts per File"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false
        }
    };

    split_counts_file_plot = new Chart(
        document.getElementById('split_counts_file_canvas'),
        config
    );

    split_counts_file_logarithmic.addEventListener("change", () => update_split_counts_file_plot());
}

function update_split_counts_file_plot() {
    if (split_counts_file_plot) {
        let plot_data = {
            labels: ["File"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"split_counts_file_min"),
                get_dataset(name,"split_counts_file_q1"),
                get_dataset(name,"split_counts_file_median"),
                get_dataset(name,"split_counts_file_mean"),
                get_dataset(name,"split_counts_file_mode"),
                get_dataset(name,"split_counts_file_q3"),
                get_dataset(name,"split_counts_file_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = split_counts_file_logarithmic.checked;

        if (logarithmic) {
            split_counts_file_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            split_counts_file_plot.config.options.scales.y.type=undefined;
        }

        split_counts_file_plot.config.data = plot_data;
        split_counts_file_plot.update();
    }
}

let split_counts_per_reference_plot;
const split_counts_per_reference_logarithmic = document.getElementById("split_counts_per_reference_logarithmic");

function setup_split_counts_per_reference_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Split Counts per Reference"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    stacked: false,
                },
                y: {
                    stacked: false
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            tooltips: {
                callbacks: {
                    boxplotLabel: boxplot_tooltip
                }
            },
            onClick: function (_, elements) {
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
            }
        }
    };

    split_counts_per_reference_plot = new Chart(
        document.getElementById('split_counts_per_reference_canvas'),
        config
    );

    split_counts_per_reference_logarithmic.addEventListener("change", () => update_split_counts_per_reference_plot());
}

function update_split_counts_per_reference_plot() {
    if (split_counts_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = boxplot_from_separate_arrays(
                get_dataset(name,"split_counts_per_reference_min"),
                get_dataset(name,"split_counts_per_reference_q1"),
                get_dataset(name,"split_counts_per_reference_median"),
                get_dataset(name,"split_counts_per_reference_mean"),
                get_dataset(name,"split_counts_per_reference_mode"),
                get_dataset(name,"split_counts_per_reference_q3"),
                get_dataset(name,"split_counts_per_reference_max"),
            );

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = split_counts_per_reference_logarithmic.checked;

        if (logarithmic) {
            split_counts_per_reference_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            split_counts_per_reference_plot.config.options.scales.y.type=undefined;
        }

        split_counts_per_reference_plot.config.data = plot_data;
        split_counts_per_reference_plot.update();
    }
}
import {linking_update_selected_reference} from "../plots";
import {bin_data_to_line_data, calculate_decimation_size, decimate_bin_data_mean, generate_labels} from "./line_plot";
import {get_reference_names} from "../reference_list";
import {get_dataset, get_file_list} from "../file_storage";

export function setup_cigar_plots() {
    setup_cigar_total_file_plot();
    setup_cigar_total_per_reference_plot();
    setup_cigar_total_per_bin_plot();
    setup_cigar_percentage_file_plot();
    setup_cigar_percentage_per_reference_plot();
    setup_cigar_percentage_per_bin_plot();
}

export function update_cigar_plots() {
    reference_names = get_reference_names();

    update_cigar_total_file_plot();
    update_cigar_total_per_reference_plot();
    update_cigar_percentage_file_plot();
    update_cigar_percentage_per_reference_plot();
}

export function update_reference_dependent_cigar_plots() {
    update_cigar_total_per_bin_plot();
    update_cigar_percentage_per_bin_plot();
}

const selected_reference = document.getElementById("chromosome-select");
let reference_names = [];

let cigar_total_file_plot;
const cigar_total_file_logarithmic = document.getElementById("cigar_total_file_logarithmic");

function setup_cigar_total_file_plot() {
    let data = {
        labels: ["Alignment Matches","Insertions","Deletions","Skips"],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations for whole File (Total Count)"
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

    cigar_total_file_plot = new Chart(
        document.getElementById('cigar_total_file_canvas'),
        config
    );

    cigar_total_file_logarithmic.addEventListener("change", () => update_cigar_total_file_plot());
}

function update_cigar_total_file_plot() {
    if (cigar_total_file_plot) {
        let plot_data = {
            labels: ["Alignment Matches","Insertions","Deletions","Skips"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            let colors = file_info[1];
            let background_colors = [colors[1], colors[2], colors[3], colors[4]];

            const data = get_dataset(name,"cigar_total_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: background_colors
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = cigar_total_file_logarithmic.checked;

        if (logarithmic) {
            cigar_total_file_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            cigar_total_file_plot.config.options.scales.y.type=undefined;
        }

        cigar_total_file_plot.config.data = plot_data;
        cigar_total_file_plot.update();
    }
}

let cigar_percentage_file_plot;

function setup_cigar_percentage_file_plot() {
    let data = {
        labels: ["Alignment Matches","Insertions","Deletions","Skips"],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations for whole File (Percentage)"
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
                    stacked: false,
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

    cigar_percentage_file_plot = new Chart(
        document.getElementById('cigar_percentage_file_canvas'),
        config
    );
}

function update_cigar_percentage_file_plot() {
    if (cigar_percentage_file_plot) {
        let plot_data = {
            labels: ["Alignment Matches","Insertions","Deletions","Skips"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            let colors = file_info[1];
            let background_colors = [colors[1], colors[2], colors[3], colors[4]];

            const data = get_dataset(name,"cigar_percentage_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: background_colors
            };

            plot_data.datasets.push(dataset);
        }

        cigar_percentage_file_plot.config.data = plot_data;
        cigar_percentage_file_plot.update();
    }
}

let cigar_total_per_reference_plot;
const cigar_total_per_reference_logarithmic = document.getElementById("cigar_total_per_reference_logarithmic");

function setup_cigar_total_per_reference_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations per reference (Total Count)"
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
            },
            animation: false,
            onClick: function (_, elements) {
                linking_update_selected_reference(elements[0]);
            }
        }
    };

    cigar_total_per_reference_plot = new Chart(
        document.getElementById('cigar_total_per_reference_canvas'),
        config
    );

    cigar_total_per_reference_logarithmic.addEventListener("change", () => update_cigar_total_per_reference_plot());
}

function update_cigar_total_per_reference_plot() {
    if (cigar_total_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const match_data = get_dataset(name,"cigar_total_per_reference_match");
            const insertion_data = get_dataset(name,"cigar_total_per_reference_insertion");
            const deletion_data = get_dataset(name,"cigar_total_per_reference_deletion");
            const skip_data = get_dataset(name,"cigar_total_per_reference_skip");

            let match_dataset = {
                label: name + " Alignment Matches",
                data: match_data,
                backgroundColor: color[1],
                stack: "" + i
            };

            let insertion_dataset = {
                label: name + " Insertions",
                data: insertion_data,
                backgroundColor: color[2],
                stack: "" + i
            }

            let deletion_dataset = {
                label: name + " Deletions",
                data: deletion_data,
                backgroundColor: color[3],
                stack: "" + i
            }
            let skip_dataset = {
                label: name + " Skips",
                data: skip_data,
                backgroundColor: color[4],
                stack: "" + i
            }

            plot_data.datasets.push(match_dataset);
            plot_data.datasets.push(insertion_dataset);
            plot_data.datasets.push(deletion_dataset);
            plot_data.datasets.push(skip_dataset);
        }

        const logarithmic = cigar_total_per_reference_logarithmic.checked;

        if (logarithmic) {
            cigar_total_per_reference_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            cigar_total_per_reference_plot.config.options.scales.y.type=undefined;
        }

        cigar_total_per_reference_plot.config.data = plot_data;
        cigar_total_per_reference_plot.update();
    }
}

let cigar_percentage_per_reference_plot;

function setup_cigar_percentage_per_reference_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations per Reference (Percentage)"
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
                    },
                    mode: "index",
                    intersect: false,
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
                    stacked: true,
                    ticks: {
                        callback: function(value){return value+"%"}
                    }
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            onClick: function (_, elements) {
                linking_update_selected_reference(elements[0]);
            }
        }
    };

    cigar_percentage_per_reference_plot = new Chart(
        document.getElementById('cigar_percentage_per_reference_canvas'),
        config
    );
}

function update_cigar_percentage_per_reference_plot() {
    if (cigar_percentage_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const match_data = get_dataset(name,"cigar_percentage_per_reference_match");
            const insertion_data = get_dataset(name,"cigar_percentage_per_reference_insertion");
            const deletion_data = get_dataset(name,"cigar_percentage_per_reference_deletion");
            const skip_data = get_dataset(name,"cigar_percentage_per_reference_skip");

            let match_dataset = {
                label: name + " Alignment Matches",
                data: match_data,
                backgroundColor: color[1],
                stack: "" + i
            };

            let insertion_dataset = {
                label: name + " Insertions",
                data: insertion_data,
                backgroundColor: color[2],
                stack: "" + i
            }

            let deletion_dataset = {
                label: name + " Deletions",
                data: deletion_data,
                backgroundColor: color[3],
                stack: "" + i
            }
            let skip_dataset = {
                label: name + " Skips",
                data: skip_data,
                backgroundColor: color[4],
                stack: "" + i
            }

            plot_data.datasets.push(match_dataset);
            plot_data.datasets.push(insertion_dataset);
            plot_data.datasets.push(deletion_dataset);
            plot_data.datasets.push(skip_dataset);
        }

        cigar_percentage_per_reference_plot.config.data = plot_data;
        cigar_percentage_per_reference_plot.update();
    }
}

let cigar_total_per_bin_plot;
const cigar_total_per_bin_logarithmic = document.getElementById("cigar_total_per_bin_logarithmic");

function setup_cigar_total_per_bin_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations per Bin (Total Count)"
                },
                tooltips: {
                    enabled: true,
                    mode: "index",
                    intersect: false,
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
                intersect: false,
                axis: "x"
            },
            animation: false,
            line: {
                cubicInterpolationMode: "monotone"
            }
        }
    };

    cigar_total_per_bin_plot = new Chart(
        document.getElementById('cigar_total_per_bin_canvas'),
        config
    );

    cigar_total_per_bin_logarithmic.addEventListener("change", () => update_cigar_total_per_bin_plot());

}

function update_cigar_total_per_bin_plot() {
    if (cigar_total_per_bin_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        };

        let file_names = get_file_list();


        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const reference_name = selected_reference.value;

            let bin_size = get_dataset(name, "bin_size")[0];

            let match_line_data = bin_data_to_line_data(get_dataset(name, reference_name + "_cigar_total_per_bin_match"));
            let insertion_line_data = bin_data_to_line_data(get_dataset(name, reference_name + "_cigar_total_per_bin_insertion"));
            let deletion_line_data = bin_data_to_line_data(get_dataset(name, reference_name + "_cigar_total_per_bin_deletion"));
            let skip_line_data = bin_data_to_line_data(get_dataset(name, reference_name + "_cigar_total_per_bin_skip"));

            let decimated_match_data = decimate_bin_data_mean(match_line_data, 1000, 1000);
            let decimated_insertion_data = decimate_bin_data_mean(insertion_line_data, 1000, 1000);
            let decimated_deletion_data = decimate_bin_data_mean(deletion_line_data, 1000, 1000);
            let decimated_skip_data = decimate_bin_data_mean(skip_line_data, 1000, 1000);

            let decimation_size = calculate_decimation_size(match_line_data, 1000, 1000);
            plot_data.labels = generate_labels(decimated_match_data, bin_size * decimation_size);


            let match_dataset = {
                label: name + " Alignment Matches",
                data: decimated_match_data,
                backgroundColor: color[1],
                borderColor: color[1],
                borderWidth: 1,
                stack: "" + i,
                radius: 0,
                fill: false,
            };

            let insertion_dataset = {
                label: name + " Insertions",
                data: decimated_insertion_data,
                backgroundColor: color[2],
                borderColor: color[2],
                borderWidth: 1,
                stack: "" + i,
                radius: 0,
                fill: false,
            }

            let deletion_dataset = {
                label: name + " Deletions",
                data: decimated_deletion_data,
                backgroundColor: color[3],
                borderColor: color[3],
                borderWidth: 1,
                stack: "" + i,
                radius: 0,
                fill: false,
            }
            let skip_dataset = {
                label: name + " Skips",
                data: decimated_skip_data,
                backgroundColor: color[4],
                borderColor: color[4],
                borderWidth: 1,
                stack: "" + i,
                radius: 0,
                fill: false,
            }

            plot_data.datasets.push(match_dataset);
            plot_data.datasets.push(insertion_dataset);
            plot_data.datasets.push(deletion_dataset);
            plot_data.datasets.push(skip_dataset);
        }

        const logarithmic = cigar_total_per_bin_logarithmic.checked;

        if (logarithmic) {
            cigar_total_per_bin_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            cigar_total_per_bin_plot.config.options.scales.y.type=undefined;
        }

        cigar_total_per_bin_plot.config.data = plot_data;
        cigar_total_per_bin_plot.update();
    }
}

let cigar_percentage_per_bin_plot;

function setup_cigar_percentage_per_bin_plot() {
    let data = {
        labels: reference_names,
        datasets: []
    };

    let config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations per Bin (Percentage)"
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
                    },
                    mode: "index",
                    intersect: false,
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
                    stacked: false,
                    ticks: {
                        callback: function(value){return value+"%"}
                    }
                }
            },
            interaction: {
                mode: 'index',
                intersect: false
            },
            animation: false,
            line: {
                cubicInterpolationMode: "monotone"
            }
        }
    };

    cigar_percentage_per_bin_plot = new Chart(
        document.getElementById('cigar_percentage_per_bin_canvas'),
        config
    );
}

function update_cigar_percentage_per_bin_plot() {
    if (cigar_percentage_per_bin_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        };

        let file_names = get_file_list();

        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const reference_name = selected_reference.value;

            let bin_size = get_dataset(name, "bin_size")[0];

            let match_data = get_dataset(name, reference_name + "_cigar_percentage_per_bin_match");
            let insertion_data = get_dataset(name, reference_name + "_cigar_percentage_per_bin_insertion");
            let deletion_data = get_dataset(name, reference_name + "_cigar_percentage_per_bin_deletion");
            let skip_data = get_dataset(name, reference_name + "_cigar_percentage_per_bin_skip");

            let match_line_data = bin_data_to_line_data(match_data);
            let insertion_line_data = bin_data_to_line_data(insertion_data);
            let deletion_line_data = bin_data_to_line_data(deletion_data);
            let skip_line_data = bin_data_to_line_data(skip_data);

            let decimated_match_data = decimate_bin_data_mean(match_line_data, 1000, 1000);
            let decimated_insertion_data = decimate_bin_data_mean(insertion_line_data, 1000, 1000);
            let decimated_deletion_data = decimate_bin_data_mean(deletion_line_data, 1000, 1000);
            let decimated_skip_data = decimate_bin_data_mean(skip_line_data, 1000, 1000);

            let decimation_size = calculate_decimation_size(match_line_data, 1000, 1000);

            plot_data.labels = generate_labels(decimated_match_data, bin_size * decimation_size);

            let match_dataset = {
                label: name + " Alignment Matches",
                data: decimated_match_data,
                backgroundColor: color[1],
                borderColor: color[1],
                borderWidth: 1,
                stack: "" + i,
                radius: 1,
                fill: false,
            };

            let insertion_dataset = {
                label: name + " Insertions",
                data: decimated_insertion_data,
                backgroundColor: color[2],
                borderColor: color[2],
                borderWidth: 1,
                stack: "" + i,
                radius: 1,
                fill: false,
            }

            let deletion_dataset = {
                label: name + " Deletions",
                data: decimated_deletion_data,
                backgroundColor: color[3],
                borderColor: color[3],
                borderWidth: 1,
                stack: "" + i,
                radius: 1,
                fill: false,
            }
            let skip_dataset = {
                label: name + " Skips",
                data: decimated_skip_data,
                backgroundColor: color[4],
                borderColor: color[4],
                borderWidth: 1,
                stack: "" + i,
                radius: 1,
                fill: false,
            }

            plot_data.datasets.push(match_dataset);
            plot_data.datasets.push(insertion_dataset);
            plot_data.datasets.push(deletion_dataset);
            plot_data.datasets.push(skip_dataset);
        }

        cigar_percentage_per_bin_plot.config.data = plot_data;
        cigar_percentage_per_bin_plot.update();
    }
}
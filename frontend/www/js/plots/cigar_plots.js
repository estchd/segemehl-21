import {get_reference_list, get_dataset, get_file_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";

export function setup_cigar_plots() {
    setup_cigar_total_file_plot();
    setup_cigar_total_per_reference_plot();
    setup_cigar_total_per_bin_plot();
    setup_cigar_percentage_file_plot();
    setup_cigar_percentage_per_reference_plot();
    setup_cigar_percentage_per_bin_plot();
}

export function update_cigar_plots() {
    reference_names = get_reference_list();

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
            colors.shift();

            const data = get_dataset(name,"cigar_total_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: colors
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
            let  color = file_info[1];
            color.shift();

            const data = get_dataset(name,"cigar_percentage_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
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
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
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
                    }
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
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
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
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Cigar Operations per Bin (Total Count)"
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

    cigar_total_per_bin_plot = new Chart(
        document.getElementById('cigar_total_per_bin_canvas'),
        config
    );

    cigar_total_per_bin_logarithmic.addEventListener("change", () => update_cigar_total_per_bin_plot());

}

function update_cigar_total_per_bin_plot() {
    if (cigar_total_per_bin_plot) {
        let plot_data = {
            datasets: []
        };

        let file_names = get_file_list();

        let max_bin_count = 0;

        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const reference_name = selected_reference.value;

            const match_data = get_dataset(name,reference_name + "_cigar_total_per_bin_match");
            const insertion_data = get_dataset(name,reference_name + "_cigar_total_per_bin_insertion");
            const deletion_data = get_dataset(name,reference_name + "_cigar_total_per_bin_deletion");
            const skip_data = get_dataset(name,reference_name + "_cigar_total_per_bin_skip");

            max_bin_count = Math.max(match_data.length, max_bin_count);

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

        const logarithmic = cigar_total_per_bin_logarithmic.checked;

        if (logarithmic) {
            cigar_total_per_bin_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            cigar_total_per_bin_plot.config.options.scales.y.type=undefined;
        }

        let labels = [];

        for (let i = 0; i <= max_bin_count; i++) {
            labels.push(i)
        }

        plot_data.labels = labels;

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

    cigar_percentage_per_bin_plot = new Chart(
        document.getElementById('cigar_percentage_per_bin_canvas'),
        config
    );
}

function update_cigar_percentage_per_bin_plot() {
    if (cigar_percentage_per_bin_plot) {
        let plot_data = {
            datasets: []
        };

        let file_names = get_file_list();

        let max_bin_count = 0;

        for (let i = 0; i < file_names.length; i++) {
            let file_info = file_names[i];

            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1];

            const reference_name = selected_reference.value;

            const match_data = get_dataset(name,reference_name + "_cigar_percentage_per_bin_match");
            const insertion_data = get_dataset(name,reference_name + "_cigar_percentage_per_bin_insertion");
            const deletion_data = get_dataset(name,reference_name + "_cigar_percentage_per_bin_deletion");
            const skip_data = get_dataset(name,reference_name + "_cigar_percentage_per_bin_skip");

            max_bin_count = Math.max(match_data.length, max_bin_count);

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

        let labels = [];

        for (let i = 0; i <= max_bin_count; i++) {
            labels.push(i)
        }

        plot_data.labels = labels;

        cigar_percentage_per_bin_plot.config.data = plot_data;
        cigar_percentage_per_bin_plot.update();
    }
}
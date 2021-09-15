import {get_reference_list, get_dataset, get_file_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";

export function setup_coverage_plots() {
    setup_read_counts_per_reference_plot();
    setup_read_counts_per_bin_on_reference_plot();
    setup_total_read_length_per_reference_plot();
    setup_total_read_length_per_bin_on_reference_plot();
    setup_coverage_per_reference_plot();
    setup_coverage_per_bin_on_reference_plot();
}

export function update_coverage_plots() {
    reference_names = get_reference_list();

    update_read_counts_per_reference_plot();
    update_total_read_length_per_reference_plot();
    update_coverage_per_reference_plot();
}

export function update_reference_dependent_coverage_plots() {
    update_read_counts_per_bin_on_reference_plot();
    update_total_read_length_per_bin_on_reference_plot();
    update_coverage_per_bin_on_reference_plot();
}

const selected_reference = document.getElementById("chromosome-select");

let reference_names = [];

let read_counts_per_reference_plot;
let read_counts_per_reference_logarithmic = document.getElementById("read_counts_per_reference_logarithmic");

function setup_read_counts_per_reference_plot() {
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
                    text: "Read Counts per Reference"
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

    read_counts_per_reference_plot = new Chart(
        document.getElementById('read_counts_per_reference_canvas'),
        config
    );

    read_counts_per_reference_logarithmic.addEventListener("change", () => update_read_counts_per_reference_plot());
}

function update_read_counts_per_reference_plot() {
    if (read_counts_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"read_counts_per_reference");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = read_counts_per_reference_logarithmic.checked;

        if (logarithmic) {
            read_counts_per_reference_plot.config.options.scales.y.type="logarithmic";
            read_counts_per_reference_plot.config.options.scales.y.stacked=false;
            read_counts_per_reference_plot.config.options.scales.x.stacked=false;
        }
        else {
            read_counts_per_reference_plot.config.options.scales.y.type=undefined;
            read_counts_per_reference_plot.config.options.scales.y.stacked=true;
            read_counts_per_reference_plot.config.options.scales.x.stacked=true;
        }

        read_counts_per_reference_plot.config.data = plot_data;
        read_counts_per_reference_plot.update();
    }
}

let read_counts_per_bin_on_reference_plot;
let read_counts_per_bin_on_reference_logarithmic = document.getElementById("read_counts_per_bin_on_reference_logarithmic");

function setup_read_counts_per_bin_on_reference_plot() {
    let data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Read Counts per Bin on selected Chromosome"
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
            animation: false
        }
    };

    read_counts_per_bin_on_reference_plot = new Chart(
        document.getElementById('read_counts_per_bin_on_reference_canvas'),
        config
    );
}

function update_read_counts_per_bin_on_reference_plot() {
    if (read_counts_per_bin_on_reference_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        };

        let file_names = get_file_list();

        let reference = selected_reference.value;

        let max_bin_count = 0;

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const dataset_name = reference + "_read_counts_per_bin";

            let data = get_dataset(name, dataset_name);

            max_bin_count = Math.max(max_bin_count, data.length);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        let labels = [];

        for (let i = 0; i < max_bin_count; i++) {
            labels.push(i);
        }

        const logarithmic = read_counts_per_bin_on_reference_logarithmic.checked;

        if (logarithmic) {
            read_counts_per_bin_on_reference_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            read_counts_per_bin_on_reference_plot.config.options.scales.y.type=undefined;
        }

        plot_data.labels = labels;
        read_counts_per_bin_on_reference_plot.config.data = plot_data;
        read_counts_per_bin_on_reference_plot.update();
    }
}

let total_read_length_per_reference_plot;
let total_read_length_per_reference_logarithmic = document.getElementById("total_read_length_reference_logarithmic");

function setup_total_read_length_per_reference_plot() {
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
                    text: "Total Read Length per Reference"
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

    total_read_length_per_reference_plot = new Chart(
        document.getElementById('total_read_length_per_reference_canvas'),
        config
    );

    total_read_length_per_reference_logarithmic.addEventListener("change", () => update_total_read_length_per_bin_on_reference_plot());
}

function update_total_read_length_per_reference_plot() {
    if (total_read_length_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"total_read_length_per_reference");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = total_read_length_per_reference_logarithmic.checked;

        if (logarithmic) {
            total_read_length_per_reference_plot.config.options.scales.y.type="logarithmic";
            total_read_length_per_reference_plot.config.options.scales.y.stacked=false;
            total_read_length_per_reference_plot.config.options.scales.x.stacked=false;
        }
        else {
            total_read_length_per_reference_plot.config.options.scales.y.type=undefined;
            total_read_length_per_reference_plot.config.options.scales.y.stacked=true;
            total_read_length_per_reference_plot.config.options.scales.x.stacked=true;
        }

        total_read_length_per_reference_plot.config.data = plot_data;
        total_read_length_per_reference_plot.update();
    }
}

let total_read_length_per_bin_on_reference_plot;
let total_read_length_per_bin_on_reference_logarithmic = document.getElementById("total_read_length_per_bin_on_reference_logarithmic");

function setup_total_read_length_per_bin_on_reference_plot() {
    let data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Total Read Length per Bin on selected Chromosome"
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
            animation: false
        }
    };

    total_read_length_per_bin_on_reference_plot = new Chart(
        document.getElementById('total_read_length_per_bin_canvas'),
        config
    );

    total_read_length_per_bin_on_reference_logarithmic.addEventListener("change", () => update_total_read_length_per_bin_on_reference_plot());
}

function update_total_read_length_per_bin_on_reference_plot() {
    if (total_read_length_per_bin_on_reference_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        };

        let file_names = get_file_list();

        let reference = selected_reference.value;

        let max_bin_count = 0;

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const dataset_name = reference + "_total_read_length_per_bin";

            let data = get_dataset(name, dataset_name);

            max_bin_count = Math.max(max_bin_count, data.length);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        let labels = [];

        for (let i = 0; i < max_bin_count; i++) {
            labels.push(i);
        }

        const logarithmic = total_read_length_per_bin_on_reference_logarithmic.checked;

        if (logarithmic) {
            total_read_length_per_bin_on_reference_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            total_read_length_per_bin_on_reference_plot.config.options.scales.y.type=undefined;
        }

        plot_data.labels = labels;
        total_read_length_per_bin_on_reference_plot.config.data = plot_data;
        total_read_length_per_bin_on_reference_plot.update();
    }
}

let coverage_per_reference_plot;

function setup_coverage_per_reference_plot() {
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
                    text: "Coverage per Reference"
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
                    stacked: false
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
            onClick: function (_, elements) {
                let element = elements[0];

                if (element) {
                    linking_update_selected_reference(element.index);
                }
            }
        }
    };

    coverage_per_reference_plot = new Chart(
        document.getElementById('coverage_per_reference_canvas'),
        config
    );
}

function update_coverage_per_reference_plot() {
    if (coverage_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"coverage_per_reference");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        coverage_per_reference_plot.config.data = plot_data;
        coverage_per_reference_plot.update();
    }
}

let coverage_per_bin_on_reference_plot;

function setup_coverage_per_bin_on_reference_plot() {
    let data = {
        labels: [],
        datasets: []
    };

    let config = {
        type: 'line',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Coverage per Bin on selected Chromosome"
                },
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
                    stacked: false,
                    ticks: {
                        callback: function(value){return value+"%"}
                    }
                }
            },
            animation: false
        }
    };

    coverage_per_bin_on_reference_plot = new Chart(
        document.getElementById('coverage_per_bin_canvas'),
        config
    );
}

function update_coverage_per_bin_on_reference_plot() {
    if (coverage_per_bin_on_reference_plot) {
        let plot_data = {
            labels: [],
            datasets: []
        };

        let file_names = get_file_list();

        let reference = selected_reference.value;

        let max_bin_count = 0;

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const dataset_name = reference + "_coverage_per_bin";

            let data = get_dataset(name, dataset_name);

            max_bin_count = Math.max(max_bin_count, data.length);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        let labels = [];

        for (let i = 0; i < max_bin_count; i++) {
            labels.push(i);
        }

        plot_data.labels = labels;
        coverage_per_bin_on_reference_plot.config.data = plot_data;
        coverage_per_bin_on_reference_plot.update();
    }
}
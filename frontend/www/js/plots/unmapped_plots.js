import {get_dataset, get_file_list, get_reference_list} from "../wasm_binding";

export function setup_unmapped_plots() {
    setup_unmapped_read_count_plot();
    setup_unmapped_read_percentage_plot();
    setup_unmapped_read_length_plot();
}

export function update_unmapped_plots() {
    reference_names = get_reference_list();

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
        labels: ["Unmapped Read Count"],
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

    unmapped_read_count_plot = new Chart(
        document.getElementById('unmapped_read_count_canvas'),
        config
    );
}

function update_unmapped_read_count_plot() {
    if (unmapped_read_count_plot) {
        let plot_data = {
            labels: ["Unmapped Read Count"],
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
        labels: ["Unmapped Read Percentage"],
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

    unmapped_read_percentage_plot = new Chart(
        document.getElementById('unmapped_read_percentage_canvas'),
        config
    );
}

function update_unmapped_read_percentage_plot() {
    if (unmapped_read_percentage_plot) {
        let plot_data = {
            labels: ["Unmapped Read Percentage"],
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
        labels: ["Mean","Mode","Median","Shortest","Longest"],
        datasets: []
    };

    let config = {
        type: 'bar',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Unmapped Read Length"
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

    unmapped_read_length_plot = new Chart(
        document.getElementById('unmapped_read_length_canvas'),
        config
    );
}

function update_unmapped_read_length_plot() {
    if (unmapped_read_length_plot) {
        let plot_data = {
            labels: ["Mean","Mode","Median","Shortest","Longest"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const data = get_dataset(name,"unmapped_read_length");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        unmapped_read_length_plot.config.data = plot_data;
        unmapped_read_length_plot.update();
    }
}
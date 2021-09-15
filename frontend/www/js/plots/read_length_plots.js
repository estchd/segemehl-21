import {get_dataset, get_file_list, get_reference_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";

export function setup_read_length_plots() {
    setup_read_length_sequence_file();
    setup_read_length_sequence_per_reference();
    setup_read_length_reference_file();
    setup_read_length_on_reference_per_reference();
}

export function update_read_length_plots() {
    reference_names = get_reference_list();

    update_read_length_sequence_file();
    update_read_length_sequence_per_reference();
    update_read_length_reference_file();
    update_read_length_on_reference_per_reference();
}

export function update_reference_dependent_read_length_plots() {
}

let reference_names = [];

let read_length_sequence_file_plot;

function setup_read_length_sequence_file() {
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
                    text: "Length of a Read (Read Sequence) per File"
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

    read_length_sequence_file_plot = new Chart(
        document.getElementById('read_length_sequence_file_canvas'),
        config
    );
}

function update_read_length_sequence_file() {
    if (read_length_sequence_file_plot) {
        let plot_data = {
            labels: ["Mean","Mode","Median","Shortest","Longest"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            let colors = file_info[1][0];

            const data = get_dataset(name,"read_length_sequence_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: colors
            };

            plot_data.datasets.push(dataset);
        }

        read_length_sequence_file_plot.config.data = plot_data;
        read_length_sequence_file_plot.update();
    }
}

let read_length_reference_file_plot;

function setup_read_length_reference_file() {
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
                    text: "Length of a Read (On the Reference) per File"
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

    read_length_reference_file_plot = new Chart(
        document.getElementById('read_length_reference_file_canvas'),
        config
    );
}

function update_read_length_reference_file() {
    if (read_length_reference_file_plot) {
        let plot_data = {
            labels: ["Mean","Mode","Median","Shortest","Longest"],
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            let colors = file_info[1][0];

            const data = get_dataset(name,"read_length_reference_file");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: colors
            };

            plot_data.datasets.push(dataset);
        }

        read_length_reference_file_plot.config.data = plot_data;
        read_length_reference_file_plot.update();
    }
}

let read_length_sequence_per_reference_plot;

const read_length_sequence_per_reference_stat = document.getElementById("read_length_sequence_per_reference_stat");

function setup_read_length_sequence_per_reference() {
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
                    text: "Length of a Read (Read Sequence) per Reference"
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

    read_length_sequence_per_reference_stat.addEventListener("change", () => update_read_length_sequence_per_reference());

    read_length_sequence_per_reference_plot = new Chart(
        document.getElementById('read_length_sequence_per_reference_canvas'),
        config
    );
}

function update_read_length_sequence_per_reference() {
    if (read_length_sequence_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const stat = read_length_sequence_per_reference_stat.value;

            const data = get_dataset(name,"read_length_sequence_per_reference_" + stat);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        read_length_sequence_per_reference_plot.config.data = plot_data;
        read_length_sequence_per_reference_plot.update();
    }
}

let read_length_reference_per_reference_plot;

const read_length_reference_per_reference_stat = document.getElementById("read_length_reference_per_reference_stat");

function setup_read_length_on_reference_per_reference() {
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
                    text: "Length of a Read (On the Reference) per Reference"
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

    read_length_reference_per_reference_stat.addEventListener("change", () => update_read_length_on_reference_per_reference());

    read_length_reference_per_reference_plot = new Chart(
        document.getElementById('read_length_reference_per_reference_canvas'),
        config
    );
}

function update_read_length_on_reference_per_reference() {
    if (read_length_reference_per_reference_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            const color = file_info[1][0];

            const stat = read_length_reference_per_reference_stat.value;

            const data = get_dataset(name,"read_length_reference_per_reference_" + stat);

            let dataset = {
                label: name,
                data: data,
                backgroundColor: color
            };

            plot_data.datasets.push(dataset);
        }

        read_length_reference_per_reference_plot.config.data = plot_data;
        read_length_reference_per_reference_plot.update();
    }
}
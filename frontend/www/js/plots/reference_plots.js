import {get_dataset, get_file_list, get_reference_list} from "../wasm_binding";
import {linking_update_selected_reference} from "../plots";

export function setup_reference_plots() {
    setup_reference_length_plot();
}

export function update_reference_plots() {
    reference_names = get_reference_list();

    update_reference_length_plot();
}

export function update_reference_dependent_reference_plots() {

}

let reference_names = [];

let reference_length_plot;
const reference_length_logarithmic = document.getElementById("reference_length_logarithmic");

function setup_reference_length_plot() {
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
                    text: "Reference Length"
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
                    stacked: false
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

    reference_length_plot = new Chart(
        document.getElementById('reference_length_canvas'),
        config
    );
}

function update_reference_length_plot() {
    if (reference_length_plot) {
        let plot_data = {
            labels: reference_names,
            datasets: []
        };

        let file_names = get_file_list();

        for (const file_info of file_names) {
            if (!file_info[2]) {continue;}

            const name = file_info[0];
            let colors = file_info[1][0];

            const data = get_dataset(name,"reference_length");

            let dataset = {
                label: name,
                data: data,
                backgroundColor: colors
            };

            plot_data.datasets.push(dataset);
        }

        const logarithmic = reference_length_logarithmic.checked;

        if (logarithmic) {
            reference_length_plot.config.options.scales.y.type="logarithmic";
        }
        else {
            reference_length_plot.config.options.scales.y.type=undefined;
        }

        reference_length_plot.config.data = plot_data;
        reference_length_plot.update();
    }
}
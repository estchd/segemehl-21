import "chartjs-chart-box-and-violin-plot"
import {
    boxplot_tooltip,
    calculate_boxplot_from_histogram,
    calculate_violin_from_histogram,
    histogram_from_values
} from "./box_plot";

export function setup_test_plots() {
    setup_test_box_numbers_item_plot();
    setup_test_box_box_plot_item_plot();
    setup_test_violin_numbers_item_plot();
    setup_test_violin_violin_item_plot();
}

export function update_test_plot() {
    update_test_box_numbers_item_plot();
    update_test_box_box_plot_item_plot();
    update_test_violin_numbers_item_plot();
    update_test_violin_violin_item_plot();
}

export function update_reference_dependent_test_plots() {

}

let test_box_numbers_item_plot;

function setup_test_box_numbers_item_plot() {
    let data = {
        labels: ["test1", "test2", "test3"],
        datasets: [
            {
                label: 'Dataset 1',
                backgroundColor: 'rgba(255,0,0,0.5)',
                borderColor: 'red',
                borderWidth: 1,
                outlierColor: '#999999',
                padding: 10,
                itemRadius: 0,
                data: [
                    [0,1,1,2,2,2,2,3,3,3,4,4,4,4,4,5,5,6,6,6,6,8,9,9,9,9,9,9,10,10],
                    [0,0,0,1,1,1,1,2,2,2,2,3,3,4,6,6,7,7,7,7,7,8,9,9,9,9,9,9,10,10],
                    [0,1,1,1,1,1,1,2,2,3,3,4,4,6,6,6,7,7,7,7,8,8,8,8,8,9,9,9,10,10]
                ]
            }
        ]
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Test Box Plot with Number Items"
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
            tooltips: {
                callbacks: {
                    boxplotLabel: function(item, data, stats, hoveredOutlierIndex) {
                        console.log(item);
                        console.log(data);
                        console.log(stats);
                        console.log(hoveredOutlierIndex);
                        return "custom tooltip";
                    }
                }
            }
        }
    };

    test_box_numbers_item_plot = new Chart(
        document.getElementById('test_box_number_item_canvas'),
        config
    );
}

function update_test_box_numbers_item_plot() {
    test_box_numbers_item_plot.update();
}

let test_box_box_plot_item_plot;

function setup_test_box_box_plot_item_plot() {
    let data = {
        labels: ["test1", "test2", "test3"],
        datasets: [
            {
                label: 'Dataset 1',
                backgroundColor: 'rgba(255,0,0,0.5)',
                borderColor: 'red',
                borderWidth: 1,
                outlierColor: '#999999',
                padding: 10,
                itemRadius: 1,
                meanRadius: 1,
                meanBackgroundColor: "#555555",
                meanStyle: "circle",
                itemStyle: "circle",
                outlierStyle: "star",
                outlierRadius: 1,
                data: [
                    {
                        min: 1,
                        q1: 1,
                        median: 1,
                        mode: 1,
                        mean: 1,
                        q3: 1,
                        max: 1
                    },
                    calculate_boxplot_from_histogram(
                        histogram_from_values(
                            [0,1,1,2,2,2,2,3,3,3,4,4,4,4,4,5,5,6,6,6,6,8,9,9,9,9,9,9,10,10]
                        )
                    ),
                    calculate_boxplot_from_histogram(
                        histogram_from_values(
                            [0,0,0,1,1,1,1,2,2,2,2,3,3,4,6,6,7,7,7,7,7,8,9,9,9,9,9,9,10,10]
                        )
                    ),
                    calculate_boxplot_from_histogram(
                        histogram_from_values(
                            [0,1,1,1,1,1,1,2,2,3,3,4,4,6,6,6,7,7,7,7,8,8,8,8,8,9,9,9,10,10]
                        )
                    )
                ]
            }
        ]
    };

    let config = {
        type: 'boxplot',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Test Box Plot with Box Plot Items"
                },
            },
            locale: "de-DE",
            responsive: true,
            maintainAspectRatio: true,
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

    test_box_box_plot_item_plot = new Chart(
        document.getElementById('test_box_box_plot_item_canvas'),
        config
    );
}

function update_test_box_box_plot_item_plot() {
    test_box_box_plot_item_plot.update();
}

let test_violin_numbers_item_plot;

function setup_test_violin_numbers_item_plot() {
    let data = {
        labels: ["test1", "test2", "test3"],
        datasets: [
            {
                label: 'Dataset 1',
                backgroundColor: 'rgba(255,0,0,0.5)',
                borderColor: 'red',
                borderWidth: 1,
                outlierColor: '#999999',
                padding: 10,
                itemRadius: 0,
                data: [
                    [0,1,1,2,2,2,2,3,3,3,4,4,4,4,4,5,5,6,6,6,6,8,9,9,9,9,9,9,10,10],
                    [0,0,0,1,1,1,1,2,2,2,2,3,3,4,6,6,7,7,7,7,7,8,9,9,9,9,9,9,10,10],
                    [0,1,1,1,1,1,1,2,2,3,3,4,4,6,6,6,7,7,7,7,8,8,8,8,8,9,9,9,10,10]
                ]
            }
        ]
    };

    let config = {
        type: 'violin',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Test Violin Plot with Number Items"
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
            animation: false
        }
    };

    test_violin_numbers_item_plot = new Chart(
        document.getElementById('test_violin_number_item_canvas'),
        config
    );
}

function update_test_violin_numbers_item_plot() {
    test_violin_numbers_item_plot.update();
}

let test_violin_violin_item_plot;

function setup_test_violin_violin_item_plot() {
    let data = {
        labels: ["test1", "test2", "test3"],
        datasets: [
            {
                label: 'Dataset 1',
                backgroundColor: 'rgba(255,0,0,0.5)',
                borderColor: 'red',
                borderWidth: 1,
                outlierColor: '#999999',
                padding: 10,
                itemRadius: 0,
                data: [
                    calculate_violin_from_histogram(
                        histogram_from_values(
                            [0,1,1,2,2,2,2,3,3,3,4,4,4,4,4,5,5,6,6,6,6,8,9,9,9,9,9,9,10,10]
                        )
                    ),
                    calculate_violin_from_histogram(
                        histogram_from_values(
                            [0,0,0,1,1,1,1,2,2,2,2,3,3,4,6,6,7,7,7,7,7,8,9,9,9,9,9,9,10,10]
                        )
                    ),
                    calculate_violin_from_histogram(
                        histogram_from_values(
                            [0,1,1,1,1,1,1,2,2,3,3,4,4,6,6,6,7,7,7,7,8,8,8,8,8,9,9,9,10,10]
                        )
                    )
                ]
            }
        ]
    };

    let config = {
        type: 'violin',
        data: data,
        options: {
            plugins: {
                title: {
                    display: false,
                    text: "Test Violin Plot with Violin Items"
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
            animation: false
        }
    };

    test_violin_violin_item_plot = new Chart(
        document.getElementById('test_violin_violin_item_canvas'),
        config
    );
}

function update_test_violin_violin_item_plot() {
    test_violin_violin_item_plot.update();
}


import "chartjs-chart-box-and-violin-plot"

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
                label: "Dataset 1",
                outlierColor: '#999999',
                backgroundColor: "rgba(255,0,0,0.5)",
                borderColor: "red",
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
            animation: false
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
                label: "Dataset 1",
                outlierColor: '#999999',
                backgroundColor: "rgba(255,0,0,0.5)",
                borderColor: "red",
                data: [
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
                label: "Dataset 1",
                data: [
                    [0,1,1,2,2,2,2,3,3,3,4,4,4,4,4,5,5,6,6,6,6,8,9,9,9,9,9,9,10,10],
                    [0,0,0,1,1,1,1,2,2,2,2,3,3,4,6,6,7,7,7,7,7,8,9,9,9,9,9,9,10,10],
                    [0,1,1,1,1,1,1,2,2,3,3,4,4,6,6,6,7,7,7,7,8,8,8,8,8,9,9,9,10,10]
                ],
                outlierColor: '#999999',
                backgroundColor: "rgba(255,0,0,0.5)",
                borderColor: "red"
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
                label: "Dataset 1",
                outlierColor: '#999999',
                backgroundColor: "rgba(255,0,0,0.5)",
                borderColor: "red",
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

function histogram_from_values(values) {
    let map = values.reduce(function (prev, cur) {
        if (prev.has(cur)) {
            prev.set(cur, prev.get(cur) + 1)
        }
        else {
            prev.set(cur, 1);
        }
        return prev;
    }, new Map());
    let histogram = [];
    map.forEach((value, key) => {
        histogram.push({
            value: key,
            amount: value
        })
    });
    return histogram;
}

function combine_histogram(values, amounts) {
    let histogram = [];
    for (let i = 0; i < values.length; i++) {
        if (i >= amounts.length) {
            return histogram;
        }

        histogram.push({
            value: values[i],
            amount: amounts[i],
        });
    }

    return histogram;
}

function calculate_violin_from_histogram(histogram) {
    let sum_histogram_amounts = histogram.reduce((prev, curr) => {
        return prev + curr.amount;
    }, 0);

    histogram = histogram.map(element => {
        return {
            value: element.value,
            amount: element.amount / sum_histogram_amounts
        }
    });

    let coords = [];

    histogram.forEach((item) => {
        coords.push({
            v: item.value,
            estimate: item.amount
        })
    })

    return {
        coords: coords
    };
}

function calculate_boxplot_from_histogram(histogram) {
    histogram.sort((a, b) => {
        return a.value - b.value;
    });

    let min = histogram[0].value;
    let max = histogram[histogram.length - 1].value;

    let sum_histogram_amounts = histogram.reduce((prev, curr) => {
        return prev + curr.amount;
    }, 0);

    let quartile_positions = calculate_quartile_positions(sum_histogram_amounts);

    let q1_calculated = calculate_quartile(histogram, 0, null, quartile_positions.q1);

    quartile_positions.q2 -= q1_calculated.covered_length;
    quartile_positions.q3 -= q1_calculated.covered_length;

    let q2_calculated = calculate_quartile(histogram, q1_calculated.new_index, q1_calculated.new_last, quartile_positions.q2);

    quartile_positions.q3 -= q2_calculated.covered_length;

    let q3_calculated = calculate_quartile(histogram, q2_calculated.new_index, q2_calculated.new_last, quartile_positions.q3);

    return {
        min: min,
        q1: q1_calculated.quartile,
        median: q2_calculated.quartile,
        q3: q3_calculated.quartile,
        max: max
    }
}

function calculate_quartile(histogram, index, last, quartile_position) {
    let quartile;
    let covered_length = 0;

    while (index < histogram.length) {
        let histogram_value = histogram[index];

        if (quartile_position < 1) {
            if (last) {
                quartile = (histogram_value.value + last) / 2;
            } else {
                quartile = (histogram_value.value);
            }
            break;
        }

        if (histogram_value.amount >= quartile_position) {
            histogram_value.amount -= quartile_position;

            quartile = histogram_value.value;
            last = histogram_value.value;
            covered_length += quartile_position;
            break;
        }

        quartile_position -= histogram_value.amount;
        covered_length += histogram_value.amount;
        last = histogram_value.value;

        index++;
    }

    return {
        quartile: quartile,
        new_index: index,
        new_last: last,
        covered_length: covered_length
    }
}

function calculate_quartile_positions(count) {
    let half_length = Math.floor(count / 2);

    let q1 = (half_length / 2) + 0.5;
    let q2 = (count / 2) + 0.5;
    let q3 = count - (q1 - 1);

    return {
        q1: q1,
        q2: q2,
        q3: q3
    }
}

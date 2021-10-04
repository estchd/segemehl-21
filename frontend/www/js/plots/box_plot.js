export function boxplot_tooltip(item, data, stats, hoveredOutlierIndex) {
    return "" + data.datasets[item.datasetIndex].label +
        " " + item.label +
        " (min: " + new Intl.NumberFormat('en-US').format(stats.min) +
        ", q1: " + new Intl.NumberFormat('en-US').format(stats.q1) +
        ", median: " + new Intl.NumberFormat('en-US').format(stats.median) +
        ", mean: " + new Intl.NumberFormat('en-US').format(stats.mean) +
        ", mode: " + new Intl.NumberFormat('en-US').format(stats.mode) +
        ", q3: " + new Intl.NumberFormat('en-US').format(stats.q3) +
        ", max: " + new Intl.NumberFormat('en-US').format(stats.max) +
        ")";
}

export function histogram_from_values(values) {
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

export function combine_histogram(values, amounts) {
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

export function calculate_violin_from_histogram(histogram) {
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
    });

    return {
        coords: coords
    };
}

export function boxplot_from_separate_arrays(min, q1, median, mean, mode, q3, max) {
    let boxplot = [];

    for (let i = 0; i < min.length; i++) {
        if (
            i >= q1.length ||
            i >= median.length ||
            i >= mean.length ||
            i >= mode.length ||
            i >= q3.length ||
            i >= max.length
        ) {
            return boxplot;
        }

        boxplot.push({
            min: min[i],
            q1: q1[i],
            median: median[i],
            mean: mean[i],
            mode: mode[i],
            q3: q3[i],
            max: max[i]
        });
    }
    return boxplot;
}

export function calculate_boxplot_from_histogram(histogram) {
    histogram.sort((a, b) => {
        return a.value - b.value;
    });

    let min = histogram[0].value;
    let max = histogram[histogram.length - 1].value;

    let sum_histogram_amounts = histogram.reduce((prev, curr) => {
        return prev + curr.amount;
    }, 0);

    let multiplied_sum = histogram.reduce((prev,curr) => {
        return prev + (curr.amount * curr.value);
    }, 0);

    let mode = histogram.reduce((prev, curr) => {
        if (prev.amount > curr.amount) {
            return prev;
        }
        else {
            return curr;
        }
    }, {value: 0, amount: 0}).value;

    let mean = multiplied_sum / sum_histogram_amounts;

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
        mean: mean,
        mode: mode,
        q3: q3_calculated.quartile,
        max: max,
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
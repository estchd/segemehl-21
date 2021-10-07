export function bin_data_to_line_data(bin_data) {
    let i = 0;
    let line_data = [];

    for (const data of bin_data) {
        if (isNaN(data)) {
            line_data.push({
                x: i,
                y: 0
            });
        }
        else {
            line_data.push({
                x: i,
                y: data
            });
        }

        i++;
    }

    return line_data;
}

export function generate_labels(data, bin_size) {
    let labels = [];

    for (let i = 0; i < data.length; i++) {
        labels.push(i * bin_size);
    }

    return labels;
}

export function decimate_bin_data_max_min(bin_data, expected_samples ,threshold) {
    let length = bin_data.length;

    if (expected_samples === 0) {return bin_data;}
    if (length <= expected_samples) {return bin_data;}
    if (length < threshold) {return bin_data;}

    let last_only_max = (length % 2 !== 0);

    let ratio = length / expected_samples;
    let decimation_size = Math.round(ratio);

    let decimation_count;

    if (last_only_max) {
        decimation_count = (expected_samples - 1) / 2;
    }
    else{
        decimation_count = expected_samples / 2;
    }

    let decimated_data = [];

    let bin_data_index = 0;

    for (let i = 0; i < decimation_count; i++) {
        let max;
        let min;

        for(let l = 0; l < decimation_size; l++) {
            let element = bin_data[bin_data_index];

            if (!max || element.y > max.y) {
                max = element;
            }
            if (!min || element.y < min.y) {
                min = element;
            }

            bin_data_index++;
        }

        if (max.x === min.x) {
            decimated_data.push(max);
        }
        else if (max.x > min.x) {
            decimated_data.push(min);
            decimated_data.push(max);
        }
        else{
            decimated_data.push(max);
            decimated_data.push(min);
        }
    }

    let last_max;

    while(bin_data_index < length) {
        let element = bin_data[bin_data_index];

        if (!last_max || element.y > last_max.y) {
            last_max = element;
        }

        bin_data_index++;
    }

    decimated_data.push(last_max);

    return decimated_data;
}

export function calculate_decimation_size(bin_data, expected_samples ,threshold) {
    let length = bin_data.length;

    if (expected_samples === 0) {return 1;}
    if (length <= expected_samples) {return 1;}
    if (length < threshold) {return 1;}

    let ratio = length / expected_samples;

    return Math.floor(ratio);
}

export function decimate_bin_data_mean(bin_data, expected_samples ,threshold) {
    let length = bin_data.length;

    if (expected_samples === 0) {return bin_data;}
    if (length <= expected_samples) {return bin_data;}
    if (length < threshold) {return bin_data;}

    let ratio = length / expected_samples;

    let decimation_size = Math.floor(ratio);

    let decimation_count = expected_samples;

    let decimated_data = [];

    let bin_data_index = 0;

    for (let i = 0; i < decimation_count; i++) {
        let sum = 0;

        for(let l = 0; l < decimation_size; l++) {
            let element = bin_data[bin_data_index];

            sum += element.y;

            bin_data_index++;
        }

        decimated_data.push(sum / decimation_size);
    }

    let last_sum = 0;
    let last_count = 0;

    while(bin_data_index < length) {
        let element = bin_data[bin_data_index];

        last_sum += element;
        last_count++;

        bin_data_index++;
    }

    decimated_data.push(last_sum / last_count);


    return decimated_data;
}
import {clear_references, has_references, set_references} from "./reference_list";

let files = new Map();

export function colors_from_array(array) {
    return {
        main_color: array[0],
        match_color: array[1],
        insertion_color: array[2],
        deletion_color: array[3],
        skip_color: array[4]
    }
}

export function colors_to_array(colors) {
    return [
        colors.main_color,
        colors.match_color,
        colors.insertion_color,
        colors.deletion_color,
        colors.skip_color
    ];
}

export function add_file(name, colors, stats, data) {
    if (files.has(name)) {return;}
    files.set(name, {
        colors: colors,
        data: data,
        stats: stats
    });
}

export function complete_file(name, stats, data, references) {
    if (!files.has(name)) {return;}

    let file = files.get(name);
    file.stats = stats;
    file.data = data;

    if (!has_references()) {
        set_references(references);
    }
}

export function remove_file(name) {
    if (!files.has(name)) {return;}
    files.delete(name);
    if (files.size === 0) {
        clear_references();
    }
}

export function has_file(name) {
    return files.has(name);
}

export function get_file_list() {
    let list = [];

    files.forEach((value, key) => {
        list.push([
            key,
            value.colors,
            value.data !== undefined
        ])
    })

    return list;
}

export function get_per_file_stats(file_name) {
    if (!files.has(file_name)) {return undefined;}

    let file = files.get(file_name);
    return file.stats;
}
export function get_file_colors(file_name) {
    if (!files.has(file_name)) {return undefined;}

    let file = files.get(file_name);
    return colors_to_array(file.colors);
}

export function get_dataset(file_name, dataset_name) {
    if (!files.has(file_name)) {return undefined;}

    let file = files.get(file_name);
    let data = file.data;

    if (!data[dataset_name]) {return undefined;}
    return data[dataset_name];
}

export function update_file_color(file_name, color, index) {
    if (!files.has(file_name)) {return;}

    let file = files.get(file_name);

    switch (index) {
        case 0:
            file.colors.main_color = color;
            break;
        case 1:
            file.colors.match_color = color;
            break;
        case 2:
            file.colors.insertion_color = color;
            break;
        case 3:
            file.colors.deletion_color = color;
            break;
        case 4:
            file.colors.skip_color = color;
            break;
    }
}
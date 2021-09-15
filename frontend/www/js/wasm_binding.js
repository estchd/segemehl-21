let generate_per_file_stats_func;
let get_chromosome_names_func;
let setup_file_list_func;
let add_file_func;
let process_file_func;
let remove_file_func;
let get_file_list_func;
let get_dataset_func;
let get_file_colors_func;
let update_file_color_func;

export function setup_functions(
    generate_per_file_stats,
    get_chromosome_names,
    setup_file_list,
    add_file,
    process_file,
    remove_file,
    get_file_list,
    get_dataset,
    get_file_colors,
    update_file_color,
) {
    generate_per_file_stats_func = generate_per_file_stats;
    get_chromosome_names_func = get_chromosome_names;
    setup_file_list_func = setup_file_list;
    add_file_func = add_file;
    process_file_func = process_file;
    remove_file_func = remove_file;
    get_file_list_func = get_file_list;
    get_dataset_func = get_dataset;
    get_file_colors_func = get_file_colors;
    update_file_color_func = update_file_color;

}

export function get_reference_list() {
    return get_chromosome_names_func();
}

export function generate_per_file_stats(file_name) {
    return generate_per_file_stats_func(file_name);
}

export function setup_file_list() {
    return setup_file_list_func();
}

export function add_file(file, colors) {
    return add_file_func(file, colors);
}

export function process_file(file) {
    return process_file_func(file);
}

export function remove_file(file_name) {
    return remove_file_func(file_name);
}

export function get_file_list() {
    return get_file_list_func();
}

export function get_dataset(file_name, dataset_name) {
    return get_dataset_func(file_name, dataset_name);
}

export function get_file_colors(file_name) {
    return get_file_colors_func(file_name);
}

export function update_file_color(file_name, new_color, index) {
    return update_file_color_func(file_name, new_color, index);
}


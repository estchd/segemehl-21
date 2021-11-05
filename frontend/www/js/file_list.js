import {rebuild_numeric_statistics} from "./numeric_statistics";
import {update_all_plots} from "./plots";
import {
    add_file,
    complete_file,
    get_file_list,
    get_per_file_stats,
    has_file,
    remove_file,
    update_file_color
} from "./file_storage";
import {check_references, get_reference_names, has_references, setup_chromosome_list} from "./reference_list";

const file_input = document.getElementById("file_input");
const file_dropdown = document.getElementById("file-dropdown");
const chromosome_select = document.getElementById("chromosome-select");

const chromosome_select_template = document.getElementById("chromosome-select-template");

const list_item_template = document.getElementById("file-dropdown-list-item-template");
const add_button_template = document.getElementById("file-dropdown-add-button-template");
const remove_button_template = document.getElementById("file-dropdown-remove-button-template");
const loading_status_template = document.getElementById("file-dropdown-loading-status-template");

let background_worker;

export function setup_file_system()
{
    setup_chromosome_list();
    setup_background_worker(handle_process_file_completion, handle_process_file_fail);
    rebuild_file_list();
    file_input.addEventListener("change", () => handle_file_input_change());
}

function setup_background_worker() {
    background_worker = new Worker("./worker.js");
    background_worker.addEventListener("message", (e) => handle_worker_message(e));
    background_worker.postMessage({files:[]});
}

function handle_add_file_button_click() {
    file_input.click();
}

async function handle_file_input_change() {
    let files = file_input.files;
    let added_files = [];

     for (const file of files) {
         const name = file.name;

         if (has_file(name)) {
             continue;
         }

         let colors = get_random_colors();

         add_file(name,colors);
         added_files.push(file);
     }

     let promises = [];
     for (const file of added_files) {
         promises.push(file.arrayBuffer());
     }

     let array_buffers = await Promise.all(promises);

     let processed_files = [];
     for (let i = 0; i < added_files.length; i++) {
        let file = added_files[i];
        let array_buffer = array_buffers[i];

        processed_files.push({name: file.name, buffer: array_buffer});
     }

     background_worker.postMessage(
         {
            files: processed_files
         },
         array_buffers
     );

     rebuild_file_list();
     update_all_plots();
}

function handle_worker_message(message) {
    let data = message.data;

    let type = data.type;

    switch(type) {
        case "success":
            handle_process_file_completion(data.files);
            break;
        case "error":
            handle_process_file_fail(data.files);
            break;
    }
}

function handle_process_file_completion(files) {
    for (const file of files) {
        let name = file.name;
        let stats = file.stats;
        let data = file.data;
        let references = file.references;

        if (has_references() && !check_references(references)) {
            remove_file(name);
        }
        else {
            complete_file(name, stats, data, references);
        }
    }

    rebuild_file_list();
    update_all_plots();
}

function handle_process_file_fail(files) {

    for (const file of files) {

        console.error("file promise failed for file " + file.name);
        console.error("Err: " + file.err);

        remove_file(file.name);
    }

    rebuild_file_list();
    update_all_plots();
}

function handle_remove_file_click(file_name) {
    remove_file(file_name);
    rebuild_file_list();
}

function handle_file_color_change(file_name, new_color, index) {
    update_file_color(file_name, new_color, index);
    update_all_plots();
}

function rebuild_file_list() {
    clear_dropdown();

    let files = get_file_list();

    for (const file of files) {
        let name = file[0];
        let colors = file[1];
        let is_loaded = file[2];

        let clone = clone_list_item(name, colors, is_loaded);
        let list_item = clone[0];
        let list_divider = clone[1];

        add_dropdown_row(list_item);
        add_dropdown_row(list_divider);
    }

    let add_button = clone_add_button();
    add_dropdown_row(add_button);

    hookup_list_events();

    rebuild_chromosome_list();

    rebuild_statistic_display();
    update_all_plots();
}

function rebuild_statistic_display() {
    const files = get_file_list();

    let file_names = [];
    let per_file_statistics = [];

    for (const file_name of files) {
        if (!file_name[2]) {continue;}

        let statistics = get_per_file_stats(file_name[0]);

        if (!statistics) { continue; }

        file_names.push(file_name[0]);
        per_file_statistics.push(statistics);
    }

    rebuild_numeric_statistics(file_names, per_file_statistics);
}

function rebuild_chromosome_list() {
    clear_chromosome_select();

    let chromosome_list = get_reference_names();

    for (const chromosome of chromosome_list) {
        let item = clone_chromosome_select_item(chromosome);
        add_chromosome_select_item(item);
    }
}

function hookup_list_events() {
    let list_items = file_dropdown.querySelectorAll(".file-list-item");

    for (const item of list_items) {
        hookup_list_item_events(item);
    }

    hookup_add_file_button_events();
}

function hookup_list_item_events(list_item) {
    const remove_button = list_item.querySelector(".file-list-item-remove-button");
    if (!remove_button) {return;}

    const name_div = list_item.querySelector(".file-list-item-name");

    const name = name_div.innerHTML;

    remove_button.addEventListener("click", () => handle_remove_file_click(name));
}

function hookup_add_file_button_events() {
    const add_button = file_dropdown.querySelector(".file-list-add-button");

    add_button.addEventListener("click", handle_add_file_button_click);
}

function clear_chromosome_select() {
    chromosome_select.innerHTML = "";
}

function add_chromosome_select_item(item) {
    chromosome_select.appendChild(item);
}

function clear_dropdown() {
    file_dropdown.innerHTML = "";
}

function add_dropdown_row(row) {
    file_dropdown.appendChild(row);
}

function clone_chromosome_select_item(name) {
    let clone = chromosome_select_template.content.firstElementChild.cloneNode(true);

    clone.value = name;
    clone.innerHTML = name;

    return clone;
}

function clone_list_item(name, color, is_loaded) {
    let list_item_clone = list_item_template.content.firstElementChild.cloneNode(true);
    let list_divider_clone = list_item_template.content.lastElementChild.cloneNode(true);

    let name_div = list_item_clone.firstElementChild;
    let status_div = list_item_clone.lastElementChild.lastElementChild;

    let status_clone;

    if (is_loaded) {
        status_clone = clone_remove_button();
    }
    else {
        status_clone = clone_loading_status();
    }

    name_div.innerHTML = name;
    status_div.appendChild(status_clone);

    let main_color_picker = list_item_clone.querySelector(".file-list-item-color");
    main_color_picker.value = color[0];
    main_color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color, 0);
    });
    let match_color_picker = list_item_clone.querySelector(".file-list-match-color");
    match_color_picker.value = color[1];
    match_color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color, 1);
    })
    let insertion_color_picker = list_item_clone.querySelector(".file-list-insertion-color");
    insertion_color_picker.value = color[2];
    insertion_color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color, 2);
    })
    let deletion_color_picker = list_item_clone.querySelector(".file-list-deletion-color");
    deletion_color_picker.value = color[3];
    deletion_color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color, 3);
    })
    let skip_color_picker = list_item_clone.querySelector(".file-list-skip-color");
    skip_color_picker.value = color[4];
    skip_color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color, 4);
    })

    return [list_item_clone, list_divider_clone];
}

function clone_add_button() {
    return add_button_template.content.firstElementChild.cloneNode(true);
}

function clone_remove_button() {
    return remove_button_template.content.firstElementChild.cloneNode(true);
}

function clone_loading_status() {
    return loading_status_template.content.firstElementChild.cloneNode(true);
}

export function get_random_colors() {
    let random_hue = Math.random();

    let main_saturation = 1.0;
    let match_saturation = 1.0;
    let insertion_saturation = 1.0 - (0.75 / 3.0);
    let deletion_saturation = 1.0 - 2.0 * (0.75 / 3.0);
    let skip_saturation = 1.0 - 3.0 * (0.75 / 3.0);

    let main_rgb = HSVtoRGB(random_hue, main_saturation, 1.0);
    let match_rgb = HSVtoRGB(random_hue, match_saturation, 1.0);
    let insertion_rgb = HSVtoRGB(random_hue, insertion_saturation, 1.0);
    let deletion_rgb = HSVtoRGB(random_hue, deletion_saturation, 1.0);
    let skip_rgb = HSVtoRGB(random_hue, skip_saturation, 1.0);

    let main_color = rgbToHex(main_rgb.r, main_rgb.g, main_rgb.b);
    let match_color = rgbToHex(match_rgb.r, match_rgb.g, match_rgb.b);
    let insertion_color = rgbToHex(insertion_rgb.r, insertion_rgb.g, insertion_rgb.b);
    let deletion_color = rgbToHex(deletion_rgb.r, deletion_rgb.g, deletion_rgb.b);
    let skip_color = rgbToHex(skip_rgb.r, skip_rgb.g, skip_rgb.b);

    let colors = [main_color, match_color, insertion_color, deletion_color, skip_color];

    return colors;
}

// copied from https://stackoverflow.com/questions/5623838/rgb-to-hex-and-hex-to-rgb
function componentToHex(c) {
    var hex = c.toString(16);
    return hex.length == 1 ? "0" + hex : hex;
}

// copied from https://stackoverflow.com/questions/5623838/rgb-to-hex-and-hex-to-rgb
function rgbToHex(r, g, b) {
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

// copied from https://stackoverflow.com/questions/17242144/javascript-convert-hsb-hsv-color-to-rgb-accurately
function HSVtoRGB(h, s, v) {
    var r, g, b, i, f, p, q, t;
    if (arguments.length === 1) {
        s = h.s, v = h.v, h = h.h;
    }
    i = Math.floor(h * 6);
    f = h * 6 - i;
    p = v * (1 - s);
    q = v * (1 - f * s);
    t = v * (1 - (1 - f) * s);
    switch (i % 6) {
        case 0: r = v, g = t, b = p; break;
        case 1: r = q, g = v, b = p; break;
        case 2: r = p, g = v, b = t; break;
        case 3: r = p, g = q, b = v; break;
        case 4: r = t, g = p, b = v; break;
        case 5: r = v, g = p, b = q; break;
    }
    return {
        r: Math.round(r * 255),
        g: Math.round(g * 255),
        b: Math.round(b * 255)
    };
}





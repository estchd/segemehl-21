import {stat} from "copy-webpack-plugin/dist/utils/promisify";
import {rebuild_numeric_statistics} from "./numeric_statistics";
import {update_all_plots} from "./plots";
import {
    add_file,
    get_chromosome_list,
    get_file_list,
    get_per_file_stats,
    process_file,
    remove_file,
    setup_file_list, update_file_color
} from "./wasm_binding";

const file_input = document.getElementById("file_input");
const file_dropdown = document.getElementById("file-dropdown");
const chromosome_select = document.getElementById("chromosome-select");

const chromosome_select_template = document.getElementById("chromosome-select-template");

const list_item_template = document.getElementById("file-dropdown-list-item-template");
const add_button_template = document.getElementById("file-dropdown-add-button-template");
const remove_button_template = document.getElementById("file-dropdown-remove-button-template");
const loading_status_template = document.getElementById("file-dropdown-loading-status-template");

export function setup_file_system()
{
    setup_file_list();
    rebuild_file_list();
    file_input.addEventListener("change", () => handle_file_input_change());
}

function handle_add_file_button_click() {
    file_input.click();
}

async function handle_file_input_change() {
    let content_promises = [];

    for (const file of file_input.files) {
        const name = file.name;

        let files = get_file_list();
        if (files.includes(name)) {
            continue;
        }

        let color = getRandomColor();

        add_file(file, color);
        let promise = process_file(file)
            .then(() => {
                rebuild_file_list();
                update_all_plots();
            })
            .catch(() => {
                rebuild_file_list();
                update_all_plots();
            });
        content_promises.push(promise);
    }
    rebuild_file_list();
    update_all_plots();
    await Promise.all(content_promises);
}

function handle_remove_file_click(file_name) {
    remove_file(file_name);
    rebuild_file_list();
}

function handle_file_color_change(file_name, new_color) {
    update_file_color(file_name, new_color);
    update_all_plots();
}

function rebuild_file_list() {
    clear_dropdown();

    let files = get_file_list();

    for (const file of files) {
        let name = file[0];
        let color = file[1];
        let is_loaded = file[2];

        let clone = clone_list_item(name, color, is_loaded);
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
        if (!file_name[1]) {continue;}

        let statistics = get_per_file_stats(file_name[0]);

        if (!statistics) { continue; }

        file_names.push(file_name[0]);
        per_file_statistics.push(statistics);
    }

    rebuild_numeric_statistics(file_names, per_file_statistics);
}

function rebuild_chromosome_list() {
    clear_chromosome_select();

    let chromosome_list = get_chromosome_list();

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

    remove_button.addEventListener("click",function() {
        handle_remove_file_click(name);
    })
}

function hookup_add_file_button_events() {
    const add_button = file_dropdown.querySelector(".file-list-add-button");

    add_button.addEventListener("click",handle_add_file_button_click);
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

    let color_picker = list_item_clone.querySelector(".file-list-item-color");
    color_picker.value = color;
    color_picker.addEventListener("change", function(event) {
        let color = event.target.value;
        handle_file_color_change(name, color);
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

function getRandomColor() {
    var letters = '0123456789ABCDEF'.split('');
    var color = '#';
    for (var i = 0; i < 6; i++ ) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}


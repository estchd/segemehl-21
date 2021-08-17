import {stat} from "copy-webpack-plugin/dist/utils/promisify";
import {rebuild_numeric_statistics} from "./numeric_statistics";

let add_file_func;
let process_file_func;
let remove_file_func;
let get_file_list_func;
let get_chromosome_list_func;
let get_per_file_statistics_func;

const file_input = document.getElementById("file_input");
const file_dropdown = document.getElementById("file-dropdown");
const chromosome_select = document.getElementById("chromosome-select");

const chromosome_select_template = document.getElementById("chromosome-select-template");

const list_item_template = document.getElementById("file-dropdown-list-item-template");
const add_button_template = document.getElementById("file-dropdown-add-button-template");
const remove_button_template = document.getElementById("file-dropdown-remove-button-template");
const loading_status_template = document.getElementById("file-dropdown-loading-status-template");

export function setup_file_system(
    setup_file_list,
    add_file,
    process_file,
    remove_file,
    get_file_list,
    get_chromosome_list,
    get_per_file_statistics,
    )
{
    add_file_func = add_file;
    process_file_func = process_file;
    remove_file_func = remove_file;
    get_file_list_func = get_file_list;
    get_chromosome_list_func = get_chromosome_list;
    get_per_file_statistics_func = get_per_file_statistics;

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

        let files = get_file_list_func();
        if (files.includes(name)) {
            continue;
        }

        add_file_func(file);
        let promise = process_file_func(file)
            .then(() => {
                rebuild_file_list();
            })
            .catch(() => {
                rebuild_file_list();
            });
        content_promises.push(promise);
    }
    rebuild_file_list();
    await Promise.all(content_promises);
}

function handle_remove_file_click(file_name) {
    remove_file_func(file_name);
    rebuild_file_list();
}

function rebuild_file_list() {
    clear_dropdown();

    let files = get_file_list_func();

    for (const file of files) {
        let name = file[0];
        let is_loaded = file[1];

        let clone = clone_list_item(name, is_loaded);
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
}

function rebuild_statistic_display() {
    const files = get_file_list_func();

    let file_names = [];
    let per_file_statistics = [];

    for (const file_name of files) {
        if (!file_name[1]) {continue;}

        let statistics = get_per_file_statistics_func(file_name[0]);

        if (!statistics) { continue; }

        file_names.push(file_name[0]);
        per_file_statistics.push(statistics);
    }

    rebuild_numeric_statistics(file_names, per_file_statistics);
}

function rebuild_chromosome_list() {
    clear_chromosome_select();

    let chromosome_list = get_chromosome_list_func();

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

function clone_list_item(name, is_loaded) {
    let list_item_clone = list_item_template.content.firstElementChild.cloneNode(true);
    let list_divider_clone = list_item_template.content.lastElementChild.cloneNode(true);

    let name_div = list_item_clone.firstElementChild;
    let status_div = list_item_clone.lastElementChild;

    let status_clone;

    if (is_loaded) {
        status_clone = clone_remove_button();
    }
    else {
       status_clone = clone_loading_status();
    }

    name_div.innerHTML = name;
    status_div.appendChild(status_clone);

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



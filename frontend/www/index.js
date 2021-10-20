import {setup_file_system} from "./js/file_list";
import {setup_plots} from "./js/plots";

export async function setup() {
    setup_plots();
    setup_file_system();
}
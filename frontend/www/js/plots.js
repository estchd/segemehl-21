import {get_reference_list, get_dataset, get_file_list} from "./wasm_binding";
import {
    setup_coverage_plots,
    update_reference_dependent_coverage_plots,
    update_coverage_plots
} from "./plots/coverage_plots";
import {setup_cigar_plots, update_cigar_plots, update_reference_dependent_cigar_plots} from "./plots/cigar_plots";
import {
    setup_read_quality_plots,
    update_read_quality_plots,
    update_reference_dependent_read_quality_plots
} from "./plots/read_quality_plots";
import {
    setup_read_length_plots,
    update_read_length_plots,
    update_reference_dependent_read_length_plots
} from "./plots/read_length_plots";
import {
    setup_unmapped_plots,
    update_reference_dependent_unmapped_plots,
    update_unmapped_plots
} from "./plots/unmapped_plots";
import {
    setup_reference_plots,
    update_reference_dependent_reference_plots,
    update_reference_plots
} from "./plots/reference_plots";
import {
    setup_split_read_plots,
    update_reference_dependent_split_read_plots,
    update_split_read_plots
} from "./plots/split_read_plots";


export function setup_plots() {
    setup_split_read_plots();
    setup_reference_plots();
    setup_unmapped_plots();
    setup_coverage_plots();
    setup_cigar_plots();
    setup_read_quality_plots();
    setup_read_length_plots();

    update_all_plots();

    selected_chromosome.addEventListener("change", () => update_reference_dependent_plots());
}

export function update_all_plots() {
    update_split_read_plots();
    update_reference_plots();
    update_unmapped_plots();
    update_coverage_plots();
    update_cigar_plots();
    update_read_quality_plots();
    update_read_length_plots();

    update_reference_dependent_plots();
}

function update_reference_dependent_plots() {
    update_reference_dependent_split_read_plots();
    update_reference_dependent_reference_plots();
    update_reference_dependent_unmapped_plots();
    update_reference_dependent_coverage_plots();
    update_reference_dependent_cigar_plots();
    update_reference_dependent_read_quality_plots();
    update_reference_dependent_read_length_plots();
}

export function linking_update_selected_reference(index) {
    let reference_names = get_reference_list();
    if (reference_names[index]) {
        selected_chromosome.value = reference_names[index];
    }
    else {
        selected_chromosome.value = reference_names[0];
    }

    update_reference_dependent_plots();
}

const selected_chromosome = document.getElementById("chromosome-select");
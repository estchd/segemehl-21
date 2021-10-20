let reference_list = new Set();
let has_references_bool = false;

export function setup_chromosome_list() {
    reference_list = new Set();
    has_references_bool = false;
}

export function check_references(references) {
    if (!has_references_bool) {return false;}
    references = new Set(references);

    reference_list.forEach((reference) => {
        if (!references.includes(reference)) {
            return false;
        }

        references.delete(reference);
    })

    return references.size === 0;
}

export function set_references(references) {
    reference_list = new Set(references);
    has_references_bool = true;
}

export function clear_references() {
    reference_list = new Set();
    has_references_bool = false;
}

export function get_reference_names() {
    return Array.from(reference_list);
}

export function has_references() {
    return has_references_bool;
}
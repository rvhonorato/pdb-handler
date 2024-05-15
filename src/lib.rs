mod constants;
mod handler;
mod js_utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn list_unknown_residues(bytes: js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let unknown_res_map = handler::identify_unknowns(structure);
    js_utils::hashmap_to_js_object(unknown_res_map)
}

#[wasm_bindgen]
pub fn list_chains(bytes: js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let chains = handler::identify_chains(structure);
    serde_wasm_bindgen::to_value(&chains).unwrap()
}

#[wasm_bindgen]
pub fn guess_moltype(bytes: js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let moltypes = handler::identify_molecular_types(structure);
    js_utils::hashmap_to_js_object(moltypes)
}

#[wasm_bindgen]
pub fn list_residues(bytes: js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let residues = handler::identify_residue_numbers(structure);
    js_utils::hashmap_to_js_object(residues)
}

#[wasm_bindgen]
pub fn chains_in_contact(bytes: js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let contacts = handler::chains_in_contact(structure);
    serde_wasm_bindgen::to_value(&contacts).unwrap()
}

pub fn set_panic_hook() {
    // See https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

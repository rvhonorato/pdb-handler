// #![allow(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
// #![deny(clippy::unwrap_used)]
#![allow(clippy::must_use_candidate)]
mod constants;
mod handler;
mod js_utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn list_unknown_residues(bytes: &js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let unknown_res_map = handler::identify_unknowns(&structure);
    js_utils::hashmap_to_js_object(unknown_res_map)
}

/// # Panics
///
/// If the input is not a valid PDB file
#[wasm_bindgen]
pub fn list_chains(bytes: &js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let chains = handler::identify_chains(&structure);
    serde_wasm_bindgen::to_value(&chains).unwrap()
}

#[wasm_bindgen]
pub fn guess_moltype(bytes: &js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let moltypes = handler::identify_molecular_types(&structure);
    js_utils::hashmap_to_js_object(moltypes)
}

#[wasm_bindgen]
pub fn list_residues(bytes: &js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let residues = handler::identify_residue_numbers(&structure);
    js_utils::hashmap_to_js_object(residues)
}

/// # Panics
///
/// This function will panic if the input is not a valid PDB file.
#[wasm_bindgen]
pub fn chains_in_contact(bytes: &js_sys::Uint8Array) -> JsValue {
    set_panic_hook();
    let structure = js_utils::load_pdb_from_bytes(bytes);
    let contacts = handler::chains_in_contact(&structure);
    serde_wasm_bindgen::to_value(&contacts).unwrap()
}

pub fn set_panic_hook() {
    // See https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

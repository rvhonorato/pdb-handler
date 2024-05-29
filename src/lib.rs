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

use js_utils::load_pdb_from_bytes;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_CUSTOM_SECTION: &'static str = r#"

interface PDBErrorWrapper {
    level: string;
    short_description: string;
}

interface MolecularType {
    Protein: string;
    Dna: string;
    Other: string;
}

export function valid_pdb(bytes: Uint8Array): any | PDBErrorWrapper;

export function list_chains(bytes: Uint8Array): string[] | PDBErrorWrapper;

export function chains_in_contact(bytes: Uint8Array): [string, string][] | PDBErrorWrapper;

export function list_unknown_residues(bytes: Uint8Array): { [key: string]: string[] } | PDBErrorWrapper;

export function list_residues(bytes: Uint8Array): { [key: string]: string[] } | PDBErrorWrapper;

export function guess_moltype(bytes: Uint8Array): { [key: string]: MolecularType[] } | PDBErrorWrapper;

"#;

/// # Errors
///
/// If the input is not a valid PDB file, it will return an error message.
///
/// # Panics
///
/// If it cannot serialize the structure or if it cannot serialize the error
/// message.
///
#[wasm_bindgen(skip_typescript)]
pub fn valid_pdb(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let js_value = serde_wasm_bindgen::to_value(&structure).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

/// # Panics
/// If the input is not a valid PDB file or if it cannot serialize the error message.
///
/// # Errors
/// If the input is not a valid PDB file, it will return an error message.
#[wasm_bindgen(skip_typescript)]
pub fn list_unknown_residues(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let unknown_res_map = handler::identify_unknowns(&structure);
            let js_value = serde_wasm_bindgen::to_value(&unknown_res_map).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

/// # Panics
///
/// If the input is not a valid PDB file or if it cannot serialize the chain vector.
///
/// # Errors
///
/// If the input is not a valid PDB file, it will return an error message.
#[wasm_bindgen(skip_typescript)]
pub fn list_chains(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let chains = handler::identify_chains(&structure);
            let js_value = serde_wasm_bindgen::to_value(&chains).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

/// # Panics
/// If the input is not a valid PDB file or if it cannot serialize the molecular types.
/// # Errors
/// If the input is not a valid PDB file, it will return an error message.
pub fn guess_moltype(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let mol_types = handler::identify_molecular_types(&structure);
            let js_value = serde_wasm_bindgen::to_value(&mol_types).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

/// # Panics
/// If the input is not a valid PDB file or if it cannot serialize the residue numbers.
///
/// # Errors
/// If the input is not a valid PDB file, it will return an error message.
#[wasm_bindgen(skip_typescript)]
pub fn list_residues(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let residues = handler::identify_residue_numbers(&structure);
            let js_value = serde_wasm_bindgen::to_value(&residues).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

/// # Panics
///
/// This function will panic if the input is not a valid PDB file.
///
/// # Errors
///
/// This function will return an error message if the input is not a valid PDB file.
#[wasm_bindgen(skip_typescript)]
pub fn chains_in_contact(bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
    set_panic_hook();
    match load_pdb_from_bytes(bytes) {
        Ok(structure) => {
            let contacts = handler::chains_in_contact(&structure);
            let js_value = serde_wasm_bindgen::to_value(&contacts).unwrap();
            Ok(js_value)
        }
        Err(e) => {
            let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
            Err(js_value)
        }
    }
}

pub fn set_panic_hook() {
    // See https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

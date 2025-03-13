use pdbtbx::ReadOptions;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PDBErrorWrapper {
    level: String,
    short_description: String,
    long_description: String,
    context: String,
}

#[wasm_bindgen]
pub struct PdbHandlerApi {}

#[wasm_bindgen]
#[allow(clippy::new_without_default)]
impl PdbHandlerApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PdbHandlerApi {}
    }

    pub fn list_chains(&self, bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
        match load_pdb_from_bytes(bytes) {
            Ok(structure) => {
                let chains = pdb_handler::identify_chains(&structure);
                let js_value = serde_wasm_bindgen::to_value(&chains).unwrap();
                Ok(js_value)
            }
            Err(e) => {
                let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
                Err(js_value)
            }
        }
    }

    pub fn chains_in_contact(&self, bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
        match load_pdb_from_bytes(bytes) {
            Ok(structure) => {
                let contacts = pdb_handler::chains_in_contact(&structure);
                let js_value = serde_wasm_bindgen::to_value(&contacts).unwrap();
                Ok(js_value)
            }
            Err(e) => {
                let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
                Err(js_value)
            }
        }
    }

    pub fn list_residues(&self, bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
        match load_pdb_from_bytes(bytes) {
            Ok(structure) => {
                let residues = pdb_handler::identify_residue_numbers(&structure);
                let js_value = serde_wasm_bindgen::to_value(&residues).unwrap();
                Ok(js_value)
            }
            Err(e) => {
                let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
                Err(js_value)
            }
        }
    }

    pub fn guess_moltype(&self, bytes: &js_sys::Uint8Array) -> Result<JsValue, JsValue> {
        match load_pdb_from_bytes(bytes) {
            Ok(structure) => {
                let mol_types = pdb_handler::identify_molecular_types(&structure);
                let js_value = serde_wasm_bindgen::to_value(&mol_types).unwrap();
                Ok(js_value)
            }
            Err(e) => {
                let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
                Err(js_value)
            }
        }
    }

    pub fn list_unknown_residues(&self, bytes: js_sys::Uint8Array) -> Result<JsValue, JsValue> {
        match load_pdb_from_bytes(&bytes) {
            Ok(structure) => {
                let unknown_res_map = pdb_handler::identify_unknowns(&structure);
                let js_value = serde_wasm_bindgen::to_value(&unknown_res_map).unwrap();
                Ok(js_value)
            }
            Err(e) => {
                let js_value = serde_wasm_bindgen::to_value(&e).unwrap();
                Err(js_value)
            }
        }
    }
}

pub fn load_pdb_from_bytes(bytes: &js_sys::Uint8Array) -> Result<pdbtbx::PDB, PDBErrorWrapper> {
    let pdb_string = if bytes.is_null() {
        String::new()
    } else {
        let vec = bytes.to_vec();
        String::from_utf8(vec).unwrap()
    };

    let bytes = pdb_string.as_bytes().to_vec();
    let cursor = Cursor::new(bytes);
    let buf = BufReader::new(cursor);

    let mut opts = ReadOptions::new();
    opts.set_format(pdbtbx::Format::Pdb)
        .set_level(pdbtbx::StrictnessLevel::Loose);

    match opts.read_raw(buf) {
        Ok((pdb, _)) => Ok(pdb),
        Err(e) => {
            let collapsed_e = collapse_pdb_error(&e);
            Err(collapsed_e)
        }
    }
}

fn collapse_pdb_error(e: &[pdbtbx::PDBError]) -> PDBErrorWrapper {
    let e: pdbtbx::PDBError = e[0].clone();

    let pdb_error = PDBErrorWrapper {
        level: e.level().to_string(),
        short_description: e.short_description().to_string(),
        long_description: e.long_description().to_string(),
        context: e.context().to_string(),
    };

    pdb_error
}

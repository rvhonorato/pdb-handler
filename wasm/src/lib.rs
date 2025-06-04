use pdbtbx::ReadOptions;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PDBErrorWrapper {
    level: String,
    short_description: String,
    long_description: String,
    context: String,
}

#[wasm_bindgen]
pub struct PdbHandlerApi {}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ChainContact {
    chain1: String,
    chain2: String,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ChainData {
    chain: String,
    items: Vec<String>,
}

#[wasm_bindgen]
#[allow(clippy::new_without_default)]
impl PdbHandlerApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PdbHandlerApi {}
    }

    pub fn list_chains(&self, bytes: &js_sys::Uint8Array) -> Result<Vec<String>, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(bytes)?;
        let chains = pdb_handler::identify_chains(&structure);
        Ok(chains)
    }

    pub fn chains_in_contact(
        &self,
        bytes: &js_sys::Uint8Array,
    ) -> Result<Vec<ChainContact>, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(bytes)?;
        let contacts = pdb_handler::chains_in_contact(&structure)
            .into_iter()
            .map(|(chain1, chain2)| ChainContact { chain1, chain2 })
            .collect();
        Ok(contacts)
    }

    pub fn list_residues(
        &self,
        bytes: &js_sys::Uint8Array,
    ) -> Result<Vec<ChainData>, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(bytes)?;
        let residues = pdb_handler::identify_residue_numbers(&structure)
            .into_iter()
            .map(|(c, res)| ChainData {
                chain: c,
                items: res,
            })
            .collect();
        Ok(residues)
    }

    pub fn guess_moltype(
        &self,
        bytes: &js_sys::Uint8Array,
    ) -> Result<Vec<ChainData>, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(bytes)?;
        let mol_types = pdb_handler::identify_molecular_types(&structure)
            .into_iter()
            .map(|(c, v)| ChainData {
                chain: c,
                items: v.into_iter().map(String::from).collect(),
            })
            .collect();
        Ok(mol_types)
    }

    pub fn list_unknown_residues(
        &self,
        bytes: js_sys::Uint8Array,
    ) -> Result<Vec<ChainData>, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(&bytes)?;
        let unknown_res_map = pdb_handler::identify_unknowns(&structure)
            .into_iter()
            .map(|(chain, residues)| ChainData {
                chain,
                items: residues,
            })
            .collect();
        Ok(unknown_res_map)
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

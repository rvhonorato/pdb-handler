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
pub struct PdbHandlerApi {
    structure: pdbtbx::PDB,
}

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
    pub fn new(bytes: &js_sys::Uint8Array) -> Result<PdbHandlerApi, PDBErrorWrapper> {
        let structure = load_pdb_from_bytes(bytes)?;
        Ok(PdbHandlerApi { structure })
    }

    pub fn list_chains(&self) -> Vec<String> {
        pdb_handler::identify_chains(&self.structure)
    }

    pub fn chains_in_contact(&self) -> Vec<ChainContact> {
        pdb_handler::chains_in_contact(&self.structure)
            .into_iter()
            .map(|(chain1, chain2)| ChainContact { chain1, chain2 })
            .collect()
    }

    pub fn list_residues(&self) -> Vec<ChainData> {
        pdb_handler::identify_residue_numbers(&self.structure)
            .into_iter()
            .map(|(c, res)| ChainData {
                chain: c,
                items: res,
            })
            .collect()
    }

    pub fn guess_moltype(&self) -> Vec<ChainData> {
        pdb_handler::identify_molecular_types(&self.structure)
            .into_iter()
            .map(|(c, v)| ChainData {
                chain: c,
                items: v.into_iter().map(String::from).collect(),
            })
            .collect()
    }

    pub fn list_unknown_residues(&self) -> Vec<ChainData> {
        pdb_handler::identify_unknowns(&self.structure)
            .into_iter()
            .map(|(chain, residues)| ChainData {
                chain,
                items: residues,
            })
            .collect()
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

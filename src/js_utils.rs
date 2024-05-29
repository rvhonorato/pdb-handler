use js_sys::Uint8Array;

use pdbtbx::{open_pdb_raw, Context, PDBError, StrictnessLevel, PDB};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};

#[derive(Debug, Serialize, Deserialize)]
pub struct PDBErrorWrapper {
    level: String,
    short_description: String,
}

pub fn load_pdb_from_bytes(bytes: &Uint8Array) -> Result<PDB, PDBErrorWrapper> {
    let pdb_string = if bytes.is_null() {
        String::new()
    } else {
        let vec = bytes.to_vec();
        String::from_utf8(vec).unwrap()
    };

    let bytes = pdb_string.as_bytes().to_vec();
    let cursor = Cursor::new(bytes);
    let buf = BufReader::new(cursor);
    match open_pdb_raw(buf, Context::none(), StrictnessLevel::Loose) {
        Ok((pdb, _)) => Ok(pdb),
        Err(e) => {
            let collapsed_e = collapse_pdb_error(&e);
            Err(collapsed_e)
        }
    }
}

fn collapse_pdb_error(e: &[PDBError]) -> PDBErrorWrapper {
    let e: PDBError = e[0].clone();

    let pdb_error = PDBErrorWrapper {
        level: e.level().to_string(),
        short_description: e.short_description().to_string(),
    };

    pdb_error
}

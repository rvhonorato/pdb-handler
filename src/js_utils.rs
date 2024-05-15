use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;

pub fn hashmap_to_js_object(map: HashMap<String, Vec<String>>) -> JsValue {
    let result = js_sys::Object::new();
    for (key, value) in map {
        let js_array = js_sys::Array::new();
        for v in value {
            js_array.push(&JsValue::from_str(&v));
        }
        js_sys::Reflect::set(&result, &JsValue::from_str(&key), &js_array).unwrap();
    }
    result.into()
}

pub fn load_pdb_from_bytes(bytes: js_sys::Uint8Array) -> pdbtbx::PDB {
    let mut pdb_string: String = "".to_string();
    if !bytes.is_null() {
        let vec = bytes.to_vec();
        pdb_string = String::from_utf8(vec).unwrap();
    }

    let bytes = pdb_string.as_bytes().to_vec();
    let cursor = Cursor::new(bytes);
    let buf = BufReader::new(cursor);
    let (pdb, _errors) =
        pdbtbx::open_pdb_raw(buf, pdbtbx::Context::none(), pdbtbx::StrictnessLevel::Loose).unwrap();
    pdb
}

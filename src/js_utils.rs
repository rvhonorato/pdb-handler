use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;

pub fn hashmap_to_js_object<T>(map: HashMap<String, Vec<T>>) -> JsValue
where
    T: Into<JsValue> + Clone,
{
    let result = js_sys::Object::new();
    for (key, value) in map {
        let js_array = js_sys::Array::new();
        for v in value {
            js_array.push(&v.into());
        }
        js_sys::Reflect::set(&result, &JsValue::from_str(&key), &js_array).unwrap();
    }
    result.into()
}

pub fn load_pdb_from_bytes(bytes: &js_sys::Uint8Array) -> pdbtbx::PDB {
    let pdb_string = if bytes.is_null() {
        String::new()
    } else {
        let vec = bytes.to_vec();
        String::from_utf8(vec).unwrap()
    };

    let bytes = pdb_string.as_bytes().to_vec();
    let cursor = Cursor::new(bytes);
    let buf = BufReader::new(cursor);
    let (pdb, _errors) =
        pdbtbx::open_pdb_raw(buf, pdbtbx::Context::none(), pdbtbx::StrictnessLevel::Loose).unwrap();
    pdb
}

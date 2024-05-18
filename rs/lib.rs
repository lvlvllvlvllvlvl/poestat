use models::RSError;
use wasm_bindgen::prelude::wasm_bindgen;

mod data;
mod models;

#[wasm_bindgen]
pub fn hello() -> Result<usize, RSError> {
    Ok(data::TRIE.child_map.len())
}

use models::{RSError, Trie};
use wasm_bindgen::prelude::wasm_bindgen;

mod models;

static TRIE: &'static Trie = include!(concat!("../data/trie.rs"));

#[wasm_bindgen]
pub fn hello() -> Result<usize, RSError> {
    Ok(TRIE.child_map.len())
}

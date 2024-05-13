use std::sync::atomic::AtomicI32;

use build_trie::TrieBuilder;
use models::{RSError, Trie};
use wasm_bindgen::prelude::*;

mod build_trie;
mod handlers;
mod models;
mod stats;

#[wasm_bindgen]
pub fn hello(stat: &str, handlers: &str) -> Result<i32, RSError> {
    let builder = TrieBuilder {
        stats: serde_json::from_str(stat)?,
        handlers: serde_json::from_str(handlers)?,
        trie: Trie::default(),
        count: AtomicI32::new(0),
    };
    builder.build();
    Ok(builder.count.load(std::sync::atomic::Ordering::Relaxed))
}

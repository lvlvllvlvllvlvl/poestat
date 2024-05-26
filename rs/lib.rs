use poestat_static::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod parser;
mod result;

pub use parser::*;
pub use result::*;

#[wasm_bindgen]
pub fn parse(words: Vec<String>) -> JsValue {
    console_error_panic_hook::set_once();
    let parser = Parser::new(&words);
    let mut results = Vec::new();
    let mut index = 0;
    while index < words.len() {
        let found = parser.search(Some(&words[index]), index + 1, TRIE, true);

        if found.text.is_none() || found.count == 0 {
            index += 1;
        } else {
            index += found.count;
            results.push(ParseResult::from(found));
        }
    }
    serde_wasm_bindgen::to_value(&results).unwrap()
}

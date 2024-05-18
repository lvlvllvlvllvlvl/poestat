#![allow(clippy::all)]
use crate::models::Trie;
pub static TRIE: &Trie = include!(concat!(env!("OUT_DIR"), "/generated.rs"));

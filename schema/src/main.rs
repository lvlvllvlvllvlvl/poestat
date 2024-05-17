use std::{
    collections::HashMap,
    fs::File,
    path::Path,
};


fn main() {
    let file = File::open("data/trie.json").unwrap();
    let trie: TrieNode = serde_json::from_reader(file).unwrap();
    uneval_static::to_file(trie, Path::new("data/trie.rs")).expect("Write failed");
}

pub type TrieCell = Box<TrieNode>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrieNode {
    #[serde(default)]
    pub child_map: HashMap<String, TrieNode>,
    pub num_child: Option<TrieCell>,
    pub any_child: Option<TrieCell>,
    pub stat_child: Option<TrieCell>,
    pub stat_value: Option<i32>,
    pub stat_id: Option<String>,
    pub terminal: Option<String>,
}

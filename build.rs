use std::{collections::HashMap, env, fs::File, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=data/trie.json");
    let file = File::open("data/trie.json").unwrap();
    let trie: Trie = serde_json::from_reader(file).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs");
    let mut uneval = uneval_static::ser::Uneval::new(File::create(path).unwrap());
    uneval.add_mapping("Trie".into(), "&Trie".into());
    uneval.serialize(trie).expect("Write failed");
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Trie {
    #[serde(default)]
    child_map: HashMap<String, Trie>,
    num_child: Option<Box<Trie>>,
    any_child: Option<Box<Trie>>,
    stat_child: Option<Box<Trie>>,
    stat_value: Option<i32>,
    stat_id: Option<String>,
    terminal: Option<String>,
}

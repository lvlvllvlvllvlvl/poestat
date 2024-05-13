use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicI32, Ordering},
};

use crate::{
    handlers::{CanonicalLine, IntHandler, Noop, RelationalData},
    models::{Trie, TrieNode},
    stats::{Enum, Literal, LiteralType, NestedStat, Stat, Token},
};
use unicode_normalization::UnicodeNormalization;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Handler {
    Int(IntHandler),
    Relational(RelationalData),
    Canonical(CanonicalLine),
    Noop(Noop),
}

pub struct TrieBuilder {
    pub stats: HashMap<String, Stat>,
    pub handlers: HashMap<String, Handler>,
    pub trie: Trie,
    pub count: AtomicI32,
}

fn tokenise(str: &String) -> Vec<String> {
    let lower = str.trim().to_lowercase();
    let normal = lower.nfc().collect::<String>();
    normal.split_whitespace().map(|s| s.to_owned()).collect()
}

impl TrieBuilder {
    pub fn build(&self) -> TrieNode {
        for (stat_text, Stat { tokens, .. }) in &self.stats {
            let mut nodes = vec![self.trie.clone()];
            for token in tokens {
                let mut next = vec![];
                for node in nodes {
                    next.append(&mut self.add_to_trie(node.clone(), &token));
                }
                nodes = next;
            }
            for node in nodes {
                node.borrow_mut().terminal.get_or_insert(stat_text.clone());
            }
        }
        (*self.trie.borrow()).clone()
    }

    fn add_to_trie(&self, node: Trie, token: &Token) -> Vec<Trie> {
        self.count.fetch_add(1, Ordering::Relaxed);
        match token {
            Token::Literal(Literal { value, .. }) => {
                let mut next = node;
                for word in tokenise(value) {
                    let tmp;
                    {
                        let mut mutable = next.borrow_mut();
                        let map = mutable.child_map.get_or_insert_with(HashMap::new);
                        tmp = map.entry(word.into()).or_insert_with(Trie::default).clone()
                    }
                    next = tmp
                }
                vec![next]
            }
            Token::Number(_) => {
                let mut mutable = node.borrow_mut();
                vec![mutable.num_child.get_or_insert_with(Trie::default).clone()]
            }
            Token::Enum(Enum {
                stat_value_handler, ..
            }) => {
                let mut nodes = vec![];
                if let Some(handler) = self.handlers.get(stat_value_handler) {
                    if let Handler::Relational(RelationalData { values, .. }) = handler {
                        for (value, text) in values {
                            let stat_value = value.parse().ok();
                            for word in tokenise(text) {
                                let token = Token::Literal(Literal {
                                    type_: LiteralType::Literal,
                                    value: word.into(),
                                });
                                let added = &mut self.add_to_trie(node.clone(), &token);
                                for a in &mut *added {
                                    a.borrow_mut().stat_value = stat_value
                                }
                                nodes.append(added);
                            }
                        }
                    }
                }
                nodes
            }
            Token::Unknown(_) => {
                let mut mutable = node.borrow_mut();
                vec![mutable.any_child.get_or_insert_with(Trie::default).clone()]
            }
            Token::NestedStat(NestedStat { added_stat, .. }) => {
                let mut mutable = node.borrow_mut();
                let mut trie = TrieNode::default();
                trie.stat_id = Some(added_stat.into());
                let child = Rc::new(RefCell::new(trie));
                mutable.stat_child = Some(child.clone());
                vec![child]
            }
        }
    }
}

use poestat_static::{Trie, TRIE};
use regex::Regex;

use crate::result::IntermediateResult;

pub struct Parser<'w> {
    words: &'w Vec<String>,
    num_re: Regex,
}

impl<'w> Parser<'w> {
    pub fn new(words: &'w Vec<String>) -> Self {
        Parser {
            words,
            num_re: Regex::new(r"^(([+-])?\d*\.?\d+)(\D.*)?$").unwrap(),
        }
    }
    pub fn search(
        &self,
        word: Option<&String>,
        i: usize,
        node: &'static Trie,
        can_recurse: bool,
    ) -> IntermediateResult {
        #[cfg(test)]
        println!(
            "word: {:?}, i: {i}, children: {}, num: {}, stat: {}, any: {}",
            word,
            node.child_map.len(),
            node.num_child.is_some(),
            node.stat_child.is_some(),
            node.any_child.is_some()
        );
        if i > self.words.len() {
            return IntermediateResult {
                text: node.terminal,
                count: 0,
                values: node.stat_value.map_or_else(Vec::new, |v| vec![v as f32]),
                nested: None,
            };
        }

        let mut results = vec![];

        if let Some(v) = word.and_then(|w| node.child_map.get(w)) {
            let mut result = self.search(self.words.get(i), i + 1, v, can_recurse);
            if result.text.is_some() {
                if let Some(stat_value) = node.stat_value {
                    result.values.push(stat_value as f32);
                }
                result.count += 1;
                results.push(result);
            }
        }
        if let Some(num_child) = node.num_child {
            if let (Some(w), Some((prefix, num, suffix))) =
                (word, word.and_then(|w| self.match_number(w)))
            {
                if let Some(plus_node) = prefix.and_then(|_| node.child_map.get("+")) {
                    let result = self.search(Some(&w[1..].to_owned()), i, plus_node, can_recurse);
                    if result.text.is_some() {
                        results.push(result);
                    }
                }
                if let Some(Ok(num)) = num.map(|n| n.parse::<f32>()) {
                    if let Some(suffix) = suffix {
                        let mut result =
                            self.search(Some(&suffix.to_owned()), i, num_child, can_recurse);
                        if result.text.is_some() {
                            result.values.push(num);
                            results.push(result);
                        }
                    } else {
                        let mut result =
                            self.search(self.words.get(i), i + 1, num_child, can_recurse);
                        if result.text.is_some() {
                            result.values.push(num);
                            result.count += 1;
                            results.push(result);
                        }
                    }
                }
            }
        }

        if let Some(stat_child) = can_recurse.then_some(()).and(node.stat_child) {
            let nested = self.search(word, i, TRIE, false);
            if nested.text.is_some() {
                let j = i + nested.count;
                let mut result = self.search(self.words.get(j - 1), j, stat_child, can_recurse);
                if result.text.is_some() {
                    result.count += nested.count;
                    result.nested = Some(Box::new(nested));
                    results.push(result);
                }
            }
        }

        if let Some(any_child) = node.any_child {
            let mut result = self.search(self.words.get(i), i + 1, any_child, can_recurse);
            if result.text.is_some() {
                result.count += 1;
                results.push(result);
            }
        }

        results
            .into_iter()
            .max_by(|l, r| {
                l.count
                    .cmp(&r.count)
                    .then_with(|| l.values.len().cmp(&r.values.len()))
                    .then_with(|| l.text.cmp(&r.text))
            })
            .unwrap_or_else(|| IntermediateResult {
                text: node.terminal,
                count: 0,
                values: node.stat_value.map_or_else(Vec::new, |v| vec![v as f32]),
                nested: None,
            })
    }
    fn match_number<'v>(
        &self,
        val: &'v str,
    ) -> Option<(Option<&'v str>, Option<&'v str>, Option<&'v str>)> {
        self.num_re.captures(val).map(move |m| {
            (
                m.get(2).map(|m| m.as_str()),
                m.get(1).map(|m| m.as_str()),
                m.get(3).map(|m| m.as_str()),
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_num_re() {
        let v = vec![];
        let p = Parser::new(&v);
        assert_eq!(p.match_number("12"), Some((None, Some("12"), None)));
        assert_eq!(p.match_number("none"), None);
        assert_eq!(p.match_number("+12"), Some((Some("+"), Some("+12"), None)));
        assert_eq!(
            p.match_number("12suf"),
            Some((None, Some("12"), Some("suf")))
        );
    }

    #[test]
    fn test_parser() {
        let v = vec![
            "87%".to_string(),
            "increased".to_string(),
            "evasion".to_string(),
            "rating".to_string(),
        ];
        let p = Parser::new(&v);
        assert_eq!(
            p.search(Some(&v[0]), 1, TRIE, true),
            IntermediateResult {
                text: Some("{0}% increased Evasion Rating"),
                count: 4,
                values: vec![87.0],
                nested: None
            }
        );
    }

    #[test]
    fn test_recursive() {
        let v = [
            "added",
            "small",
            "passive",
            "skills",
            "grant:",
            "12%",
            "increased",
            "trap",
            "damage",
            "added",
            "small",
            "passive",
            "skills",
            "grant:",
            "12%",
            "increased",
            "mine",
            "damage",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect();
        let p = Parser::new(&v);
        assert_eq!(
            p.search(Some(&v[0]), 1, TRIE, true),
            IntermediateResult {
                text: Some("Added Small Passive Skills grant: {0}"),
                count: 9,
                values: vec![],
                nested: Some(Box::new(IntermediateResult {
                    text: Some("{0}% increased Trap Damage"),
                    count: 4,
                    values: vec![12.0],
                    nested: None
                }))
            }
        );
    }

    #[test]
    fn test_enum() {
        let v = [
            "allocates",
            "gratuitous",
            "violence",
            "if",
            "you",
            "have",
            "the",
            "matching",
            "modifier",
            "on",
            "forbidden",
            "flame",
        ]
        .iter()
        .map(|t| t.to_string())
        .collect();
        let p = Parser::new(&v);
        assert_eq!(
            p.search(Some(&v[0]), 1, TRIE, true),
            IntermediateResult {
                text: Some("Allocates {0} if you have the matching modifier on Forbidden Flame"),
                count: 12,
                values: vec![27864.0],
                nested: None
            }
        );
    }
}

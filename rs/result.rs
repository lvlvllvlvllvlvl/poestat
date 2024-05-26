use poestat_static::{Token, HANDLERS, IMPLIED, TOKENS};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../ts/types/rs/")]
pub struct ParsedStat {
    /// The id of the row in Stats.dat
    pub id: String,
    /// The template variable in the stat text (e.g. {0})
    pub index: i32,
    /// The value as read from the mod that was parsed
    pub parsed_value: f32,
    /// Approximate value of the raw value of this stat as per the game files
    /// (for instance if the game stores the stat as ms but displays it as seconds,
    /// this will be the ms value).
    pub base_value: f32,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../ts/types/rs/")]
pub struct ParseResult {
    pub text: String,
    pub stats: Vec<ParsedStat>,
}

impl ParseResult {
    pub fn from(mut input: IntermediateResult) -> ParseResult {
        let mut result = ParseResult {
            text: input.text.unwrap_or_default().to_string(),
            stats: Vec::new(),
        };
        result.process(&mut input);
        result
    }
    fn process(&mut self, input: &mut IntermediateResult) {
        if let Some(tokens) = input.text.and_then(|i| TOKENS.get(i)) {
            for t in tokens.iter() {
                match t {
                    Token::Number {
                        index,
                        stat,
                        stat_value_handlers,
                    } => {
                        if let Some(parsed_value) = input.values.pop() {
                            let mut base_value = parsed_value;
                            for h in stat_value_handlers.iter().rev() {
                                if let Some(handler) = HANDLERS.get(h) {
                                    base_value = handler.reverse(base_value);
                                }
                            }
                            self.stats.push(ParsedStat {
                                id: stat.to_string(),
                                index: *index,
                                parsed_value,
                                base_value,
                            })
                        }
                    }
                    Token::Enum { index, stat, .. } => {
                        if let Some(parsed_value) = input.values.pop() {
                            self.stats.push(ParsedStat {
                                id: stat.to_string(),
                                index: *index,
                                parsed_value,
                                base_value: parsed_value,
                            })
                        }
                    }
                    Token::NestedStat { added_stat } => {
                        if let Some(ref mut nested) = input.nested {
                            let nested = nested.as_mut();
                            if !added_stat.is_empty() {
                                self.stats.push(ParsedStat {
                                    id: added_stat.to_string(),
                                    index: -2,
                                    base_value: 1.0,
                                    parsed_value: 1.0,
                                });
                            }
                            if let Some(text) = nested.text {
                                self.text = self.text.replace("{0}", text);
                            }
                            self.process(nested)
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(implied) = input.text.and_then(|i| IMPLIED.get(i)) {
            self.stats
                .extend(
                    implied
                        .into_iter()
                        .filter(|(_, v)| **v != 0)
                        .map(|(id, base_value)| ParsedStat {
                            id: id.to_string(),
                            index: -1,
                            base_value: *base_value as f32,
                            parsed_value: *base_value as f32,
                        }),
                );
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IntermediateResult {
    pub count: usize,
    pub values: Vec<f32>,
    pub text: Option<&'static str>,
    pub nested: Option<Box<IntermediateResult>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_result() {
        let input = IntermediateResult {
            text: Some("Added Small Passive Skills grant: {0}"),
            count: 9,
            values: vec![],
            nested: Some(Box::new(IntermediateResult {
                text: Some("{0}% increased Trap Damage"),
                count: 4,
                values: vec![12.0],
                nested: None,
            })),
        };
        assert_eq!(
            ParseResult::from(input),
            ParseResult {
                text: "Added Small Passive Skills grant: {0}% increased Trap Damage".to_string(),
                stats: vec![ParsedStat {
                    id: "trap_damage_+%".into(),
                    index: 0,
                    parsed_value: 12.0,
                    base_value: 12.0
                }]
            }
        );
    }
}

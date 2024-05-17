use std::{error::Error, panic::Location};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, serde::Serialize, ts_rs::TS)]
#[ts(export, export_to = "../ts/types/rs/")]
pub struct ParseResult {
    text: String,
    stats: Vec<ParsedStat>,
}

#[wasm_bindgen]
#[derive(Clone, serde::Serialize, ts_rs::TS)]
#[ts(export, export_to = "../ts/types/rs/")]
pub struct ParsedStat {
    /// The id of the row in Stats.dat
    id: String,
    /// The template variable in the stat text (e.g. {0})
    index: i32,
    /// The value as read from the mod that was parsed
    parsed_value: f64,
    /// Approximate value of the raw value of this stat as per the game files
    /// (for instance if the game stores the stat as ms but displays it as seconds,
    /// this will be the ms value).
    base_value: f64,
}

pub struct Trie {
    pub child_map: phf::Map<&'static str, &'static Trie>,
    pub num_child: Option<&'static Trie>,
    pub any_child: Option<&'static Trie>,
    pub stat_child: Option<&'static Trie>,
    pub stat_value: Option<i32>,
    pub stat_id: Option<&'static str>,
    pub terminal: Option<&'static str>,
}

pub struct IntermediateResult {
    text: Option<String>,
    count: i32,
    values: Vec<i32>,
    nested: Option<Box<IntermediateResult>>,
}

#[wasm_bindgen]
#[derive(Clone, serde::Serialize, ts_rs::TS)]
#[ts(export, export_to = "../ts/types/rs/")]
pub struct RSError {
    message: String,
    location: Option<String>,
    cause: Vec<RSError>,
    debug: Option<String>,
}

#[wasm_bindgen]
impl RSError {
    pub fn get_message(&self) -> String {
        return self.message.clone();
    }
    pub fn get_location(&self) -> Option<String> {
        return self.location.clone();
    }
    pub fn get_cause(&self) -> Vec<RSError> {
        return self.cause.clone();
    }
    pub fn get_debug(&self) -> Option<String> {
        return self.debug.clone();
    }
}

impl<E: Error> From<E> for RSError {
    #[track_caller]
    fn from(e: E) -> RSError {
        wrap(e, Some(Location::caller().to_string()))
    }
}

pub fn wrap<E: Error>(e: E, location: Option<String>) -> RSError {
    let mut cause: Vec<RSError> = Vec::new();
    let mut next = e.source();
    while let Some(source) = next {
        cause.push(wrap(source, None));
        next = source.source();
    }
    RSError {
        message: e.to_string().into(),
        location,
        debug: cfg!(debug_assertions).then(|| format!("{e:#?}").into()),
        cause,
    }
}

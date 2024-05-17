#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Integer stat values mapped to rows of a dat file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Integer stat values mapped to rows of a dat file.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"index\","]
#[doc = "    \"stat\","]
#[doc = "    \"stat_value_handler\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"index\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"stat\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stat_value_handler\": {"]
#[doc = "      \"description\": \"Reference to the entry in stat_value_handlers.json where the enum values can be found.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"enum\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Enum {
    pub index: i64,
    pub stat: String,
    #[doc = "Reference to the entry in stat_value_handlers.json where the enum values can be found."]
    pub stat_value_handler: String,
    #[serde(rename = "type")]
    pub type_: EnumType,
}
impl From<&Enum> for Enum {
    fn from(value: &Enum) -> Self {
        value.clone()
    }
}
impl Enum {
    pub fn builder() -> builder::Enum {
        Default::default()
    }
}
#[doc = "EnumType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"enum\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EnumType {
    #[serde(rename = "enum")]
    Enum,
}
impl From<&EnumType> for EnumType {
    fn from(value: &EnumType) -> Self {
        value.clone()
    }
}
impl ToString for EnumType {
    fn to_string(&self) -> String {
        match *self {
            Self::Enum => "enum".to_string(),
        }
    }
}
impl std::str::FromStr for EnumType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "enum" => Ok(Self::Enum),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EnumType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EnumType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EnumType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Literal"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"literal\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Literal {
    #[serde(rename = "type")]
    pub type_: LiteralType,
    pub value: String,
}
impl From<&Literal> for Literal {
    fn from(value: &Literal) -> Self {
        value.clone()
    }
}
impl Literal {
    pub fn builder() -> builder::Literal {
        Default::default()
    }
}
#[doc = "LiteralType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"literal\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiteralType {
    #[serde(rename = "literal")]
    Literal,
}
impl From<&LiteralType> for LiteralType {
    fn from(value: &LiteralType) -> Self {
        value.clone()
    }
}
impl ToString for LiteralType {
    fn to_string(&self) -> String {
        match *self {
            Self::Literal => "literal".to_string(),
        }
    }
}
impl std::str::FromStr for LiteralType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "literal" => Ok(Self::Literal),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LiteralType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiteralType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiteralType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Matches any other stat - used by 'While a ... is in your Presence, <stat>' mods"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Matches any other stat - used by 'While a ... is in your Presence, <stat>' mods\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"added_stat\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"added_stat\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"nested\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NestedStat {
    pub added_stat: String,
    #[serde(rename = "type")]
    pub type_: NestedStatType,
}
impl From<&NestedStat> for NestedStat {
    fn from(value: &NestedStat) -> Self {
        value.clone()
    }
}
impl NestedStat {
    pub fn builder() -> builder::NestedStat {
        Default::default()
    }
}
#[doc = "NestedStatType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"nested\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NestedStatType {
    #[serde(rename = "nested")]
    Nested,
}
impl From<&NestedStatType> for NestedStatType {
    fn from(value: &NestedStatType) -> Self {
        value.clone()
    }
}
impl ToString for NestedStatType {
    fn to_string(&self) -> String {
        match *self {
            Self::Nested => "nested".to_string(),
        }
    }
}
impl std::str::FromStr for NestedStatType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "nested" => Ok(Self::Nested),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NestedStatType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NestedStatType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NestedStatType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Matches a number of the form /[+-]?\\d*\\.?\\d+/"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Matches a number of the form /[+-]?\\\\d*\\\\.?\\\\d+/\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"index\","]
#[doc = "    \"stat\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"index\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"stat\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stat_value_handlers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"number\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Number {
    pub index: i64,
    pub stat: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stat_value_handlers: Vec<String>,
    #[serde(rename = "type")]
    pub type_: NumberType,
}
impl From<&Number> for Number {
    fn from(value: &Number) -> Self {
        value.clone()
    }
}
impl Number {
    pub fn builder() -> builder::Number {
        Default::default()
    }
}
#[doc = "NumberType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NumberType {
    #[serde(rename = "number")]
    Number,
}
impl From<&NumberType> for NumberType {
    fn from(value: &NumberType) -> Self {
        value.clone()
    }
}
impl ToString for NumberType {
    fn to_string(&self) -> String {
        match *self {
            Self::Number => "number".to_string(),
        }
    }
}
impl std::str::FromStr for NumberType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NumberType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NumberType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NumberType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Stat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"files\","]
#[doc = "    \"generated_name\","]
#[doc = "    \"tokens\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"files\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"generated_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"implied_stats\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Token\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stat {
    pub files: Vec<String>,
    pub generated_name: String,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub implied_stats: std::collections::HashMap<String, i64>,
    pub tokens: Vec<Token>,
}
impl From<&Stat> for Stat {
    fn from(value: &Stat) -> Self {
        value.clone()
    }
}
impl Stat {
    pub fn builder() -> builder::Stat {
        Default::default()
    }
}
#[doc = "Token"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Literal\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Enum\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Unknown\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/NestedStat\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Token {
    Literal(Literal),
    Number(Number),
    Enum(Enum),
    Unknown(Unknown),
    NestedStat(NestedStat),
}
impl From<&Token> for Token {
    fn from(value: &Token) -> Self {
        value.clone()
    }
}
impl From<Literal> for Token {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}
impl From<Number> for Token {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}
impl From<Enum> for Token {
    fn from(value: Enum) -> Self {
        Self::Enum(value)
    }
}
impl From<Unknown> for Token {
    fn from(value: Unknown) -> Self {
        Self::Unknown(value)
    }
}
impl From<NestedStat> for Token {
    fn from(value: NestedStat) -> Self {
        Self::NestedStat(value)
    }
}
#[doc = "How to handle this item is not known. At the current time only the 'sells for an additional unique <base item type>' crucible mod has an unknown token."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"How to handle this item is not known. At the current time only the 'sells for an additional unique <base item type>' crucible mod has an unknown token.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"index\","]
#[doc = "    \"stat\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"index\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"stat\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stat_value_handler\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"unknown\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Unknown {
    pub index: i64,
    pub stat: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stat_value_handler: Option<String>,
    #[serde(rename = "type")]
    pub type_: UnknownType,
}
impl From<&Unknown> for Unknown {
    fn from(value: &Unknown) -> Self {
        value.clone()
    }
}
impl Unknown {
    pub fn builder() -> builder::Unknown {
        Default::default()
    }
}
#[doc = "UnknownType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UnknownType {
    #[serde(rename = "unknown")]
    Unknown,
}
impl From<&UnknownType> for UnknownType {
    fn from(value: &UnknownType) -> Self {
        value.clone()
    }
}
impl ToString for UnknownType {
    fn to_string(&self) -> String {
        match *self {
            Self::Unknown => "unknown".to_string(),
        }
    }
}
impl std::str::FromStr for UnknownType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for UnknownType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UnknownType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UnknownType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Enum {
        index: Result<i64, String>,
        stat: Result<String, String>,
        stat_value_handler: Result<String, String>,
        type_: Result<super::EnumType, String>,
    }
    impl Default for Enum {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                stat: Err("no value supplied for stat".to_string()),
                stat_value_handler: Err("no value supplied for stat_value_handler".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Enum {
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn stat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.stat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stat: {}", e));
            self
        }
        pub fn stat_value_handler<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.stat_value_handler = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for stat_value_handler: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::EnumType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Enum> for super::Enum {
        type Error = super::error::ConversionError;
        fn try_from(value: Enum) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                stat: value.stat?,
                stat_value_handler: value.stat_value_handler?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Enum> for Enum {
        fn from(value: super::Enum) -> Self {
            Self {
                index: Ok(value.index),
                stat: Ok(value.stat),
                stat_value_handler: Ok(value.stat_value_handler),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Literal {
        type_: Result<super::LiteralType, String>,
        value: Result<String, String>,
    }
    impl Default for Literal {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Literal {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LiteralType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Literal> for super::Literal {
        type Error = super::error::ConversionError;
        fn try_from(value: Literal) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Literal> for Literal {
        fn from(value: super::Literal) -> Self {
            Self {
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NestedStat {
        added_stat: Result<String, String>,
        type_: Result<super::NestedStatType, String>,
    }
    impl Default for NestedStat {
        fn default() -> Self {
            Self {
                added_stat: Err("no value supplied for added_stat".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl NestedStat {
        pub fn added_stat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.added_stat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for added_stat: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NestedStatType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NestedStat> for super::NestedStat {
        type Error = super::error::ConversionError;
        fn try_from(value: NestedStat) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                added_stat: value.added_stat?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::NestedStat> for NestedStat {
        fn from(value: super::NestedStat) -> Self {
            Self {
                added_stat: Ok(value.added_stat),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Number {
        index: Result<i64, String>,
        stat: Result<String, String>,
        stat_value_handlers: Result<Vec<String>, String>,
        type_: Result<super::NumberType, String>,
    }
    impl Default for Number {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                stat: Err("no value supplied for stat".to_string()),
                stat_value_handlers: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Number {
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn stat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.stat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stat: {}", e));
            self
        }
        pub fn stat_value_handlers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.stat_value_handlers = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for stat_value_handlers: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NumberType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Number> for super::Number {
        type Error = super::error::ConversionError;
        fn try_from(value: Number) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                stat: value.stat?,
                stat_value_handlers: value.stat_value_handlers?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Number> for Number {
        fn from(value: super::Number) -> Self {
            Self {
                index: Ok(value.index),
                stat: Ok(value.stat),
                stat_value_handlers: Ok(value.stat_value_handlers),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Stat {
        files: Result<Vec<String>, String>,
        generated_name: Result<String, String>,
        implied_stats: Result<std::collections::HashMap<String, i64>, String>,
        tokens: Result<Vec<super::Token>, String>,
    }
    impl Default for Stat {
        fn default() -> Self {
            Self {
                files: Err("no value supplied for files".to_string()),
                generated_name: Err("no value supplied for generated_name".to_string()),
                implied_stats: Ok(Default::default()),
                tokens: Err("no value supplied for tokens".to_string()),
            }
        }
    }
    impl Stat {
        pub fn files<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for files: {}", e));
            self
        }
        pub fn generated_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.generated_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for generated_name: {}", e));
            self
        }
        pub fn implied_stats<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.implied_stats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for implied_stats: {}", e));
            self
        }
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Token>>,
            T::Error: std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Stat> for super::Stat {
        type Error = super::error::ConversionError;
        fn try_from(value: Stat) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                files: value.files?,
                generated_name: value.generated_name?,
                implied_stats: value.implied_stats?,
                tokens: value.tokens?,
            })
        }
    }
    impl From<super::Stat> for Stat {
        fn from(value: super::Stat) -> Self {
            Self {
                files: Ok(value.files),
                generated_name: Ok(value.generated_name),
                implied_stats: Ok(value.implied_stats),
                tokens: Ok(value.tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Unknown {
        index: Result<i64, String>,
        stat: Result<String, String>,
        stat_value_handler: Result<Option<String>, String>,
        type_: Result<super::UnknownType, String>,
    }
    impl Default for Unknown {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                stat: Err("no value supplied for stat".to_string()),
                stat_value_handler: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Unknown {
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn stat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.stat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stat: {}", e));
            self
        }
        pub fn stat_value_handler<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.stat_value_handler = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for stat_value_handler: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UnknownType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Unknown> for super::Unknown {
        type Error = super::error::ConversionError;
        fn try_from(value: Unknown) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                stat: value.stat?,
                stat_value_handler: value.stat_value_handler?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Unknown> for Unknown {
        fn from(value: super::Unknown) -> Self {
            Self {
                index: Ok(value.index),
                stat: Ok(value.stat),
                stat_value_handler: Ok(value.stat_value_handler),
                type_: Ok(value.type_),
            }
        }
    }
}

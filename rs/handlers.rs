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
#[doc = "CanonicalLine"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CanonicalLine\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalLine {
    #[serde(rename = "type")]
    pub type_: CanonicalLineType,
}
impl From<&CanonicalLine> for CanonicalLine {
    fn from(value: &CanonicalLine) -> Self {
        value.clone()
    }
}
impl CanonicalLine {
    pub fn builder() -> builder::CanonicalLine {
        Default::default()
    }
}
#[doc = "CanonicalLineType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CanonicalLineType {
    #[serde(rename = "string")]
    String,
}
impl From<&CanonicalLineType> for CanonicalLineType {
    fn from(value: &CanonicalLineType) -> Self {
        value.clone()
    }
}
impl ToString for CanonicalLineType {
    fn to_string(&self) -> String {
        match *self {
            Self::String => "string".to_string(),
        }
    }
}
impl std::str::FromStr for CanonicalLineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CanonicalLineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CanonicalLineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CanonicalLineType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IntHandler"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"IntHandler\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addend\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"divisor\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"fixed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"multiplier\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"precision\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"int\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntHandler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addend: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(rename = "type")]
    pub type_: IntHandlerType,
}
impl From<&IntHandler> for IntHandler {
    fn from(value: &IntHandler) -> Self {
        value.clone()
    }
}
impl IntHandler {
    pub fn builder() -> builder::IntHandler {
        Default::default()
    }
}
#[doc = "IntHandlerType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"int\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IntHandlerType {
    #[serde(rename = "int")]
    Int,
}
impl From<&IntHandlerType> for IntHandlerType {
    fn from(value: &IntHandlerType) -> Self {
        value.clone()
    }
}
impl ToString for IntHandlerType {
    fn to_string(&self) -> String {
        match *self {
            Self::Int => "int".to_string(),
        }
    }
}
impl std::str::FromStr for IntHandlerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "int" => Ok(Self::Int),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IntHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntHandlerType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Noop"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Noop\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"noop\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Noop {
    #[serde(rename = "type")]
    pub type_: NoopType,
}
impl From<&Noop> for Noop {
    fn from(value: &Noop) -> Self {
        value.clone()
    }
}
impl Noop {
    pub fn builder() -> builder::Noop {
        Default::default()
    }
}
#[doc = "NoopType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"noop\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NoopType {
    #[serde(rename = "noop")]
    Noop,
}
impl From<&NoopType> for NoopType {
    fn from(value: &NoopType) -> Self {
        value.clone()
    }
}
impl ToString for NoopType {
    fn to_string(&self) -> String {
        match *self {
            Self::Noop => "noop".to_string(),
        }
    }
}
impl std::str::FromStr for NoopType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "noop" => Ok(Self::Noop),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NoopType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NoopType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NoopType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Predicate"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Predicate\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"column\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Predicate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl From<&Predicate> for Predicate {
    fn from(value: &Predicate) -> Self {
        value.clone()
    }
}
impl Predicate {
    pub fn builder() -> builder::Predicate {
        Default::default()
    }
}
#[doc = "RelationalData"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"RelationalData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dat_file\","]
#[doc = "    \"type\","]
#[doc = "    \"value_column\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dat_file\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"index_column\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"predicate\": {"]
#[doc = "      \"$ref\": \"#/definitions/Predicate\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"relational\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value_column\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"Map from integer stat values to text associated with that value\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationalData {
    pub dat_file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(rename = "type")]
    pub type_: RelationalDataType,
    pub value_column: String,
    #[doc = "Map from integer stat values to text associated with that value"]
    pub values: std::collections::HashMap<String, String>,
}
impl From<&RelationalData> for RelationalData {
    fn from(value: &RelationalData) -> Self {
        value.clone()
    }
}
impl RelationalData {
    pub fn builder() -> builder::RelationalData {
        Default::default()
    }
}
#[doc = "RelationalDataType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"relational\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RelationalDataType {
    #[serde(rename = "relational")]
    Relational,
}
impl From<&RelationalDataType> for RelationalDataType {
    fn from(value: &RelationalDataType) -> Self {
        value.clone()
    }
}
impl ToString for RelationalDataType {
    fn to_string(&self) -> String {
        match *self {
            Self::Relational => "relational".to_string(),
        }
    }
}
impl std::str::FromStr for RelationalDataType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "relational" => Ok(Self::Relational),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RelationalDataType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RelationalDataType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RelationalDataType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CanonicalLine {
        type_: Result<super::CanonicalLineType, String>,
    }
    impl Default for CanonicalLine {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CanonicalLine {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CanonicalLineType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CanonicalLine> for super::CanonicalLine {
        type Error = super::error::ConversionError;
        fn try_from(value: CanonicalLine) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl From<super::CanonicalLine> for CanonicalLine {
        fn from(value: super::CanonicalLine) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntHandler {
        addend: Result<Option<f64>, String>,
        divisor: Result<Option<f64>, String>,
        fixed: Result<Option<bool>, String>,
        multiplier: Result<Option<f64>, String>,
        precision: Result<Option<i64>, String>,
        type_: Result<super::IntHandlerType, String>,
    }
    impl Default for IntHandler {
        fn default() -> Self {
            Self {
                addend: Ok(Default::default()),
                divisor: Ok(Default::default()),
                fixed: Ok(Default::default()),
                multiplier: Ok(Default::default()),
                precision: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IntHandler {
        pub fn addend<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.addend = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addend: {}", e));
            self
        }
        pub fn divisor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.divisor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for divisor: {}", e));
            self
        }
        pub fn fixed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.fixed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fixed: {}", e));
            self
        }
        pub fn multiplier<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.multiplier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multiplier: {}", e));
            self
        }
        pub fn precision<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.precision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for precision: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntHandlerType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IntHandler> for super::IntHandler {
        type Error = super::error::ConversionError;
        fn try_from(value: IntHandler) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                addend: value.addend?,
                divisor: value.divisor?,
                fixed: value.fixed?,
                multiplier: value.multiplier?,
                precision: value.precision?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IntHandler> for IntHandler {
        fn from(value: super::IntHandler) -> Self {
            Self {
                addend: Ok(value.addend),
                divisor: Ok(value.divisor),
                fixed: Ok(value.fixed),
                multiplier: Ok(value.multiplier),
                precision: Ok(value.precision),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Noop {
        type_: Result<super::NoopType, String>,
    }
    impl Default for Noop {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Noop {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NoopType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Noop> for super::Noop {
        type Error = super::error::ConversionError;
        fn try_from(value: Noop) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl From<super::Noop> for Noop {
        fn from(value: super::Noop) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Predicate {
        column: Result<Option<String>, String>,
        value: Result<Option<i64>, String>,
    }
    impl Default for Predicate {
        fn default() -> Self {
            Self {
                column: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Predicate {
        pub fn column<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.column = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for column: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Predicate> for super::Predicate {
        type Error = super::error::ConversionError;
        fn try_from(value: Predicate) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                column: value.column?,
                value: value.value?,
            })
        }
    }
    impl From<super::Predicate> for Predicate {
        fn from(value: super::Predicate) -> Self {
            Self {
                column: Ok(value.column),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RelationalData {
        dat_file: Result<String, String>,
        index_column: Result<Option<String>, String>,
        predicate: Result<Option<super::Predicate>, String>,
        type_: Result<super::RelationalDataType, String>,
        value_column: Result<String, String>,
        values: Result<std::collections::HashMap<String, String>, String>,
    }
    impl Default for RelationalData {
        fn default() -> Self {
            Self {
                dat_file: Err("no value supplied for dat_file".to_string()),
                index_column: Ok(Default::default()),
                predicate: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value_column: Err("no value supplied for value_column".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl RelationalData {
        pub fn dat_file<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.dat_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dat_file: {}", e));
            self
        }
        pub fn index_column<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.index_column = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index_column: {}", e));
            self
        }
        pub fn predicate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Predicate>>,
            T::Error: std::fmt::Display,
        {
            self.predicate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for predicate: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RelationalDataType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value_column<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.value_column = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_column: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, String>>,
            T::Error: std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RelationalData> for super::RelationalData {
        type Error = super::error::ConversionError;
        fn try_from(value: RelationalData) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                dat_file: value.dat_file?,
                index_column: value.index_column?,
                predicate: value.predicate?,
                type_: value.type_?,
                value_column: value.value_column?,
                values: value.values?,
            })
        }
    }
    impl From<super::RelationalData> for RelationalData {
        fn from(value: super::RelationalData) -> Self {
            Self {
                dat_file: Ok(value.dat_file),
                index_column: Ok(value.index_column),
                predicate: Ok(value.predicate),
                type_: Ok(value.type_),
                value_column: Ok(value.value_column),
                values: Ok(value.values),
            }
        }
    }
}

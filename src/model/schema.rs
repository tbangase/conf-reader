use std::collections::HashMap;

use serde_json::Value;
use strum::{Display, EnumString};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Invalid value type: {0}")]
    InvalidValueType(String),
}

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq)]
pub struct Schema(HashMap<String, ValueType>);

#[derive(EnumString, PartialEq, Eq, Display, Debug)]
pub enum ValueType {
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl Default for Schema {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl Schema {
    pub fn new(hash_map: HashMap<String, ValueType>) -> Self {
        Schema(hash_map)
    }

    // FIXME: Consider for doubled schema rule
    pub fn add_rule(
        mut self,
        key: impl ToString,
        value_type: impl ToString,
    ) -> Result<Self, SchemaError> {
        self.0.insert(
            key.to_string(),
            value_type
                .to_string()
                .parse()
                .map_err(|_| SchemaError::InvalidValueType(value_type.to_string()))?,
        );
        Ok(self)
    }

    /// Validation is to check the key_path and value pair is inclue in Schema
    /// with matched value
    pub fn is_valid(&self, key_path: &[impl ToString], value: &Value) -> bool {
        let key = key_path
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(".");
        if let Some(value_type) = self.0.get(&key) {
            match value {
                Value::Array(_) => value_type == &ValueType::Array,
                Value::Bool(_) => value_type == &ValueType::Bool,
                Value::Number(_) => value_type == &ValueType::Number,
                Value::Object(_) => value_type == &ValueType::Object,
                Value::String(_) => value_type == &ValueType::String,
                _ => false,
            }
        } else {
            false
        }
    }
}

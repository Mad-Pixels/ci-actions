use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::number::Number;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Boolean,
    String,
    Number,
    Object,
    Array,
    Null,
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValueType::String => "String",
            ValueType::Number => "Number",
            ValueType::Boolean => "Boolean",
            ValueType::Array => "Array",
            ValueType::Object => "Object",
            ValueType::Null => "Null",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawValue {
    Object(HashMap<String, RawValue>),
    Array(Vec<RawValue>),
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

impl RawValue {
    pub fn value_type(&self) -> ValueType {
        match self {
            RawValue::Boolean(_) => ValueType::Boolean,
            RawValue::String(_) => ValueType::String,
            RawValue::Number(_) => ValueType::Number,
            RawValue::Object(_) => ValueType::Object,
            RawValue::Array(_) => ValueType::Array,
            RawValue::Null => ValueType::Null,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, RawValue::Boolean(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, RawValue::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, RawValue::Number(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, RawValue::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, RawValue::Array(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, RawValue::Null)
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            RawValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            RawValue::Boolean(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[RawValue]> {
        match self {
            RawValue::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, RawValue>> {
        match self {
            RawValue::Object(o) => Some(o),
            _ => None,
        }
    }
}

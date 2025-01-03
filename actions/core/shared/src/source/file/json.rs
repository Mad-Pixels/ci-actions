use super::SourceError;
use crate::types::{Number, RawValue};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub(crate) fn parse(content: &str) -> Result<HashMap<String, RawValue>, SourceError> {
    let json: JsonValue = serde_json::from_str(content)
        .map_err(|e| SourceError::InvalidFormat(format!("Invalid JSON: {}", e)))?;

    convert_json_value(json)
}

pub(crate) fn serialize(values: &HashMap<String, RawValue>) -> Result<String, SourceError> {
    let json_value = convert_to_json_value(values)?;
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| SourceError::InvalidFormat(format!("Failed to serialize JSON: {}", e)))
}

fn convert_json_value(value: JsonValue) -> Result<HashMap<String, RawValue>, SourceError> {
    match value {
        JsonValue::Object(map) => {
            let mut result = HashMap::new();
            for (key, value) in map {
                result.insert(key, json_to_raw_value(value)?);
            }
            Ok(result)
        }
        _ => Err(SourceError::InvalidFormat("Root must be an object".into())),
    }
}

fn json_to_raw_value(value: JsonValue) -> Result<RawValue, SourceError> {
    match value {
        JsonValue::Null => Ok(RawValue::Null),
        JsonValue::Bool(b) => Ok(RawValue::Boolean(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(RawValue::Number(Number::Integer(i)))
            } else if let Some(f) = n.as_f64() {
                Ok(RawValue::Number(Number::Float(f)))
            } else {
                Err(SourceError::InvalidFormat("Invalid number".into()))
            }
        }
        JsonValue::String(s) => Ok(RawValue::String(s)),
        JsonValue::Array(arr) => {
            let values: Result<Vec<_>, _> = arr.into_iter().map(json_to_raw_value).collect();
            Ok(RawValue::Array(values?))
        }
        JsonValue::Object(map) => {
            let mut result = HashMap::new();
            for (key, value) in map {
                result.insert(key, json_to_raw_value(value)?);
            }
            Ok(RawValue::Object(result))
        }
    }
}

fn convert_to_json_value(values: &HashMap<String, RawValue>) -> Result<JsonValue, SourceError> {
    let mut map = serde_json::Map::new();
    for (key, value) in values {
        map.insert(key.clone(), raw_value_to_json(value)?);
    }
    Ok(JsonValue::Object(map))
}

fn raw_value_to_json(value: &RawValue) -> Result<JsonValue, SourceError> {
    match value {
        RawValue::Null => Ok(JsonValue::Null),
        RawValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
        RawValue::Number(Number::Integer(i)) => Ok(JsonValue::Number((*i).into())),
        RawValue::Number(Number::Float(f)) => {
            let n = serde_json::Number::from_f64(*f)
                .ok_or_else(|| SourceError::InvalidFormat("Invalid float value".into()))?;
            Ok(JsonValue::Number(n))
        }
        RawValue::String(s) => Ok(JsonValue::String(s.clone())),
        RawValue::Array(arr) => {
            let values: Result<Vec<_>, _> = arr.iter().map(raw_value_to_json).collect();
            Ok(JsonValue::Array(values?))
        }
        RawValue::Object(map) => {
            let mut result = serde_json::Map::new();
            for (key, value) in map {
                result.insert(key.clone(), raw_value_to_json(value)?);
            }
            Ok(JsonValue::Object(result))
        }
    }
}

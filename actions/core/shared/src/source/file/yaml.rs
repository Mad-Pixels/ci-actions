use super::SourceError;
use crate::types::{Number, RawValue};
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;

pub(crate) fn parse(content: &str) -> Result<HashMap<String, RawValue>, SourceError> {
    let yaml: YamlValue = serde_yaml::from_str(content)
        .map_err(|e| SourceError::InvalidFormat(format!("Invalid YAML: {}", e)))?;

    convert_yaml_value(yaml)
}

pub(crate) fn serialize(values: &HashMap<String, RawValue>) -> Result<String, SourceError> {
    let yaml_value = convert_to_yaml_value(values)?;
    serde_yaml::to_string(&yaml_value)
        .map_err(|e| SourceError::InvalidFormat(format!("Failed to serialize YAML: {}", e)))
}

fn convert_yaml_value(value: YamlValue) -> Result<HashMap<String, RawValue>, SourceError> {
    match value {
        YamlValue::Mapping(map) => {
            let mut result = HashMap::new();
            for (key, value) in map {
                let key = key
                    .as_str()
                    .ok_or_else(|| SourceError::InvalidFormat("Keys must be strings".into()))?
                    .to_string();
                result.insert(key, yaml_to_raw_value(value)?);
            }
            Ok(result)
        }
        _ => Err(SourceError::InvalidFormat("Root must be a mapping".into())),
    }
}

fn yaml_to_raw_value(value: YamlValue) -> Result<RawValue, SourceError> {
    match value {
        YamlValue::Null => Ok(RawValue::Null),
        YamlValue::Bool(b) => Ok(RawValue::Boolean(b)),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(RawValue::Number(Number::Integer(i)))
            } else if let Some(f) = n.as_f64() {
                Ok(RawValue::Number(Number::Float(f)))
            } else {
                Err(SourceError::InvalidFormat("Invalid number".into()))
            }
        }
        YamlValue::String(s) => Ok(RawValue::String(s)),
        YamlValue::Sequence(seq) => {
            let values: Result<Vec<_>, _> = seq.into_iter().map(yaml_to_raw_value).collect();
            Ok(RawValue::Array(values?))
        }
        YamlValue::Mapping(map) => {
            let mut result = HashMap::new();
            for (key, value) in map {
                let key = key
                    .as_str()
                    .ok_or_else(|| SourceError::InvalidFormat("Keys must be strings".into()))?
                    .to_string();
                result.insert(key, yaml_to_raw_value(value)?);
            }
            Ok(RawValue::Object(result))
        }
        YamlValue::Tagged(_) => todo!(),
    }
}

fn convert_to_yaml_value(values: &HashMap<String, RawValue>) -> Result<YamlValue, SourceError> {
    let mut map = serde_yaml::Mapping::new();
    for (key, value) in values {
        map.insert(YamlValue::String(key.clone()), raw_value_to_yaml(value)?);
    }
    Ok(YamlValue::Mapping(map))
}

fn raw_value_to_yaml(value: &RawValue) -> Result<YamlValue, SourceError> {
    match value {
        RawValue::Null => Ok(YamlValue::Null),
        RawValue::Boolean(b) => Ok(YamlValue::Bool(*b)),
        RawValue::Number(Number::Integer(i)) => Ok(YamlValue::Number((*i).into())),
        RawValue::Number(Number::Float(f)) => {
            let n = serde_yaml::Number::from(*f);
            Ok(YamlValue::Number(n))
        }
        RawValue::String(s) => Ok(YamlValue::String(s.clone())),
        RawValue::Array(arr) => {
            let values: Result<Vec<_>, _> = arr.iter().map(raw_value_to_yaml).collect();
            Ok(YamlValue::Sequence(values?))
        }
        RawValue::Object(map) => {
            let mut result = serde_yaml::Mapping::new();
            for (key, value) in map {
                result.insert(YamlValue::String(key.clone()), raw_value_to_yaml(value)?);
            }
            Ok(YamlValue::Mapping(result))
        }
    }
}

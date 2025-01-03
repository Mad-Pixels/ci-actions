use super::{RawValue, TypeError};

pub trait FromValue: Sized {
    fn from_value(value: &RawValue) -> Result<Self, TypeError>;
}

impl FromValue for String {
    fn from_value(value: &RawValue) -> Result<Self, TypeError> {
        match value {
            RawValue::String(s) => Ok(s.clone()),
            _ => Err(TypeError::WrongType {
                expected: "String",
                actual: value.value_type().as_str(),
            }),
        }
    }
}

impl FromValue for bool {
    fn from_value(value: &RawValue) -> Result<Self, TypeError> {
        match value {
            RawValue::Boolean(b) => Ok(*b),
            RawValue::String(s) => s
                .parse()
                .map_err(|_| TypeError::ConversionError("Invalid boolean string".to_string())),
            _ => Err(TypeError::WrongType {
                expected: "Boolean",
                actual: value.value_type().as_str(),
            }),
        }
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: &RawValue) -> Result<Self, TypeError> {
        match value {
            RawValue::Array(arr) => arr.iter().map(|v| T::from_value(v)).collect(),
            _ => Err(TypeError::WrongType {
                expected: "Array",
                actual: value.value_type().as_str(),
            }),
        }
    }
}

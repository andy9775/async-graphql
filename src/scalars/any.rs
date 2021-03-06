use crate::{Result, ScalarType, Value};
use async_graphql_derive::Scalar;
use itertools::Itertools;
use serde::de::DeserializeOwned;

/// Any scalar
///
/// The `Any` scalar is used to pass representations of entities from external services into the root `_entities` field for execution.
#[derive(Clone, PartialEq, Debug)]
pub struct Any(pub Value);

#[Scalar(internal)]
impl ScalarType for Any {
    fn type_name() -> &'static str {
        "_Any"
    }

    fn description() -> Option<&'static str> {
        Some("The `_Any` scalar is used to pass representations of entities from external services into the root `_entities` field for execution.")
    }

    fn parse(value: &Value) -> Option<Self> {
        Some(Self(value.clone()))
    }

    fn is_valid(_value: &Value) -> bool {
        true
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(gql_value_to_json_value(self.0.clone()))
    }
}

impl Any {
    /// Parse this `Any` value to T by `serde_json`.
    pub fn parse_value<T: DeserializeOwned>(&self) -> std::result::Result<T, serde_json::Error> {
        serde_json::from_value(self.to_json().unwrap())
    }
}

pub(crate) fn gql_value_to_json_value(value: Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Variable(name) => name.into(),
        Value::Int(n) => n.as_i64().unwrap().into(),
        Value::Float(n) => n.into(),
        Value::String(s) => s.into(),
        Value::Boolean(v) => v.into(),
        Value::Enum(e) => e.into(),
        Value::List(values) => values
            .into_iter()
            .map(gql_value_to_json_value)
            .collect_vec()
            .into(),
        Value::Object(obj) => serde_json::Value::Object(
            obj.into_iter()
                .map(|(k, v)| (k, gql_value_to_json_value(v)))
                .collect(),
        ),
    }
}

impl<T> From<T> for Any
where
    T: Into<Value>,
{
    fn from(value: T) -> Any {
        Any(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conversion_ok() {
        let value = Value::List(vec![Value::Int(1.into()), Value::Float(2.0), Value::Null]);
        let expected = Any(value.clone());
        let output: Any = value.into();
        assert_eq!(output, expected);
    }
}

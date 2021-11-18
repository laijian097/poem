use std::borrow::Cow;

use serde_json::Value;

use crate::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type},
};

/// Represents a binary data encoded with base64.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Base64(pub Vec<u8>);

impl Type for Base64 {
    fn name() -> Cow<'static, str> {
        "string(bytes)".into()
    }

    impl_raw_value_type!();

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("bytes", "string")))
    }
}

impl ParseFromJSON for Base64 {
    fn parse_from_json(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(Self(base64::decode(value)?))
        } else {
            Err(ParseError::expected_type(value))
        }
    }
}

impl ParseFromParameter for Base64 {
    fn parse_from_parameter(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(Self(base64::decode(value)?)),
            None => Err(ParseError::expected_input()),
        }
    }
}

impl ToJSON for Base64 {
    fn to_json(&self) -> Value {
        Value::String(base64::encode(&self.0))
    }
}

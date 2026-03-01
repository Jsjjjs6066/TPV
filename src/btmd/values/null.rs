use serde_jsonc::Value;

use crate::btmd::values::{ValueType, ValueTypes};

#[derive(Clone, Default, Debug)]
pub struct NullType;

impl ValueType for NullType {
    fn parse(&self, _: &Value) -> ValueTypes {
        ValueTypes::Null(Self)
    }
}

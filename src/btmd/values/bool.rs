use serde_jsonc::Value;

use crate::btmd::values::{ValueType, ValueTypes};

#[derive(Clone, Debug, Default)]
pub struct BoolType {
    pub value: bool,
}

impl ValueType for BoolType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Bool(BoolType {
            value: match value {
                Value::Bool(b) => *b,
                _ => self.value,
            },
        })
    }
}

use serde_jsonc::{Value, to_string};

use crate::btmd::values::{ValueType, ValueTypes};

#[derive(Default, Clone, Debug)]
pub struct CharType(pub char);

impl ValueType for CharType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Char(match value {
            Value::String(s) => CharType(s.clone().chars().next().unwrap()),
            Value::Number(n) => CharType(to_string(n).unwrap_or_default().chars().next().unwrap()),
            _ => self.to_owned(),
        })
    }
}

use serde_jsonc::{Value, to_string};

use crate::btmd::{
    content::Text,
    values::{ValueType, ValueTypes},
};

#[derive(Default, Clone, Debug)]
pub struct TextType(pub Text);

impl ValueType for TextType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Text(match value {
            Value::Null | Value::Array(_) | Value::Object(_) => self.to_owned(),
            Value::String(s) => TextType(Text::new_default(s.clone())),
            Value::Number(n) => TextType(Text::new_default(to_string(n).unwrap_or_default())),
            Value::Bool(b) => TextType(Text::new_default(to_string(b).unwrap_or_default())),
        })
    }
}

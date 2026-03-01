use crossterm::style::Color;
use serde_jsonc::Value;

use crate::btmd::values::{ValueType, ValueTypes};

#[derive(Clone, Debug)]
pub struct ColorType {
    pub value: Color,
}

impl ValueType for ColorType {
    fn parse(&self, value: &Value) -> ValueTypes {
        match value {
            Value::String(s) => ValueTypes::Color(ColorType { value: Color::try_from(s.as_str()).unwrap_or(Color::Reset) }),
            _ => ValueTypes::Color(ColorType { value: self.value }),
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        Self { value: Color::Reset }
    }
}

impl From<ColorType> for String {
    fn from(value: ColorType) -> Self {
        match value.value {
            Color::Reset => "reset".to_string(),
            Color::Black => "black".to_string(),
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Magenta => "magenta".to_string(),
            Color::Cyan => "cyan".to_string(),
            Color::White => "white".to_string(),
            Color::Grey => "grey".to_string(),
            Color::DarkGrey => "dark_grey".to_string(),
            Color::DarkRed => "dark_red".to_string(),
            Color::DarkGreen => "dark_green".to_string(),
            Color::DarkYellow => "dark_yellow".to_string(),
            Color::DarkBlue => "dark_blue".to_string(),
            Color::DarkMagenta => "dark_magenta".to_string(),
            Color::DarkCyan => "dark_cyan".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
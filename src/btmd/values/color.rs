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
            Value::Number(n) => {
                if n.is_u64() {
                    ValueTypes::Color(ColorType { value: Color::AnsiValue(n.as_u64().unwrap() as u8) })
                } else if n.is_f64() {
                    ValueTypes::Color(ColorType { value: Color::AnsiValue(n.as_f64().unwrap() as u8) })
                } else {
                    ValueTypes::Color(ColorType { value: self.value })
                }
            },
            Value::Array(arr) => {
                if arr.len() >= 3 {
                    if arr[0].is_u64() && arr[1].is_u64() && arr[2].is_u64() {
                        ValueTypes::Color(ColorType { value: Color::Rgb { r: arr[0].as_u64().unwrap() as u8, g: arr[1].as_u64().unwrap() as u8, b: arr[2].as_u64().unwrap() as u8 } })
                    } else if arr[0].is_f64() && arr[1].is_f64() && arr[2].is_f64() {
                        ValueTypes::Color(ColorType { value: Color::Rgb { r: arr[0].as_f64().unwrap() as u8, g: arr[1].as_f64().unwrap() as u8, b: arr[2].as_f64().unwrap() as u8 } })
                    } else {
                        ValueTypes::Color(ColorType { value: self.value })
                    }
                } else {
                    ValueTypes::Color(ColorType { value: self.value })
                }
            },
            _ => ValueTypes::Color(ColorType { value: self.value }),
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        Self { value: Color::Reset }
    }
}

impl From<ColorType> for Value {
    fn from(value: ColorType) -> Self {
        match value.value {
            Color::Reset => Value::String("reset".to_string()),
            Color::Black => Value::String("black".to_string()),
            Color::Red => Value::String("red".to_string()),
            Color::Green => Value::String("green".to_string()),
            Color::Yellow => Value::String("yellow".to_string()),
            Color::Blue => Value::String("blue".to_string()),
            Color::Magenta => Value::String("magenta".to_string()),
            Color::Cyan => Value::String("cyan".to_string()),
            Color::White => Value::String("white".to_string()),
            Color::Grey => Value::String("grey".to_string()),
            Color::DarkGrey => Value::String("dark_grey".to_string()),
            Color::DarkRed => Value::String("dark_red".to_string()),
            Color::DarkGreen => Value::String("dark_green".to_string()),
            Color::DarkYellow => Value::String("dark_yellow".to_string()),
            Color::DarkBlue => Value::String("dark_blue".to_string()),
            Color::DarkMagenta => Value::String("dark_magenta".to_string()),
            Color::DarkCyan => Value::String("dark_cyan".to_string()),
            Color::Rgb { r, g, b } => Value::Array(vec![Value::Number(r.into()), Value::Number(g.into()), Value::Number(b.into())]),
            Color::AnsiValue(v) => Value::Number(v.into()),
        }
    }
}
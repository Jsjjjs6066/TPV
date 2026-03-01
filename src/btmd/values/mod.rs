pub mod array;
pub mod bool;
pub mod char;
pub mod color;
pub mod config;
pub mod element;
pub mod int;
pub mod null;
pub mod size;
pub mod text;

pub use array::ArrayType;
pub use bool::BoolType;
pub use char::CharType;
pub use color::ColorType;
pub use config::ConfigType;
pub use element::ElementType;
pub use int::IntType;
pub use null::NullType;
pub use size::SizeType;
pub use text::TextType;

use btmd_macro::ConfigLookup;
use enum_dispatch::enum_dispatch;
use serde_jsonc::Value;

#[enum_dispatch]
pub trait ValueType {
    fn parse(&self, value: &Value) -> ValueTypes;
}

#[enum_dispatch(ValueType)]
#[derive(ConfigLookup, Clone, Debug)]
pub enum ValueTypes {
    #[config_def("null")]
    Null(NullType),
    #[config_def("text")]
    Text(TextType),
    #[config_def("int")]
    Int(IntType),
    #[config_def("config")]
    Config(ConfigType),
    #[config_def("char")]
    Char(CharType),
    #[config_def("element")]
    Element(ElementType),
    #[config_def("array")]
    Array(ArrayType),
    #[config_def("size")]
    Size(SizeType),
    #[config_def("bool")]
    Bool(BoolType),
    #[config_def("color")]
    Color(ColorType),
}

impl Default for ValueTypes {
    fn default() -> Self {
        ValueTypes::Null(NullType)
    }
}
impl Default for &ValueTypes {
    fn default() -> Self {
        &ValueTypes::Null(NullType)
    }
}

pub fn enforce_type(value: ValueTypes, type_: &ValueTypes) -> Option<ValueTypes> {
    if std::mem::discriminant(&value) == std::mem::discriminant(type_) {
        Some(value)
    } else {
        None
    }
}

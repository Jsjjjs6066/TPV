use std::cmp::{max, min};

use serde_jsonc::Value;

use crate::btmd::values::{ValueType, ValueTypes};
use enum_dispatch::enum_dispatch;

#[derive(Clone, Default, Debug)]
pub struct IntType {
    pub int: Int,
    pub min: Int,
    pub max: Int,
}

impl ValueType for IntType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Int(IntType {
            int: max(
                min(
                    self.max.to_owned(),
                    Int::from_value(value.to_owned(), self.int.to_owned()),
                ),
                self.min.to_owned(),
            ),
            min: self.min.to_owned(),
            max: self.max.to_owned(),
        })
    }
}

#[derive(Clone)]
#[enum_dispatch(
    Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Rem, RemAssign, Neg
)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Int {
    Bit64(i64),
    Bit32(i32),
    Bit16(i16),
    Bit8(i8),
    Bit64U(u64),
    Bit32U(u32),
    Bit16U(u16),
    Bit8U(u8),
}

impl Default for Int {
    fn default() -> Self {
        Int::Bit64(0)
    }
}

impl Int {
    pub fn from_value(int: Value, size: Int) -> Self {
        match int {
            Value::Number(n) => match n {
                n if n.is_i64() => Self::from_int(n.as_i64().unwrap(), size),
                n if n.is_u64() => Self::from_int(n.as_u64().unwrap() as i64, size),
                _ => size,
            },
            _ => size,
        }
    }
    pub fn from_int(int: i64, size: Int) -> Self {
        match size {
            Int::Bit64(_) => Int::Bit64(int),
            Int::Bit32(_) => Int::Bit32(int as i32),
            Int::Bit16(_) => Int::Bit16(int as i16),
            Int::Bit8(_) => Int::Bit8(int as i8),
            Int::Bit64U(_) => Int::Bit64U(int as u64),
            Int::Bit32U(_) => Int::Bit32U(int as u32),
            Int::Bit16U(_) => Int::Bit16U(int as u16),
            Int::Bit8U(_) => Int::Bit8U(int as u8),
        }
    }
}

impl From<Int> for u8 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit8U(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for i8 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit8(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for i16 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit16(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for u16 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit16U(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for i32 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit32(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for u32 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit32U(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for i64 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit64(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

impl From<Int> for u64 {
    fn from(value: Int) -> Self {
        match value {
            Int::Bit64U(v) => v.to_owned(),
            _ => Default::default(),
        }
    }
}

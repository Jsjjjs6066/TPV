use std::cmp::{max, min};

use serde_jsonc::Value;

use crate::btmd::values::{ValueType, ValueTypes, int::Int};

#[derive(Clone, Default, Debug)]
pub struct SizeType {
    pub size: Int,
    pub min: Int,
    pub max: Int,
    pub auto: Int,
}

impl ValueType for SizeType {
    fn parse(&self, value: &Value) -> ValueTypes {
        if let Value::String(s) = value {
            match s.as_str() {
                "auto" => ValueTypes::Size(SizeType {
                    size: self.auto.to_owned(),
                    min: self.min.to_owned(),
                    max: self.max.to_owned(),
                    auto: self.auto.to_owned(),
                }),
                "max" => ValueTypes::Size(SizeType {
                    size: self.max.to_owned(),
                    min: self.min.to_owned(),
                    max: self.max.to_owned(),
                    auto: self.auto.to_owned(),
                }),
                "min" => ValueTypes::Size(SizeType {
                    size: self.min.to_owned(),
                    min: self.min.to_owned(),
                    max: self.max.to_owned(),
                    auto: self.auto.to_owned(),
                }),
                _ => ValueTypes::Size(SizeType {
                    size: max(
                        min(
                            self.max.to_owned(),
                            Int::from_value(value.to_owned(), self.size.to_owned()),
                        ),
                        self.min.to_owned(),
                    ),
                    min: self.min.to_owned(),
                    max: self.max.to_owned(),
                    auto: self.auto.to_owned(),
                }),
            }
        } else {
            ValueTypes::Size(SizeType {
                size: if let Value::Number(n) = value {
                    max(
                        min(
                            self.max.to_owned(),
                            Int::from_value(
                                serde_jsonc::Value::Number(n.to_owned()),
                                self.size.to_owned(),
                            ),
                        ),
                        self.min.to_owned(),
                    )
                } else {
                    self.size.to_owned()
                },
                min: self.min.to_owned(),
                max: self.max.to_owned(),
                auto: self.auto.to_owned(),
            })
        }
    }
}

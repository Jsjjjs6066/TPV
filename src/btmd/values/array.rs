use serde_jsonc::Value;

use crate::btmd::values::{enforce_type, ValueType, ValueTypes};

#[derive(Clone, Default, Debug)]
pub struct ArrayType {
    pub array: Vec<ValueTypes>,
    pub vec_type: Box<ValueTypes>,
}

impl ValueType for ArrayType {
    fn parse(&self, args: &Value) -> ValueTypes {
        ValueTypes::Array(ArrayType {
            array: match args {
                Value::Array(arr) => arr
                    .iter()
                    .zip(self.array.iter())
                    .map(|(v, other): (&Value, &ValueTypes)| {
                        enforce_type(other.parse(v), self.vec_type.as_ref())
                    })
                    .filter_map(|v| v)
                    .collect(),
                _ => self.array.to_owned(),
            },
            vec_type: self.vec_type.to_owned(),
        })
    }
}

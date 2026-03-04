use serde_jsonc::Value;

use crate::btmd;

use btmd::{
    args::ArgPreset,
    values::{ValueType, ValueTypes},
};

#[derive(Clone, Default)]
pub struct ArgParser {
    pub(crate) preset: ArgPreset,
}

impl ArgParser {
    pub fn new(config_preset: ArgPreset) -> ArgParser {
        ArgParser {
            preset: config_preset,
        }
    }

    pub fn parse(&self, vec_to_parse: &Vec<Value>) -> Vec<ValueTypes> {
        let mut vec_preset = self.preset.vec.clone();
        let mut vec_to_parse_iter = vec_to_parse.iter()
            .map(Some)
            .chain(std::iter::repeat_with(|| None));
        for (vec_preset_val, vec_to_parse_val) in vec_preset.iter_mut().zip(vec_to_parse_iter.by_ref()) {
            if let None = vec_to_parse_val {
                *vec_preset_val = vec_preset_val.parse(&Value::Null);
                continue;
            }
            let val1u = vec_preset_val.parse(vec_to_parse_val.unwrap());
            *vec_preset_val = val1u;
        }
        vec_preset
    }
}

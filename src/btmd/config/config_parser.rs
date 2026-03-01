use std::collections::HashMap;

use serde_jsonc::{Map, Value};

use crate::btmd;

use btmd::{
    config::ConfigPreset,
    values::{ValueType, ValueTypes},
};

#[derive(Clone, Default)]
pub struct ConfigParser {
    preset: ConfigPreset,
}

impl ConfigParser {
    pub fn new(config_preset: ConfigPreset) -> ConfigParser {
        ConfigParser {
            preset: config_preset,
        }
    }

    pub fn parse(&self, map: Map<String, Value>) -> HashMap<String, ValueTypes> {
        let mut hm = HashMap::new();
        for (k, v) in &self.preset.map {
            let input_val = map.get(k).unwrap_or(&Value::Null);
            hm.insert(k.clone(), v.parse(input_val));
        }
        for (k, v) in map {
            if !hm.contains_key(&k) {
                hm.insert(k.clone(), ValueTypes::default().parse(&v));
            }
        }
        hm
    }
}

pub mod config_parser;

use std::collections::HashMap;

use crate::btmd;

use btmd::values::ValueTypes;

#[derive(Clone, Default, Debug)]
pub struct ConfigPreset {
    map: HashMap<String, ValueTypes>,
}

impl ConfigPreset {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, key: String, value_type: ValueTypes) {
        self.map.insert(key, value_type);
    }

    pub fn get_type(&self, key: &str) -> &ValueTypes {
        self.map.get(key).unwrap_or(Default::default())
    }

    pub fn get_map(&self) -> &HashMap<String, ValueTypes> {
        &self.map
    }
}

#[macro_export]
macro_rules! config_preset {
    ($($key:expr => $value:expr),*) => {
        {
            use crate::btmd::config::ConfigPreset;
            #[allow(unused_mut)]
            // Must be mutable to add values,
            // but compiler states it does 
            // not need to be mutable
            let mut preset = ConfigPreset::new();
            $(preset.add_value($key.to_string(), $value);)*
            preset
        }
    };
}
use std::collections::HashMap;

use serde_jsonc::{Map, Value};

use crate::btmd::{config::{ConfigPreset, config_parser::ConfigParser}, values::{ValueType, ValueTypes}};

#[derive(Clone, Default, Debug)]
pub struct OnHoverType {
    pub map: Map<String, Value>
}

impl ValueType for OnHoverType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::OnHover(OnHoverType {
            map: match value {
                Value::Object(o) => o.clone(),
                _ => Default::default()
            }
        })
    }
}

impl OnHoverType {
    pub fn parse_inner(&self, preset: ConfigPreset) -> HashMap<String, ValueTypes> {
        let config_parser = ConfigParser::new(preset);
        config_parser.parse(self.map.clone())
    }
}
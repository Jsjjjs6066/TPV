use std::collections::HashMap;

use serde_jsonc::Value;

use crate::btmd::{
    config::{ConfigPreset, config_parser::ConfigParser},
    values::{ValueType, ValueTypes},
};

#[derive(Clone, Default, Debug)]
pub struct ConfigType(pub ConfigPreset, pub HashMap<String, ValueTypes>);

impl ValueType for ConfigType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Config(Self(
            self.0.clone(),
            match value {
                Value::Object(o) => {
                    let config_parser = ConfigParser::new(self.0.to_owned());
                    config_parser.parse(o.clone())
                }
                _ => {
                    let config_parser = ConfigParser::new(self.0.to_owned());
                    config_parser.parse(Default::default())
                },
            },
        ))
    }
}

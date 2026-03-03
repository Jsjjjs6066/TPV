use std::sync::{Arc, RwLock};

use serde_jsonc::Value;

use crate::btmd::{
    element::{Element, NONE, registry::ElementRegistry}, values::{ValueType, ValueTypes}
};

#[derive(Clone, Debug)]
pub struct ElementType {
    pub element: Arc<RwLock<Element>>,
    pub registry: ElementRegistry,
}

impl ValueType for ElementType {
    fn parse(&self, args: &Value) -> ValueTypes {
        ValueTypes::Element(ElementType {
            element: match args {
                Value::Array(arr) => {
                    if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                        let args: Vec<Value> = arr[1..].to_vec();
                        Arc::new(RwLock::new(
                            self.registry.get_element(element_type).new_from(args),
                        ))
                    }
                    else {
                        self.element.to_owned()
                    }
                },
                _ => self.element.to_owned(),
            },
            registry: self.registry.to_owned(),
        })
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self {
            element: Arc::new(RwLock::new(NONE.clone())),
            registry: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! element_array {
    () => {
        ValueTypes::Array(ArrayType {
            array: vec![],
            vec_type: Box::new(ValueTypes::Element(Default::default())),
        })
    };
}
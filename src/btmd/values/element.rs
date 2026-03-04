use std::sync::{Arc};

use serde_jsonc::Value;

use crate::btmd::{
    element::{Element, NONE, RawElement, registry::ElementRegistry}, values::{ValueType, ValueTypes}
};

#[derive(Clone, Debug)]
pub struct ElementType {
    pub element: Arc<parking_lot::RwLock<Element>>,
    pub parent: Arc<parking_lot::RwLock<RawElement>>,
    pub registry: ElementRegistry,
}

impl ValueType for ElementType {
    fn parse(&self, args: &Value) -> ValueTypes {
        ValueTypes::Element(ElementType {
            element: match args {
                Value::Array(arr) => {
                    if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                        let args: Vec<Value> = arr[1..].to_vec();
                        Arc::new(parking_lot::RwLock::new(
                            self.registry.get_element(element_type).new_from(args, Some(self.parent.clone())),
                        ))
                    }
                    else {
                        self.element.to_owned()
                    }
                },
                _ => self.element.to_owned(),
            },
            parent: self.parent.clone(),
            registry: self.registry.to_owned(),
        })
    }
}

impl Default for ElementType {
    fn default() -> Self {
        Self {
            element: Arc::new(parking_lot::RwLock::new(NONE.clone())),
            parent: NONE.raw_element.clone(),
            registry: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! element_array {
    (parent: $parent:expr) => {
        ValueTypes::Array(ArrayType {
            array: vec![],
            vec_type: Box::new(ValueTypes::Element(crate::btmd::values::ElementType {
                element: Arc::new(parking_lot::RwLock::new(crate::btmd::element::NONE.clone())),
                parent: $parent,
                registry: Default::default(),
            })),
        })
    };
}
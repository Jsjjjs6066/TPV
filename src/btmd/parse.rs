use crate::btmd::{self, element::RawElement};

use btmd::{
    element::registry::ElementRegistry,
    import_default_elements,
    page::Page,
};
use serde_jsonc::Value;
use std::sync::{Arc};

pub fn parse_json_to_page(json_page: Value) -> Page {
    let title: String = json_page["title"].as_str().unwrap_or("Page").to_string();
    let body_unparsed: Vec<Value> = json_page["body"].as_array().unwrap_or(&Vec::new()).to_vec();

    let mut body: Vec<Arc<parking_lot::RwLock<RawElement>>> = Vec::with_capacity(body_unparsed.len());

    let mut registry: ElementRegistry = ElementRegistry::new();
    import_default_elements(&mut registry);

    for element in body_unparsed {
        if let Some(arr) = element.as_array() {
            if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                let args: Vec<Value> = arr[1..].to_vec();
                let element_instance: Arc<parking_lot::RwLock<RawElement>> = registry.get_element(element_type).new_from(args, None).raw_element;
                body.push(element_instance);
            }
        }
    }

    Page::new(title, body, json_page["body"].clone(), registry)
}
pub fn parse_str_to_page(input: &str) -> Page {
    let json_page: Value = serde_jsonc::from_str(input).unwrap();
    parse_json_to_page(json_page)
}

pub fn parse_vec_to_vec(
    input: Vec<Value>,
    registry: &ElementRegistry,
    parent: Arc<parking_lot::RwLock<RawElement>>,
) -> Vec<Arc<parking_lot::RwLock<RawElement>>> {
    let mut body: Vec<Arc<parking_lot::RwLock<RawElement>>> = Vec::with_capacity(input.len());

    for element in input {
        if let Some(arr) = element.as_array() {
            if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                let args: Vec<Value> = arr[1..].to_vec();
                let element_instance: Arc<parking_lot::RwLock<RawElement>> = registry.get_element(element_type).new_from(args, Some(parent.clone())).raw_element;
                body.push(element_instance);
            }
        }
    }

    body
}
pub fn parse_str_to_vec(input: &str, registry: &ElementRegistry, parent: Arc<parking_lot::RwLock<RawElement>>) -> Vec<Arc<parking_lot::RwLock<RawElement>>> {
    let elements: Vec<Value> = serde_jsonc::from_str(input)
        .unwrap_or(Value::Array(vec![]))
        .as_array()
        .unwrap_or(&Vec::new())
        .to_vec();
    parse_vec_to_vec(elements, registry, parent)
}

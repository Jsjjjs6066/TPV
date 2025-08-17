use serde_json::Value;
use crate::tprl::{element::{registry, Element}, page::Page};

pub fn parse_json_to_page(json_page: Value) -> Page {
	let title: String = json_page["title"].as_str().unwrap_or("Page").to_string();
	let body_unparsed: Vec<Value> = json_page["body"].as_array().unwrap_or(&Vec::new()).to_vec();

	let mut body: Vec<Element> = Vec::with_capacity(body_unparsed.len());

	for element in body_unparsed {
		if let Some(arr) = element.as_array() {
			if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
				let args: Vec<Value> = arr[1..].to_vec();
				let element_instance: Element = Element::new(registry::get_element(element_type).render_func, args);
				body.push(element_instance);
			}
		}
	}

	Page::new(title, body)
}
pub fn parse_str_to_page(input: &str) -> Page {
	let json_page: Value = serde_json::from_str(input).unwrap();
	parse_json_to_page(json_page)
}

pub fn parse_vec_to_vec(input: Vec<Value>) -> Vec<Element> {
	let mut body: Vec<Element> = Vec::with_capacity(input.len());

	for element in input {
		if let Some(arr) = element.as_array() {
			if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
				let args: Vec<Value> = arr[1..].to_vec();
				let element_instance: Element = Element::new(registry::get_element(element_type).render_func, args);
				body.push(element_instance);
			}
		}
	}

	body
}
pub fn parse_str_to_vec(input: &str) -> Vec<Element> {
	let elements: Vec<Value> = serde_json::from_str(input).unwrap_or(Value::Array(vec![])).as_array().unwrap_or(&Vec::new()).to_vec();
	parse_vec_to_vec(elements)
}
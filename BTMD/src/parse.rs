use std::collections::HashMap;

use serde_json::Value;
use crate::{element::{Element}, page::Page};

pub fn parse_json_to_page(json_page: Value, registry: HashMap<String, Box<dyn Element>>) -> Page {
	let title: String = json_page["title"].as_str().unwrap_or("Page").to_string();
	let body_unparsed: Vec<Value> = json_page["body"].as_array().unwrap_or(&Vec::new()).to_vec();

	let mut body: Vec<Box<dyn Element>> = Vec::with_capacity(body_unparsed.len());

	for element in body_unparsed {
		if let Some(arr) = element.as_array() {
			if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
				let args: Vec<Value> = arr[1..].to_vec();
				let mut element_instance= registry.get(element_type);
				println!("1");
				let mut element_instance1= element_instance.unwrap();
				println!("2");
				let mut element_instance2= element_instance1.clone_this();
				println!("3");
				let mut element_instance3= element_instance2.new_from(args);
				println!("4");
				let mut element_instance4= Box::new(element_instance3);
				println!("5");
				body.push(element_instance4);
			}
		}
	}

	Page::new(title, body, json_page["body"].clone())
}
pub fn parse_str_to_page(input: &str, registry: HashMap<String, Box<dyn Element>>) -> Page {
	let json_page: Value = serde_json::from_str(input).unwrap();
	parse_json_to_page(json_page, registry)
}

pub fn parse_vec_to_vec<>(input: Vec<Value>, registry: HashMap<String, Box<dyn Element>>) -> Vec<Box<dyn Element>> {
	let mut body: Vec<Box<dyn Element>> = Vec::with_capacity(input.len());

	for element in input {
		if let Some(arr) = element.as_array() {
			if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
				let args: Vec<Value> = arr[1..].to_vec();
				let element_instance: Box<dyn Element> = Box::new(registry.get(element_type).unwrap().clone().new_from(args));
				body.push(element_instance);
			}
		}
	}

	body
}
pub fn parse_str_to_vec(input: &str, registry: HashMap<String, Box<dyn Element>>) -> Vec<Box<dyn Element>> {
	let elements: Vec<Value> = serde_json::from_str(input).unwrap_or(Value::Array(vec![])).as_array().unwrap_or(&Vec::new()).to_vec();
	parse_vec_to_vec(elements, registry)
}
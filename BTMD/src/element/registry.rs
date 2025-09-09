use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::element::Element;

static ELEMENT_REGISTRY: LazyLock<Mutex<HashMap<String, Element>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
	

pub fn register_element(name: String, element: &Element) {
	let mut registry = ELEMENT_REGISTRY.lock().unwrap();
	registry.insert(name, element.to_owned().clone());
}

pub fn get_element(name: &str) -> Element {
	let registry = ELEMENT_REGISTRY.lock().unwrap();
	registry.get(name)
		.or_else(|| registry.get("none"))
		.cloned().unwrap()
}

pub fn add_alias(alias: String, target: &str) {
	register_element(alias, &get_element(target));
}
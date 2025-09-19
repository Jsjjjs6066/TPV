use std::collections::HashMap;

use serde_json::Value;

use crate::{default_elements, element::Element};

#[derive(Clone)]
pub struct Page {
	pub title: String,
	pub body: Vec<Box<dyn Element>>,
	pub body_raw: Value,
	pub cursor: (u16, u16),
	pub element_registry: HashMap<String, Box<dyn Element>>,
}

impl Page {
	pub fn new(title: String, body: Vec<Box<dyn Element>>, body_raw: Value) -> Self {
		Page {title, body, body_raw, cursor: (0, 0), element_registry: default_elements()}
	}
}